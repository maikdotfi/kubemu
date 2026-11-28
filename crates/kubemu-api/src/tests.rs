//! Frontend tests through HTTP-shaped requests, on a simulated Env.

use std::time::Duration;

use kubemu_env_sim::SimEnv;
use serde_json::{Value, json};

use crate::{Api, HttpRequest, Reply, WatchCursor};

struct T {
    sim: SimEnv,
    api: Api,
}

impl T {
    fn new() -> Self {
        let sim = SimEnv::new(7);
        let mut api = Api::with_log_capacity(sim.env.clone(), 50);
        api.add_cluster("c");
        Self { sim, api }
    }

    fn req(&mut self, r: HttpRequest) -> (u16, Value) {
        let resp = self.api.handle("c", &r).response();
        (resp.status, resp.value().unwrap_or(Value::Null))
    }

    fn get(&mut self, path: &str) -> (u16, Value) {
        self.req(HttpRequest::new("GET", path))
    }

    fn post(&mut self, path: &str, body: Value) -> (u16, Value) {
        self.req(HttpRequest::new("POST", path).json(&body))
    }

    fn put(&mut self, path: &str, body: Value) -> (u16, Value) {
        self.req(HttpRequest::new("PUT", path).json(&body))
    }

    fn patch(&mut self, path: &str, ct: &str, body: Value) -> (u16, Value) {
        self.req(
            HttpRequest::new("PATCH", path)
                .json(&body)
                .with_content_type(ct),
        )
    }

    fn delete(&mut self, path: &str) -> (u16, Value) {
        self.req(HttpRequest::new("DELETE", path))
    }

    fn watch(&mut self, path: &str) -> WatchCursor {
        match self.api.handle("c", &HttpRequest::new("GET", path)) {
            Reply::Watch(c) => c,
            Reply::Response(r) => panic!("expected a watch, got {} {:?}", r.status, r.value()),
        }
    }

    fn poll(&mut self, c: &mut WatchCursor) -> Vec<(String, String)> {
        self.api
            .watch_poll(c)
            .events
            .iter()
            .map(|e| {
                (
                    e["type"].as_str().unwrap().to_owned(),
                    e["object"]["metadata"]["name"]
                        .as_str()
                        .unwrap_or("")
                        .to_owned(),
                )
            })
            .collect()
    }
}

fn cm(name: &str) -> Value {
    json!({"apiVersion": "v1", "kind": "ConfigMap", "metadata": {"name": name, "labels": {"app": "x"}}, "data": {"k": "v"}})
}

const CMS: &str = "/api/v1/namespaces/default/configmaps";

#[test]
fn system_namespaces_exist() {
    let mut t = T::new();
    let (s, l) = t.get("/api/v1/namespaces");
    assert_eq!(s, 200);
    let names: Vec<&str> = l["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|n| n["metadata"]["name"].as_str().unwrap())
        .collect();
    assert_eq!(
        names,
        ["default", "kube-node-lease", "kube-public", "kube-system"]
    );
    assert_eq!(l["items"][0]["status"]["phase"], "Active");
    assert!(
        l["items"][0].get("kind").is_none(),
        "list items have no kind"
    );
}

#[test]
fn create_get_update_delete() {
    let mut t = T::new();
    let (s, c) = t.post(CMS, cm("a"));
    assert_eq!(s, 201);
    assert_eq!(c["metadata"]["namespace"], "default");
    assert_eq!(c["metadata"]["uid"].as_str().unwrap().len(), 36);
    assert_eq!(c["metadata"]["creationTimestamp"], "2025-01-01T00:00:00Z");
    let rv = c["metadata"]["resourceVersion"]
        .as_str()
        .unwrap()
        .to_owned();
    assert_eq!(t.post(CMS, cm("a")).0, 409);

    let (s, g) = t.get(&format!("{CMS}/a"));
    assert_eq!((s, &g), (200, &c));

    let mut u = g.clone();
    u["data"]["k"] = json!("w");
    let (s, u2) = t.put(&format!("{CMS}/a"), u.clone());
    assert_eq!(s, 200);
    assert_ne!(u2["metadata"]["resourceVersion"].as_str().unwrap(), rv);
    assert_eq!(u2["metadata"]["uid"], c["metadata"]["uid"]);
    // Stale resourceVersion: conflict.
    let (s, e) = t.put(&format!("{CMS}/a"), u);
    assert_eq!((s, e["reason"].as_str()), (409, Some("Conflict")));
    // No-op update keeps the resourceVersion.
    let (_, u3) = t.put(&format!("{CMS}/a"), u2.clone());
    assert_eq!(
        u3["metadata"]["resourceVersion"],
        u2["metadata"]["resourceVersion"]
    );

    let (s, d) = t.delete(&format!("{CMS}/a"));
    assert_eq!((s, d["status"].as_str()), (200, Some("Success")));
    let (s, e) = t.get(&format!("{CMS}/a"));
    assert_eq!((s, e["reason"].as_str()), (404, Some("NotFound")));
    assert_eq!(e["message"], "configmaps \"a\" not found");
}

#[test]
fn create_rules() {
    let mut t = T::new();
    let (s, c) = t.post(CMS, json!({"metadata": {"generateName": "gen-"}}));
    assert_eq!(s, 201);
    assert!(c["metadata"]["name"].as_str().unwrap().starts_with("gen-"));
    assert_eq!(c["kind"], "ConfigMap");
    assert_eq!(t.post(CMS, json!({"metadata": {}})).0, 422);
    let (s, e) = t.post(CMS, cm("Bad_Name"));
    assert_eq!((s, e["reason"].as_str()), (422, Some("Invalid")));
    assert!(
        e["message"]
            .as_str()
            .unwrap()
            .contains("RFC 1123 subdomain")
    );
    assert_eq!(
        t.post(CMS, json!({"kind": "Secret", "metadata": {"name": "x"}}))
            .0,
        400
    );
    assert_eq!(
        t.post(
            CMS,
            json!({"metadata": {"name": "x", "namespace": "other"}})
        )
        .0,
        400
    );
    assert_eq!(t.post("/api/v1/namespaces/nope/configmaps", cm("x")).0, 404);
    assert_eq!(
        t.post("/api/v1/services", cm("x")).0,
        405,
        "no create across namespaces"
    );
    assert_eq!(
        t.post(
            CMS,
            json!({"metadata": {"name": "rv", "resourceVersion": "5"}})
        )
        .0,
        400
    );
    let (s, _) = t.req(
        HttpRequest::new("POST", CMS)
            .json(&cm("y"))
            .with_content_type("application/vnd.kubernetes.protobuf"),
    );
    assert_eq!(s, 415);
    let (s, _) =
        t.req(HttpRequest::new("GET", CMS).with_accept("application/vnd.kubernetes.protobuf"));
    assert_eq!(s, 406);
}

#[test]
fn unknown_fields_and_dry_run() {
    let mut t = T::new();
    let mut body = cm("u");
    body["bogus"] = json!(1);
    let r = t
        .api
        .handle("c", &HttpRequest::new("POST", CMS).json(&body))
        .response();
    assert_eq!(r.status, 201);
    assert_eq!(r.warnings, ["unknown field \"bogus\""]);
    assert!(r.value().unwrap().get("bogus").is_none());
    assert_eq!(
        t.post(&format!("{CMS}?fieldValidation=Strict"), body.clone())
            .0,
        400
    );
    assert_eq!(t.post(&format!("{CMS}?dryRun=All"), cm("dry")).0, 201);
    assert_eq!(t.get(&format!("{CMS}/dry")).0, 404);
}

#[test]
fn patches() {
    let mut t = T::new();
    t.post(CMS, cm("p"));
    let p = format!("{CMS}/p");
    let (s, v) = t.patch(
        &p,
        "application/merge-patch+json",
        json!({"data": {"k": null, "n": "1"}}),
    );
    assert_eq!((s, &v["data"]), (200, &json!({"n": "1"})));
    let (s, v) = t.patch(
        &p,
        "application/json-patch+json",
        json!([{"op": "add", "path": "/data/m", "value": "2"}]),
    );
    assert_eq!((s, &v["data"]["m"]), (200, &json!("2")));
    let (s, v) = t.patch(
        &p,
        "application/strategic-merge-patch+json",
        json!({"metadata": {"labels": {"b": "c"}}}),
    );
    assert_eq!(
        (s, &v["metadata"]["labels"]),
        (200, &json!({"app": "x", "b": "c"}))
    );
    assert_eq!(
        t.patch(
            &p,
            "application/json-patch+json",
            json!([{"op": "test", "path": "/data/m", "value": "x"}])
        )
        .0,
        422
    );
    assert_eq!(
        t.patch(&p, "application/apply-patch+yaml", json!({})).0,
        415
    );
    // A patch setting a stale resourceVersion conflicts.
    assert_eq!(
        t.patch(
            &p,
            "application/merge-patch+json",
            json!({"metadata": {"resourceVersion": "1"}})
        )
        .0,
        409
    );
    assert_eq!(
        t.patch(
            &format!("{CMS}/nope"),
            "application/merge-patch+json",
            json!({})
        )
        .0,
        404
    );
}

#[test]
fn status_subresource() {
    let mut t = T::new();
    let pods = "/api/v1/namespaces/default/pods";
    let pod = json!({"metadata": {"name": "p"}, "spec": {"containers": [{"name": "c", "image": "i"}]}, "status": {"phase": "Running"}});
    let (s, p) = t.post(pods, pod);
    assert_eq!(s, 201);
    assert_eq!(p["status"], json!({"phase": "Pending"}));
    assert_eq!(p["spec"]["serviceAccountName"], "default");
    assert_eq!(p["metadata"]["generation"], 1);
    // The main resource ignores status...
    let mut u = p.clone();
    u["status"]["phase"] = json!("Running");
    u["metadata"]["labels"] = json!({"a": "b"});
    let (_, u) = t.put(&format!("{pods}/p"), u);
    assert_eq!(u["status"]["phase"], "Pending");
    assert_eq!(u["metadata"]["labels"]["a"], "b");
    assert_eq!(
        u["metadata"]["generation"], 1,
        "metadata changes keep the generation"
    );
    // ...and /status changes nothing else.
    let mut s = u.clone();
    s["status"]["phase"] = json!("Running");
    s["metadata"]["labels"] = json!({});
    let (_, s) = t.put(&format!("{pods}/p/status"), s);
    assert_eq!(s["status"]["phase"], "Running");
    assert_eq!(s["metadata"]["labels"]["a"], "b");
    let (_, g) = t.patch(
        &format!("{pods}/p"),
        "application/merge-patch+json",
        json!({"spec": {"activeDeadlineSeconds": 5}}),
    );
    assert_eq!(g["metadata"]["generation"], 2);
    assert_eq!(
        t.put(&format!("{CMS}/x/status"), cm("x")).0,
        404,
        "configmaps have no status"
    );
}

#[test]
fn selectors_and_pagination() {
    let mut t = T::new();
    for i in 0..5 {
        let mut c = cm(&format!("c{i}"));
        c["metadata"]["labels"]["n"] = json!(i.to_string());
        t.post(CMS, c);
    }
    let names = |l: &Value| -> Vec<String> {
        l["items"]
            .as_array()
            .unwrap()
            .iter()
            .map(|i| i["metadata"]["name"].as_str().unwrap().to_owned())
            .collect()
    };
    let (_, l) = t.get(&format!("{CMS}?labelSelector=n%20in%20(1,3)"));
    assert_eq!(names(&l), ["c1", "c3"]);
    let (_, l) = t.get(&format!("{CMS}?fieldSelector=metadata.name!=c0"));
    assert_eq!(names(&l).len(), 4);
    assert_eq!(
        t.get(&format!("{CMS}?fieldSelector=spec.nodeName=x")).0,
        400
    );

    let (_, p1) = t.get(&format!("{CMS}?limit=2"));
    assert_eq!(names(&p1), ["c0", "c1"]);
    assert_eq!(p1["metadata"]["remainingItemCount"], 3);
    // Changes after the first page are not seen by its continuation.
    t.post(CMS, cm("c10"));
    t.delete(&format!("{CMS}/c3"));
    let c = p1["metadata"]["continue"].as_str().unwrap().to_owned();
    let (_, p2) = t.get(&format!("{CMS}?limit=10&continue={c}"));
    assert_eq!(names(&p2), ["c2", "c3", "c4"]);
    assert_eq!(
        p2["metadata"]["resourceVersion"],
        p1["metadata"]["resourceVersion"]
    );
    assert!(p2["metadata"].get("continue").is_none());
}

#[test]
fn resource_version_semantics() {
    let mut t = T::new();
    let (_, a) = t.post(CMS, cm("a"));
    let rv_a: u64 = a["metadata"]["resourceVersion"]
        .as_str()
        .unwrap()
        .parse()
        .unwrap();
    t.post(CMS, cm("b"));
    t.delete(&format!("{CMS}/a"));
    let (s, l) = t.get(&format!(
        "{CMS}?resourceVersion={rv_a}&resourceVersionMatch=Exact"
    ));
    assert_eq!(s, 200);
    assert_eq!(l["items"].as_array().unwrap().len(), 1);
    assert_eq!(l["items"][0]["metadata"]["name"], "a");
    assert_eq!(l["metadata"]["resourceVersion"], rv_a.to_string());
    let (s, e) = t.get(&format!("{CMS}?resourceVersion=999999"));
    assert_eq!(
        (s, e["details"]["causes"][0]["reason"].as_str()),
        (504, Some("ResourceVersionTooLarge"))
    );
    assert_eq!(t.get(&format!("{CMS}?resourceVersionMatch=Exact")).0, 400);
    // Past the log (capacity 50): 410.
    for i in 0..60 {
        t.post(CMS, cm(&format!("x{i}")));
    }
    let (s, e) = t.get(&format!(
        "{CMS}?resourceVersion={rv_a}&resourceVersionMatch=Exact"
    ));
    assert_eq!((s, e["reason"].as_str()), (410, Some("Expired")));
}

#[test]
fn watches() {
    let mut t = T::new();
    t.post(CMS, cm("a"));
    let mut w = t.watch(&format!("{CMS}?watch=true&labelSelector=app=x"));
    assert_eq!(t.poll(&mut w), [("ADDED".into(), "a".into())]);
    t.post(CMS, cm("b"));
    let (_, mut b) = t.get(&format!("{CMS}/b"));
    b["metadata"]["labels"] = json!({"app": "y"});
    t.put(&format!("{CMS}/b"), b);
    t.delete(&format!("{CMS}/a"));
    assert_eq!(
        t.poll(&mut w),
        [
            ("ADDED".into(), "b".into()),
            ("DELETED".into(), "b".into()),
            ("DELETED".into(), "a".into())
        ]
    );
    assert!(t.poll(&mut w).is_empty());

    // From a resourceVersion: no initial events.
    let rv = w.resource_version();
    t.post(CMS, cm("c"));
    let mut w2 = t.watch(&format!("{CMS}?watch=1&resourceVersion={rv}"));
    assert_eq!(t.poll(&mut w2), [("ADDED".into(), "c".into())]);

    // Legacy single-object watch.
    let mut w3 = t.watch(&format!(
        "/api/v1/watch/namespaces/default/configmaps/c?resourceVersion={rv}"
    ));
    t.post(CMS, cm("d"));
    assert_eq!(t.poll(&mut w3), [("ADDED".into(), "c".into())]);

    // Compacted: an ERROR event with a 410 Status, then the end.
    for i in 0..60 {
        t.post(CMS, cm(&format!("x{i}")));
    }
    let mut old = t.watch(&format!("{CMS}?watch=1&resourceVersion={rv}"));
    let b = t.api.watch_poll(&mut old);
    assert!(b.closed);
    assert_eq!(b.events[0]["type"], "ERROR");
    assert_eq!(b.events[0]["object"]["code"], 410);
}

#[test]
fn watch_list_and_bookmarks() {
    let mut t = T::new();
    t.post(CMS, cm("a"));
    let mut w = t.watch(&format!(
        "{CMS}?watch=1&sendInitialEvents=true&resourceVersionMatch=NotOlderThan&allowWatchBookmarks=true&timeoutSeconds=120"
    ));
    let b = t.api.watch_poll(&mut w);
    assert_eq!(b.events[0]["type"], "ADDED");
    assert_eq!(b.events[1]["type"], "BOOKMARK");
    assert_eq!(
        b.events[1]["object"]["metadata"]["annotations"]["k8s.io/initial-events-end"],
        "true"
    );
    t.sim.clock.advance(Duration::from_secs(61));
    let b = t.api.watch_poll(&mut w);
    assert_eq!(b.events.len(), 1);
    assert_eq!(b.events[0]["type"], "BOOKMARK");
    assert!(!b.closed);
    t.sim.clock.advance(Duration::from_secs(60));
    assert!(
        t.api.watch_poll(&mut w).closed,
        "timeoutSeconds ends the watch"
    );
    assert_eq!(
        t.get(&format!("{CMS}?watch=1&sendInitialEvents=true")).0,
        400
    );
}

#[test]
fn finalizers_and_propagation() {
    let mut t = T::new();
    let mut c = cm("f");
    c["metadata"]["finalizers"] = json!(["example.com/hold"]);
    t.post(CMS, c);
    let (s, d) = t.delete(&format!("{CMS}/f"));
    assert_eq!(s, 200);
    assert!(d["metadata"]["deletionTimestamp"].is_string());
    let (_, mut g) = t.get(&format!("{CMS}/f"));
    // No new finalizers once deleting.
    g["metadata"]["finalizers"] = json!(["example.com/hold", "x.io/new"]);
    assert_eq!(t.put(&format!("{CMS}/f"), g.clone()).0, 422);
    g["metadata"]["finalizers"] = json!([]);
    assert_eq!(t.put(&format!("{CMS}/f"), g).0, 200);
    assert_eq!(
        t.get(&format!("{CMS}/f")).0,
        404,
        "last finalizer removed: gone"
    );

    t.post(CMS, cm("o"));
    let (_, d) = t.req(
        HttpRequest::new("DELETE", &format!("{CMS}/o"))
            .json(&json!({"propagationPolicy": "Orphan"})),
    );
    assert_eq!(d["metadata"]["finalizers"], json!(["orphan"]));
    assert_eq!(
        t.req(HttpRequest::new(
            "DELETE",
            &format!("{CMS}/o?propagationPolicy=Bogus")
        ))
        .0,
        422
    );
}

#[test]
fn namespace_lifecycle() {
    let mut t = T::new();
    assert_eq!(t.delete("/api/v1/namespaces/default").0, 403);
    assert_eq!(
        t.post("/api/v1/namespaces", json!({"metadata": {"name": "team"}}))
            .0,
        201
    );
    assert_eq!(t.post("/api/v1/namespaces/team/configmaps", cm("a")).0, 201);
    let (s, ns) = t.delete("/api/v1/namespaces/team");
    assert_eq!(s, 200);
    assert_eq!(ns["status"]["phase"], "Terminating");
    let (s, e) = t.post("/api/v1/namespaces/team/configmaps", cm("b"));
    assert_eq!((s, e["reason"].as_str()), (403, Some("Forbidden")));
    assert_eq!(
        t.get("/api/v1/namespaces/team").0,
        200,
        "the namespace controller finishes it"
    );
    // What the namespace controller does once the content is gone.
    t.delete("/api/v1/namespaces/team/configmaps/a");
    let (_, mut ns) = t.get("/api/v1/namespaces/team");
    ns["spec"]["finalizers"] = json!([]);
    ns["metadata"]["labels"] = json!({"ignored": "by /finalize"});
    assert_eq!(t.put("/api/v1/namespaces/team/finalize", ns).0, 200);
    assert_eq!(t.get("/api/v1/namespaces/team").0, 404);
}

#[test]
fn binding_assigns_a_node_once() {
    let mut t = T::new();
    let pods = "/api/v1/namespaces/default/pods";
    t.post(
        pods,
        json!({"metadata": {"name": "p"}, "spec": {"containers": [{"name": "c", "image": "i"}]}}),
    );
    let b = json!({"apiVersion": "v1", "kind": "Binding", "metadata": {"name": "p"}, "target": {"kind": "Node", "name": "n1"}});
    assert_eq!(t.post(&format!("{pods}/p/binding"), b.clone()).0, 201);
    let (_, p) = t.get(&format!("{pods}/p"));
    assert_eq!(p["spec"]["nodeName"], "n1");
    assert_eq!(p["status"]["conditions"][0]["type"], "PodScheduled");
    assert_eq!(t.post(&format!("{pods}/p/binding"), b).0, 409);
    assert_eq!(t.post(&format!("{CMS}/x/binding"), json!({})).0, 404);
}

#[test]
fn pod_graceful_deletion() {
    let mut t = T::new();
    let pods = "/api/v1/namespaces/default/pods";
    let pod = |n: &str, node: Option<&str>| {
        let mut p =
            json!({"metadata": {"name": n}, "spec": {"containers": [{"name": "c", "image": "i"}]}});
        if let Some(node) = node {
            p["spec"]["nodeName"] = json!(node);
        }
        p
    };
    t.post(pods, pod("unscheduled", None));
    let (s, d) = t.delete(&format!("{pods}/unscheduled"));
    assert_eq!(
        (s, d["kind"].as_str()),
        (200, Some("Pod")),
        "pods return the deleted object"
    );
    assert_eq!(t.get(&format!("{pods}/unscheduled")).0, 404);

    t.post(pods, pod("running", Some("n1")));
    let (_, d) = t.delete(&format!("{pods}/running"));
    assert_eq!(d["metadata"]["deletionGracePeriodSeconds"], 30);
    assert_eq!(
        t.get(&format!("{pods}/running")).0,
        200,
        "waits for the kubelet"
    );
    t.delete(&format!("{pods}/running?gracePeriodSeconds=0"));
    assert_eq!(t.get(&format!("{pods}/running")).0, 404);
}

#[test]
fn discovery_and_routing() {
    let mut t = T::new();
    let r = t
        .api
        .handle(
            "c",
            &HttpRequest::new("GET", "/apis")
                .with_accept("application/json;g=apidiscovery.k8s.io;v=v2;as=APIGroupDiscoveryList,application/json"),
        )
        .response();
    assert!(r.content_type.contains("apidiscovery.k8s.io"));
    assert_eq!(t.get("/api/v1").1["groupVersion"], "v1");
    assert_eq!(t.get("/version").1["minor"], "37");
    assert_eq!(t.get("/apis/apps/v1/namespaces/default/deployments").0, 404);
    assert_eq!(
        t.get("/api/v1/namespaces/default/nodes").0,
        404,
        "nodes are cluster-scoped"
    );
    assert_eq!(t.get("/api/v1/namespaces/default/pods/x/exec").0, 404);
    assert_eq!(
        t.req(HttpRequest::new(
            "POST",
            "/api/v1/namespaces/default/configmaps/x"
        ))
        .0,
        405
    );
    assert_eq!(t.get("/healthz").0, 200);
    let trace = t.api.take_trace();
    assert!(trace.iter().any(|l| l.contains("GET /api/v1 -> 200")));
}

#[test]
fn forks_are_isolated() {
    let mut t = T::new();
    t.post(CMS, cm("a"));
    t.api.fork_cluster("c", "f").unwrap();
    t.delete(&format!("{CMS}/a"));
    let r = t
        .api
        .handle("f", &HttpRequest::new("GET", &format!("{CMS}/a")))
        .response();
    assert_eq!(r.status, 200);
}

#[test]
fn self_subject_reviews_allow_everything() {
    let mut t = T::new();
    let (s, r) = t.post(
        "/apis/authorization.k8s.io/v1/selfsubjectaccessreviews",
        json!({"spec": {"resourceAttributes": {"verb": "create", "resource": "pods"}}}),
    );
    assert_eq!((s, r["status"]["allowed"].as_bool()), (201, Some(true)));
    let (s, r) = t.post(
        "/apis/authorization.k8s.io/v1/selfsubjectrulesreviews",
        json!({"spec": {"namespace": "default"}}),
    );
    assert_eq!(
        (s, r["status"]["resourceRules"][0]["verbs"][0].as_str()),
        (201, Some("*"))
    );
    assert_eq!(
        t.get("/apis/authorization.k8s.io/v1/selfsubjectaccessreviews")
            .0,
        405
    );
}

#[test]
fn defaulting_on_writes() {
    let mut t = T::new();
    let (code, pod) = t.post(
        "/api/v1/namespaces/default/pods",
        json!({"apiVersion": "v1", "kind": "Pod", "metadata": {"name": "p"},
               "spec": {"containers": [{"name": "c", "image": "nginx",
                   "ports": [{"containerPort": 80}],
                   "resources": {"limits": {"cpu": "1000m", "memory": "1n"}}}]}}),
    );
    assert_eq!(code, 201, "{pod}");
    let spec = &pod["spec"];
    assert_eq!(spec["restartPolicy"], "Always");
    assert_eq!(spec["dnsPolicy"], "ClusterFirst");
    assert_eq!(spec["terminationGracePeriodSeconds"], 30);
    assert_eq!(spec["schedulerName"], "default-scheduler");
    let c = &spec["containers"][0];
    assert_eq!(c["imagePullPolicy"], "Always");
    assert_eq!(c["ports"][0]["protocol"], "TCP");
    assert_eq!(c["terminationMessagePath"], "/dev/termination-log");
    // Canonical, rounded up to milli units, and requests from limits.
    assert_eq!(
        c["resources"],
        json!({"limits": {"cpu": "1", "memory": "1m"}, "requests": {"cpu": "1", "memory": "1m"}})
    );

    let (code, svc) = t.post(
        "/api/v1/namespaces/default/services",
        json!({"apiVersion": "v1", "kind": "Service", "metadata": {"name": "s"},
               "spec": {"ports": [{"port": 80}]}}),
    );
    assert_eq!(code, 201, "{svc}");
    assert_eq!(svc["spec"]["type"], "ClusterIP");
    assert_eq!(svc["spec"]["sessionAffinity"], "None");
    assert_eq!(svc["spec"]["internalTrafficPolicy"], "Cluster");
    assert_eq!(
        svc["spec"]["ports"][0],
        json!({"port": 80, "protocol": "TCP", "targetPort": 80})
    );

    // Defaulting runs on patches too.
    let (code, svc) = t.patch(
        "/api/v1/namespaces/default/services/s",
        "application/merge-patch+json",
        json!({"spec": {"ports": [{"port": 81}]}}),
    );
    assert_eq!(code, 200, "{svc}");
    assert_eq!(svc["spec"]["ports"][0]["targetPort"], 81);
}

#[test]
fn namespace_name_label() {
    let mut t = T::new();
    let (_, ns) = t.get("/api/v1/namespaces/default");
    assert_eq!(
        ns["metadata"]["labels"]["kubernetes.io/metadata.name"],
        "default"
    );
    let (code, ns) = t.post(
        "/api/v1/namespaces",
        json!({"apiVersion": "v1", "kind": "Namespace", "metadata": {"generateName": "gen-"}}),
    );
    assert_eq!(code, 201, "{ns}");
    let name = ns["metadata"]["name"].as_str().unwrap();
    assert_eq!(
        ns["metadata"]["labels"]["kubernetes.io/metadata.name"],
        name
    );
}

#[test]
fn strategies_match_upstream() {
    let mut t = T::new();
    // Services allow create-on-update; endpoint slices do not.
    let svc = json!({"apiVersion": "v1", "kind": "Service", "metadata": {"name": "s"},
                     "spec": {"ports": [{"port": 80}]}});
    assert_eq!(t.put("/api/v1/namespaces/default/services/s", svc).0, 201);
    let eps = json!({"apiVersion": "discovery.k8s.io/v1", "kind": "EndpointSlice",
                     "metadata": {"name": "e"}, "addressType": "IPv4", "endpoints": []});
    let path = "/apis/discovery.k8s.io/v1/namespaces/default/endpointslices";
    assert_eq!(t.put(&format!("{path}/e"), eps.clone()).0, 404);
    // Endpoint slices track generation: labels and content bump it.
    let (_, e) = t.post(path, eps.clone());
    assert_eq!(e["metadata"]["generation"], 1);
    let (_, e) = t.patch(
        &format!("{path}/e"),
        "application/merge-patch+json",
        json!({"metadata": {"annotations": {"a": "b"}}}),
    );
    assert_eq!(e["metadata"]["generation"], 1);
    let (_, e) = t.patch(
        &format!("{path}/e"),
        "application/merge-patch+json",
        json!({"metadata": {"labels": {"a": "b"}}}),
    );
    assert_eq!(e["metadata"]["generation"], 2);
    let (_, e) = t.patch(
        &format!("{path}/e"),
        "application/merge-patch+json",
        json!({"endpoints": [{"addresses": ["10.0.0.1"]}]}),
    );
    assert_eq!(e["metadata"]["generation"], 3);
}

#[test]
fn per_kind_validation() {
    let mut t = T::new();
    let (code, st) = t.post(
        "/api/v1/namespaces/default/pods",
        json!({"apiVersion": "v1", "kind": "Pod", "metadata": {"name": "p"},
               "spec": {"containers": [{"name": "c", "ports": [{"containerPort": 70000}],
                   "resources": {"limits": {"cpu": "1"}, "requests": {"cpu": "2"}}}]}}),
    );
    assert_eq!(code, 422, "{st}");
    let fields: Vec<&str> = st["details"]["causes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["field"].as_str().unwrap())
        .collect();
    assert_eq!(
        fields,
        [
            "spec.containers[0].image",
            "spec.containers[0].ports[0].containerPort",
            "spec.containers[0].resources.requests"
        ]
    );
    assert!(
        st["message"]
            .as_str()
            .unwrap()
            .starts_with("Pod \"p\" is invalid: [")
    );

    // Service names are DNS-1123 labels (RelaxedServiceNameValidation).
    let svc = |name: &str, ports: Value| json!({"apiVersion": "v1", "kind": "Service", "metadata": {"name": name}, "spec": {"ports": ports}});
    let p = "/api/v1/namespaces/default/services";
    assert_eq!(t.post(p, svc("1svc", json!([{"port": 80}]))).0, 201);
    let (code, st) = t.post(p, svc("two", json!([{"port": 80}, {"port": 81}])));
    assert_eq!(code, 422, "{st}");
    assert_eq!(st["details"]["causes"][0]["field"], "spec.ports[0].name");
}
