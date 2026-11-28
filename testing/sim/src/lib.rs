//! DST harness (README §10). Drives the API frontend (`kubemu-api`) with
//! random clients under a seeded [`SimEnv`], checks invariants
//! continuously, and records a trace whose hash must be identical for
//! identical seeds.
//!
//! Every client goes through the real request path (ADR 0011):
//! - writers create, update (sometimes with a stale resourceVersion),
//!   patch (merge, strategic, JSON), delete, and hold or release objects
//!   with finalizers;
//! - informers list (paginated) and watch with bookmarks, like client-go's
//!   reflector, under injected watch drops and 410s, relisting when told
//!   their history is gone;
//! - paginated listers read one page per step while writes go on between
//!   pages;
//! - clusters are forked mid-run, and informers follow onto the fork;
//! - owners and dependents are created and deleted with every propagation
//!   policy; pods and Services come and go (Services with allocated,
//!   requested or no cluster IPs, in CIDRs small enough to run out); the
//!   `team` namespace is deleted and recreated;
//! - the built-in controllers (`kubemu-controllers`, `kubemu-scheduler`)
//!   are actors too, stepped one at a time between client requests.
//!
//! Invariants: no unexpected response codes, watch resourceVersions
//! strictly increase, no lost events (every informer mirrors the cluster
//! once caught up), a paginated list equals the list at its
//! resourceVersion, and no two live objects share an IP (Service cluster
//! IPs and pod IPs, each inside its CIDR). Once clients stop and controllers settle (virtual time
//! advancing to their requeues): nothing is left terminating, no
//! dependent outlives all its owners (GC eventually consistent), every
//! namespace has its default ServiceAccount, every pod is bound and
//! Running, and Endpoints list exactly the ready pods.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::time::Duration;

use kubemu_api::object;
use kubemu_api::{Api, Cidr, ClusterNetwork, HttpRequest, Reply, WatchCursor};
use kubemu_controllers::{Manager, stored};
use kubemu_env::Rng;
use kubemu_env_sim::SimEnv;
use kubemu_faults::{Fault, FaultConfig, FaultSchedule};
use serde_json::{Value, json};

pub const CLUSTERS: [&str; 3] = ["alpha", "beta", "gamma"];
const NAMESPACES: [&str; 2] = ["default", "team"];
const FINALIZER: &str = "sim.kubemu.dev/hold";

/// A create may meet an existing name (409) or a namespace that is
/// terminating (403) or gone (404).
const CREATE_OK: &[u16] = &[201, 403, 404, 409];

/// Service names; more than a cluster's Service CIDR holds.
const SERVICES: [&str; 6] = ["web", "db", "api", "cache", "queue", "search"];

/// Cluster `i`'s CIDRs: room for every pod the sim makes (4 names in 2
/// namespaces), and for fewer Services than it makes, so allocation runs
/// out and freed IPs are reused.
fn network(i: u8) -> ClusterNetwork {
    let c = |s: String| s.parse::<Cidr>().expect("valid CIDR");
    ClusterNetwork::new(
        c(format!("10.{}.0.0/28", 64 + i)),
        c(format!("10.{}.0.0/29", 128 + i)),
    )
    .expect("disjoint")
}

/// The controllers of `kubemu up`, for one cluster. A fork is a copy of
/// its root cluster, nodes included, so its kubelet is the root's node.
fn controllers(sim: &SimEnv, api: &Api, cluster: &str) -> Manager {
    let root = cluster.split('.').next().unwrap_or(cluster);
    let pod_cidr = api.network(cluster).expect("cluster exists").pod_cidr;
    let mut cs = kubemu_controllers::standard(&sim.env, root, pod_cidr);
    cs.push(Box::new(kubemu_scheduler::Scheduler::new(sim.env.clone())));
    Manager::new(cs)
}

#[derive(Clone, Debug)]
pub struct Config {
    pub steps: u32,
    pub log_capacity: usize,
    pub keys_per_namespace: u64,
    /// Forks stop once this many clusters exist.
    pub max_clusters: usize,
    pub faults: FaultConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            steps: 500,
            log_capacity: 32,
            keys_per_namespace: 8,
            max_clusters: 6,
            faults: FaultConfig::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Outcome {
    pub seed: u64,
    pub trace_hash: u64,
    pub trace_lines: usize,
    pub faults: usize,
    pub forks: u32,
    pub relists: u32,
    pub violations: Vec<String>,
}

impl Outcome {
    pub fn ok(&self) -> bool {
        self.violations.is_empty()
    }
}

type Mirror = BTreeMap<(String, String), (String, Value)>;

/// What an informer's cache holds for one object: its resourceVersion
/// and data, keyed by namespace and name.
fn entry(obj: &Value) -> ((String, String), (String, Value)) {
    let m = &obj["metadata"];
    (
        (
            m["namespace"].as_str().unwrap_or("").to_owned(),
            m["name"].as_str().unwrap_or("").to_owned(),
        ),
        (
            m["resourceVersion"].as_str().unwrap_or("").to_owned(),
            obj.get("data").cloned().unwrap_or(Value::Null),
        ),
    )
}

fn rv_of(obj: &Value) -> u64 {
    obj["metadata"]["resourceVersion"]
        .as_str()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}

const CMS: &str = "/api/v1/configmaps";

fn cms(ns: &str) -> String {
    format!("/api/v1/namespaces/{ns}/configmaps")
}

struct Sim<'a> {
    api: Api,
    rng: &'a dyn Rng,
    trace: String,
    violations: Vec<String>,
    step: u32,
}

impl Sim<'_> {
    fn call(&mut self, cluster: &str, req: HttpRequest, ok: &[u16]) -> (u16, Value) {
        let resp = self.api.handle(cluster, &req).response();
        let v = resp.value().unwrap_or(Value::Null);
        if !ok.contains(&resp.status) {
            self.violations.push(format!(
                "step {}: {cluster} {} {}?{} -> {} {}",
                self.step, req.method, req.path, req.query, resp.status, v["message"]
            ));
        }
        (resp.status, v)
    }

    fn pick<'b>(&self, xs: &[&'b str]) -> &'b str {
        xs[self.rng.below(xs.len() as u64) as usize]
    }
}

/// A client-go-style informer: list, then watch from the list's
/// resourceVersion; relist on 410.
#[derive(Debug, Default)]
struct Informer {
    rv: u64,
    cursor: Option<WatchCursor>,
    mirror: Mirror,
    relists: u32,
}

impl Informer {
    fn relist(&mut self, sim: &mut Sim<'_>, cluster: &str) {
        self.relists += 1;
        self.cursor = None;
        self.mirror.clear();
        let limit = 1 + sim.rng.below(6);
        let mut cont = String::new();
        loop {
            let q = if cont.is_empty() {
                format!("{CMS}?limit={limit}")
            } else {
                format!("{CMS}?limit={limit}&continue={cont}")
            };
            let (s, l) = sim.call(cluster, HttpRequest::new("GET", &q), &[200, 410]);
            if s == 410 {
                // The list fell out of history between pages: start over.
                self.mirror.clear();
                cont.clear();
                continue;
            }
            for item in l["items"].as_array().into_iter().flatten() {
                let (k, v) = entry(item);
                self.mirror.insert(k, v);
            }
            self.rv = l["metadata"]["resourceVersion"]
                .as_str()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
            match l["metadata"]["continue"].as_str() {
                Some(c) => cont = c.to_owned(),
                None => break,
            }
        }
    }

    fn sync(&mut self, sim: &mut Sim<'_>, cluster: &str, faults: Option<&mut FaultSchedule>) {
        if self.cursor.is_none() {
            let q = format!(
                "{CMS}?watch=1&allowWatchBookmarks=true&resourceVersion={}",
                self.rv
            );
            match sim.api.handle(cluster, &HttpRequest::new("GET", &q)) {
                Reply::Watch(c) => self.cursor = Some(c),
                Reply::Response(r) => {
                    sim.violations
                        .push(format!("{cluster}: watch refused: {}", r.status));
                    return;
                }
            }
        }
        let mut cursor = self.cursor.take().expect("set above");
        let batch = sim.api.watch_poll(&mut cursor);
        let deliver = match faults.and_then(|f| f.on_watch(cluster, batch.events.len())) {
            Some(Fault::WatchGone) => return self.relist(sim, cluster),
            // The connection breaks: keep what arrived, rewatch later.
            Some(Fault::WatchDrop { deliver }) => Some(deliver),
            None => None,
        };
        let n = deliver.unwrap_or(batch.events.len());
        for e in batch.events.iter().take(n) {
            let obj = &e["object"];
            let t = e["type"].as_str().unwrap_or("");
            if t == "ERROR" {
                if obj["code"] != 410 {
                    sim.violations.push(format!("{cluster}: watch error {obj}"));
                }
                return self.relist(sim, cluster);
            }
            let rv = rv_of(obj);
            let monotonic = if t == "BOOKMARK" {
                rv >= self.rv
            } else {
                rv > self.rv
            };
            if !monotonic {
                sim.violations.push(format!(
                    "{cluster}: watch went from rv {} to {rv} ({t})",
                    self.rv
                ));
            }
            self.rv = rv;
            let (k, v) = entry(obj);
            match t {
                "ADDED" | "MODIFIED" => {
                    self.mirror.insert(k, v);
                }
                "DELETED" => {
                    self.mirror.remove(&k);
                }
                _ => {}
            }
        }
        if deliver.is_none() && !batch.closed {
            self.cursor = Some(cursor);
        }
    }
}

/// A list read one page per step, checked against the list at its
/// resourceVersion once complete.
#[derive(Debug)]
struct Pager {
    ns: &'static str,
    limit: u64,
    cont: String,
    rv: String,
    items: Mirror,
}

pub fn run(seed: u64, cfg: &Config) -> Outcome {
    let simenv = SimEnv::new(seed);
    let rng = simenv.env.rng.clone();
    let mut faults = FaultSchedule::new(simenv.env.clone(), cfg.faults.clone());
    let mut sim = Sim {
        api: Api::with_log_capacity(simenv.env.clone(), cfg.log_capacity),
        rng: rng.as_ref(),
        trace: String::new(),
        violations: Vec::new(),
        step: 0,
    };
    let mut informers: BTreeMap<String, Informer> = BTreeMap::new();
    let mut pagers: BTreeMap<String, Pager> = BTreeMap::new();
    let mut managers: BTreeMap<String, Manager> = BTreeMap::new();
    for (i, c) in CLUSTERS.into_iter().enumerate() {
        sim.api.add_cluster_with(c, network(i as u8));
        managers.insert(c.to_owned(), controllers(&simenv, &sim.api, c));
        sim.call(
            c,
            HttpRequest::new("POST", "/api/v1/namespaces")
                .json(&json!({"metadata": {"name": "team"}})),
            &[201],
        );
        let mut i = Informer::default();
        i.relist(&mut sim, c);
        informers.insert(c.to_owned(), i);
    }
    let mut forks = 0u32;

    for step in 0..cfg.steps {
        sim.step = step;
        simenv.clock.advance(Duration::from_millis(rng.below(2000)));
        let names: Vec<String> = informers.keys().cloned().collect();
        let cluster = names[rng.below(names.len() as u64) as usize].clone();
        let c = cluster.as_str();
        let ns = sim.pick(&NAMESPACES);
        let name = format!("cm-{}", rng.below(cfg.keys_per_namespace));
        let path = format!("{}/{name}", cms(ns));
        let faults_before = faults.injected().len();

        let what = match rng.below(34) {
            0..=4 => {
                let mut body =
                    json!({"metadata": {"name": name}, "data": {"v": rng.next_u64().to_string()}});
                if rng.chance(0.3) {
                    body["metadata"]["finalizers"] = json!([FINALIZER]);
                }
                let (s, _) = sim.call(c, HttpRequest::new("POST", &cms(ns)).json(&body), CREATE_OK);
                format!("create {ns}/{name} -> {s}")
            }
            5..=7 => {
                let (s, mut obj) = sim.call(c, HttpRequest::new("GET", &path), &[200, 404]);
                if s == 200 {
                    obj["data"] = json!({"v": rng.next_u64().to_string()});
                    if rng.chance(0.2) {
                        // A stale write: conflicts unless nothing changed.
                        obj["metadata"]["resourceVersion"] = json!((rv_of(&obj) - 1).to_string());
                    }
                    let (s, _) =
                        sim.call(c, HttpRequest::new("PUT", &path).json(&obj), &[200, 409]);
                    format!("update {ns}/{name} -> {s}")
                } else {
                    format!("update {ns}/{name} -> missing")
                }
            }
            8..=10 => {
                let v = rng.next_u64().to_string();
                let (ct, body) = match rng.below(3) {
                    0 => ("application/merge-patch+json", json!({"data": {"v": v}})),
                    1 => (
                        "application/strategic-merge-patch+json",
                        json!({"data": {"v": v}, "metadata": {"labels": {"p": "1"}}}),
                    ),
                    _ => (
                        "application/json-patch+json",
                        json!([{"op": "add", "path": "/data", "value": {"v": v}}]),
                    ),
                };
                let (s, _) = sim.call(
                    c,
                    HttpRequest::new("PATCH", &path)
                        .json(&body)
                        .with_content_type(ct),
                    &[200, 404],
                );
                format!("patch {ns}/{name} {ct} -> {s}")
            }
            11..=13 => {
                let (s, _) = sim.call(c, HttpRequest::new("DELETE", &path), &[200, 404]);
                format!("delete {ns}/{name} -> {s}")
            }
            14 => {
                // Release the finalizer of a deleting object.
                let (s, mut obj) = sim.call(c, HttpRequest::new("GET", &path), &[200, 404]);
                if s == 200 && obj["metadata"]["deletionTimestamp"].is_string() {
                    obj["metadata"]["finalizers"] = json!([]);
                    let (s, _) =
                        sim.call(c, HttpRequest::new("PUT", &path).json(&obj), &[200, 409]);
                    format!("release {ns}/{name} -> {s}")
                } else {
                    format!("release {ns}/{name} -> nothing")
                }
            }
            15 if names.len() < cfg.max_clusters => {
                forks += 1;
                let to = format!("{c}.{forks}");
                sim.api.fork_cluster(c, &to).expect("fork target is new");
                // The fork's informer resumes from the parent's state.
                let parent = &informers[c];
                let i = Informer {
                    rv: parent.rv,
                    cursor: None,
                    mirror: parent.mirror.clone(),
                    relists: 0,
                };
                informers.insert(to.clone(), i);
                managers.insert(to.clone(), controllers(&simenv, &sim.api, &to));
                format!("fork -> {to}")
            }
            16..=17 => match pagers.remove(c) {
                None => {
                    let p = Pager {
                        ns,
                        limit: 1 + rng.below(3),
                        cont: String::new(),
                        rv: String::new(),
                        items: Mirror::new(),
                    };
                    pagers.insert(c.to_owned(), p);
                    "pager start".into()
                }
                Some(mut p) => {
                    let q = if p.cont.is_empty() {
                        format!("{}?limit={}", cms(p.ns), p.limit)
                    } else {
                        format!("{}?limit={}&continue={}", cms(p.ns), p.limit, p.cont)
                    };
                    let (s, l) = sim.call(c, HttpRequest::new("GET", &q), &[200, 410]);
                    if s == 200 {
                        if !p.rv.is_empty() && l["metadata"]["resourceVersion"] != p.rv.as_str() {
                            sim.violations
                                .push(format!("{c}: page changed resourceVersion"));
                        }
                        p.rv = l["metadata"]["resourceVersion"]
                            .as_str()
                            .unwrap_or("")
                            .to_owned();
                        for item in l["items"].as_array().into_iter().flatten() {
                            let (k, v) = entry(item);
                            p.items.insert(k, v);
                        }
                        match l["metadata"]["continue"].as_str() {
                            Some(cont) => {
                                p.cont = cont.to_owned();
                                pagers.insert(c.to_owned(), p);
                            }
                            None => {
                                // The whole list must equal the list at its rv.
                                let q = format!(
                                    "{}?resourceVersion={}&resourceVersionMatch=Exact",
                                    cms(p.ns),
                                    p.rv
                                );
                                let (s, l) = sim.call(c, HttpRequest::new("GET", &q), &[200, 410]);
                                if s == 200 {
                                    let exact: Mirror = l["items"]
                                        .as_array()
                                        .into_iter()
                                        .flatten()
                                        .map(entry)
                                        .collect();
                                    if exact != p.items {
                                        sim.violations.push(format!(
                                            "{c}: paginated list differs from the list at rv {}",
                                            p.rv
                                        ));
                                    }
                                }
                            }
                        }
                    }
                    format!("pager page -> {s}")
                }
            },
            24..=25 => {
                // A dependent of an existing configmap.
                let (s, owner) = sim.call(c, HttpRequest::new("GET", &path), &[200, 404]);
                if s == 200 {
                    let dep = format!("dep-{}", rng.below(cfg.keys_per_namespace));
                    let body = json!({"metadata": {"name": dep, "ownerReferences": [{
                        "apiVersion": "v1", "kind": "ConfigMap",
                        "name": object::name(&owner), "uid": object::uid(&owner),
                    }]}});
                    let (s, _) =
                        sim.call(c, HttpRequest::new("POST", &cms(ns)).json(&body), CREATE_OK);
                    format!("dependent {ns}/{dep} of {name} -> {s}")
                } else {
                    format!("dependent of {ns}/{name} -> no owner")
                }
            }
            26 => {
                let policy = sim.pick(&["Background", "Foreground", "Orphan"]);
                let req =
                    HttpRequest::new("DELETE", &path).json(&json!({"propagationPolicy": policy}));
                let (s, _) = sim.call(c, req, &[200, 404]);
                format!("delete {ns}/{name} {policy} -> {s}")
            }
            27 => {
                let pod = format!("pod-{}", rng.below(4));
                let p = format!("/api/v1/namespaces/{ns}/pods");
                if rng.chance(0.6) {
                    let app = sim.pick(&["web", "db"]);
                    let body = json!({"metadata": {"name": pod, "labels": {"app": app}}, "spec": {"containers": [
                        {"name": "c", "image": "app:1", "ports": [{"name": "http", "containerPort": 8080}]}
                    ]}});
                    let (s, _) = sim.call(c, HttpRequest::new("POST", &p).json(&body), CREATE_OK);
                    format!("create pod {ns}/{pod} -> {s}")
                } else {
                    let (s, _) = sim.call(
                        c,
                        HttpRequest::new("DELETE", &format!("{p}/{pod}")),
                        &[200, 404],
                    );
                    format!("delete pod {ns}/{pod} -> {s}")
                }
            }
            18..=20 | 28 => {
                let svc = sim.pick(&SERVICES);
                let p = format!("/api/v1/namespaces/{ns}/services");
                let one = format!("{p}/{svc}");
                let cidr = sim.api.network(c).expect("cluster exists").service_cidr;
                match rng.below(6) {
                    0..=3 => {
                        let app = sim.pick(&["web", "db"]);
                        let mut spec = json!({"selector": {"app": app},
                            "ports": [{"port": 80, "targetPort": "http", "protocol": "TCP"}]});
                        // Requested IPs may be taken (422); allocation may
                        // find the CIDR full (500).
                        let ok: &[u16] = match rng.below(4) {
                            0 => {
                                let host = rng.below(6) as u32;
                                let ip =
                                    std::net::Ipv4Addr::from(u32::from(cidr.first_host()) + host);
                                spec["clusterIP"] = json!(ip.to_string());
                                &[201, 403, 404, 409, 422]
                            }
                            1 => {
                                spec["clusterIP"] = json!("None");
                                CREATE_OK
                            }
                            _ => &[201, 403, 404, 409, 500],
                        };
                        let body = json!({"metadata": {"name": svc}, "spec": spec});
                        let (s, _) = sim.call(c, HttpRequest::new("POST", &p).json(&body), ok);
                        format!("create service {ns}/{svc} -> {s}")
                    }
                    4 => {
                        // An update that leaves the cluster IPs out keeps them.
                        let (s, mut obj) = sim.call(c, HttpRequest::new("GET", &one), &[200, 404]);
                        if s == 200 {
                            let before = obj["spec"]["clusterIPs"].clone();
                            if let Some(spec) = obj["spec"].as_object_mut() {
                                spec.remove("clusterIP");
                                spec.remove("clusterIPs");
                            }
                            obj["metadata"]["labels"] = json!({"v": rng.below(9).to_string()});
                            let (s, after) =
                                sim.call(c, HttpRequest::new("PUT", &one).json(&obj), &[200, 409]);
                            if s == 200 && after["spec"]["clusterIPs"] != before {
                                sim.violations.push(format!(
                                    "step {step}: {c} update of {ns}/{svc} moved its IPs from {before} to {}",
                                    after["spec"]["clusterIPs"]
                                ));
                            }
                            format!("update service {ns}/{svc} -> {s}")
                        } else {
                            format!("update service {ns}/{svc} -> missing")
                        }
                    }
                    _ => {
                        let (s, _) = sim.call(c, HttpRequest::new("DELETE", &one), &[200, 404]);
                        format!("delete service {ns}/{svc} -> {s}")
                    }
                }
            }
            29 => {
                if rng.chance(0.3) {
                    let (s, _) = sim.call(
                        c,
                        HttpRequest::new("DELETE", "/api/v1/namespaces/team"),
                        &[200, 404],
                    );
                    format!("delete namespace team -> {s}")
                } else {
                    let body = json!({"metadata": {"name": "team"}});
                    let (s, _) = sim.call(
                        c,
                        HttpRequest::new("POST", "/api/v1/namespaces").json(&body),
                        &[201, 409],
                    );
                    format!("create namespace team -> {s}")
                }
            }
            30..=33 => {
                let m = managers.get_mut(c).expect("manager exists");
                let i = rng.below(m.len() as u64) as usize;
                let r = m.step_one(i, &mut sim.api, c);
                format!("controller {} -> {} requests", m.name(i), r.requests)
            }
            _ => {
                let i = informers.get_mut(c).expect("informer exists");
                let mut i = std::mem::take(i);
                i.sync(&mut sim, c, Some(&mut faults));
                let out = format!("sync rv={} relists={}", i.rv, i.relists);
                informers.insert(c.to_owned(), i);
                out
            }
        };
        check_ips(&mut sim, c);
        for f in &faults.injected()[faults_before..] {
            let _ = writeln!(sim.trace, "{step} {f}");
        }
        for line in sim.api.take_trace() {
            let _ = writeln!(sim.trace, "{step} {line}");
        }
        let _ = writeln!(sim.trace, "{step} {c} {what}");
    }

    // Clients stop: release our finalizers, let controllers settle, with
    // virtual time jumping to their requeues, then check the end state.
    let names: Vec<String> = informers.keys().cloned().collect();
    for c in &names {
        release_holds(&mut sim, c);
        let m = managers.get_mut(c).expect("manager exists");
        for _ in 0..50 {
            let n = m.settle(&mut sim.api, c, 50);
            let now = simenv.env.clock.now();
            match m.next_wake() {
                Some(t) if t > now => simenv.clock.advance(t.saturating_since(now)),
                _ if n == 0 => break,
                _ => {}
            }
        }
        check_settled(&mut sim, c);
    }

    // Invariant: after catching up without faults, every informer mirrors
    // its cluster exactly.
    let mut relists = 0;
    for c in &names {
        let mut i = informers.remove(c).expect("listed");
        i.sync(&mut sim, c, None);
        let (_, l) = sim.call(c, HttpRequest::new("GET", CMS), &[200]);
        let truth: Mirror = l["items"]
            .as_array()
            .into_iter()
            .flatten()
            .map(entry)
            .collect();
        if i.mirror != truth {
            sim.violations
                .push(format!("{c}: informer mirror diverged from the cluster"));
        }
        let _ = writeln!(
            sim.trace,
            "final {c} rv={} objects={} relists={}",
            l["metadata"]["resourceVersion"],
            truth.len(),
            i.relists
        );
        relists += i.relists;
    }
    for line in sim.api.take_trace() {
        let _ = writeln!(sim.trace, "final {line}");
    }

    Outcome {
        seed,
        trace_hash: fnv1a(sim.trace.as_bytes()),
        trace_lines: sim.trace.lines().count(),
        faults: faults.injected().len(),
        forks,
        relists,
        violations: sim.violations,
    }
}

/// Removes the sim's finalizer from every object: whoever owned it is
/// done, so nothing it held can stay stuck.
fn release_holds(sim: &mut Sim<'_>, c: &str) {
    let (_, l) = sim.call(c, HttpRequest::new("GET", CMS), &[200]);
    for item in l["items"].as_array().cloned().unwrap_or_default() {
        if !object::finalizers(&item).iter().any(|f| f == FINALIZER) {
            continue;
        }
        let mut obj = item.clone();
        obj["kind"] = json!("ConfigMap");
        obj["apiVersion"] = json!("v1");
        let fins: Vec<String> = object::finalizers(&item)
            .into_iter()
            .filter(|f| f != FINALIZER)
            .collect();
        obj["metadata"]["finalizers"] = json!(fins);
        let p = format!("{}/{}", cms(object::namespace(&item)), object::name(&item));
        sim.call(c, HttpRequest::new("PUT", &p).json(&obj), &[200]);
    }
}

/// No two live objects of cluster `c` share an IP, and each is in its
/// CIDR. Reads the store directly, so the check leaves no trace.
fn check_ips(sim: &mut Sim<'_>, c: &str) {
    let Some(net) = sim.api.network(c).copied() else {
        return;
    };
    let Ok(cs) = sim.api.store().cluster(c) else {
        return;
    };
    let list = |plural: &str| {
        let r = kubemu_controllers::res("", plural);
        cs.list(&object::storage_resource(r), None)
            .0
            .into_iter()
            .map(|(_, o)| o.value.clone())
            .collect::<Vec<_>>()
    };
    let mut held: BTreeMap<String, String> = BTreeMap::new();
    let mut bad = Vec::new();
    let mut hold = |ip: &str, cidr: Cidr, who: String| {
        if !ip.parse().is_ok_and(|a| cidr.contains(a)) {
            bad.push(format!("{who} has {ip}, outside {cidr}"));
        }
        if let Some(other) = held.insert(ip.to_owned(), who.clone()) {
            bad.push(format!("{who} and {other} share {ip}"));
        }
    };
    for s in list("services") {
        for ip in s["spec"]["clusterIPs"].as_array().into_iter().flatten() {
            let ip = ip.as_str().unwrap_or("");
            if ip != "None" {
                let who = format!("service {}/{}", object::namespace(&s), object::name(&s));
                hold(ip, net.service_cidr, who);
            }
        }
    }
    for p in list("pods") {
        if !kubemu_controllers::kubelet::holds_pod_ip(&p) {
            continue;
        }
        for ip in p["status"]["podIPs"].as_array().into_iter().flatten() {
            let who = format!("pod {}/{}", object::namespace(&p), object::name(&p));
            hold(ip["ip"].as_str().unwrap_or(""), net.pod_cidr, who);
        }
    }
    for b in bad {
        sim.violations.push(format!("step {}: {c}: {b}", sim.step));
    }
}

/// End-state invariants once controllers have settled.
fn check_settled(sim: &mut Sim<'_>, c: &str) {
    let mut all: Vec<(&str, Value)> = Vec::new();
    for r in stored() {
        let p = object::path(r, "", None, None);
        let (_, l) = sim.call(c, HttpRequest::new("GET", &p), &[200]);
        for item in l["items"].as_array().cloned().unwrap_or_default() {
            all.push((r.plural, item));
        }
    }
    let uids: Vec<&str> = all.iter().filter_map(|(_, v)| object::uid(v)).collect();
    let mut bad = Vec::new();
    for (plural, v) in &all {
        let what = format!("{plural} {}/{}", object::namespace(v), object::name(v));
        if object::is_deleting(v) {
            bad.push(format!(
                "{what} still terminating ({:?})",
                object::finalizers(v)
            ));
        }
        let refs = object::owner_refs(v);
        if !refs.is_empty()
            && !refs
                .iter()
                .any(|r| uids.contains(&r["uid"].as_str().unwrap_or("")))
        {
            bad.push(format!("{what} outlived all its owners"));
        }
        if *plural == "pods"
            && (v["spec"]["nodeName"].is_null() || v["status"]["phase"] != "Running")
        {
            bad.push(format!("{what} not running: {}", v["status"]));
        }
    }
    fn of<'a>(all: &'a [(&str, Value)], plural: &'a str) -> impl Iterator<Item = &'a Value> + 'a {
        all.iter()
            .filter(move |(p, _)| *p == plural)
            .map(|(_, v)| v)
    }
    for ns in of(&all, "namespaces") {
        let n = object::name(ns);
        if !of(&all, "serviceaccounts")
            .any(|sa| object::namespace(sa) == n && object::name(sa) == "default")
        {
            bad.push(format!("namespace {n} has no default ServiceAccount"));
        }
    }
    for svc in of(&all, "services") {
        let ns = object::namespace(svc);
        let app = &svc["spec"]["selector"]["app"];
        if app.is_null() {
            continue;
        }
        let mut want: Vec<&str> = of(&all, "pods")
            .filter(|p| object::namespace(p) == ns && p["metadata"]["labels"]["app"] == *app)
            .filter_map(|p| p["status"]["podIP"].as_str())
            .collect();
        want.sort_unstable();
        let ep = of(&all, "endpoints")
            .find(|e| object::namespace(e) == ns && object::name(e) == object::name(svc));
        let mut have: Vec<&str> = ep
            .and_then(|e| e["subsets"][0]["addresses"].as_array())
            .into_iter()
            .flatten()
            .filter_map(|a| a["ip"].as_str())
            .collect();
        have.sort_unstable();
        if ep.is_none() || have != want {
            let name = object::name(svc);
            bad.push(format!("endpoints {ns}/{name} are {have:?}, want {want:?}"));
        }
    }
    for b in bad {
        sim.violations.push(format!("{c} settled: {b}"));
    }
}

/// FNV-1a 64: stable, dependency-free trace fingerprint.
pub fn fnv1a(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |h, b| {
        (h ^ u64::from(*b)).wrapping_mul(0x0100_0000_01b3)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_same_trace() {
        let cfg = Config::default();
        for seed in 0..10 {
            assert_eq!(
                run(seed, &cfg).trace_hash,
                run(seed, &cfg).trace_hash,
                "seed {seed}"
            );
        }
    }

    #[test]
    fn forks_faults_and_relists_happen() {
        let cfg = Config::default();
        let o = run(0, &cfg);
        assert!(o.faults > 0, "default config should inject faults");
        assert!(o.forks > 0, "default config should fork clusters");
        assert!(
            o.relists > CLUSTERS.len() as u32,
            "faults and compaction should force relists"
        );
        let quiet = Config {
            faults: FaultConfig::NONE,
            ..Config::default()
        };
        assert_eq!(run(0, &quiet).faults, 0);
    }

    #[test]
    fn different_seeds_diverge() {
        let cfg = Config::default();
        assert_ne!(run(1, &cfg).trace_hash, run(2, &cfg).trace_hash);
    }

    #[test]
    fn invariants_hold_across_seeds() {
        let cfg = Config::default();
        for seed in 0..100 {
            let o = run(seed, &cfg);
            assert!(o.ok(), "seed {seed}: {:?}", o.violations);
        }
    }
}
