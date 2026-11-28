//! A fake kubelet (the M1 `SimExecutor`, README §4.1): it registers a
//! logical Node and "runs" the pods assigned to it without running
//! anything. After a seeded delay a pod becomes Running and Ready, with
//! the status a real kubelet reports (conditions, IPs, container
//! statuses, QoS class). A pod being deleted gracefully is finished with
//! a forced delete, as the kubelet does once its containers stop.
//!
//! Pod IPs come from the cluster's pod CIDR (ADR 0003), allocated when a
//! pod starts. The node owns the whole CIDR; its first address is the
//! bridge gateway. The IPs in use are those of the live pods in the store,
//! so nothing else records them.
//!
//! The real executor (youki, M2) replaces the "run" part; registration
//! and status reporting stay this shape.

use std::collections::{BTreeMap, BTreeSet};
use std::net::Ipv4Addr;
use std::time::Duration;

use kubemu_api::{Cidr, object};
use kubemu_env::{Env, MonoTime};
use serde_json::{Value, json};

use crate::{Action, Controller, View, res};

pub const NODE_IP: &str = "172.18.0.2";

#[derive(Debug)]
pub struct SimKubelet {
    env: Env,
    node: String,
    pod_cidr: Cidr,
    /// Pod UID -> when it is "started".
    ready_at: BTreeMap<String, MonoTime>,
}

/// Whether a pod holds its pod IP: it has one of its own and has not
/// finished.
pub fn holds_pod_ip(pod: &Value) -> bool {
    pod["spec"]["hostNetwork"] != true
        && !matches!(
            pod["status"]["phase"].as_str(),
            Some("Succeeded" | "Failed")
        )
}

/// The pod IPs held by `pods`.
pub fn pod_ips<'a>(pods: impl Iterator<Item = &'a Value>) -> BTreeSet<Ipv4Addr> {
    pods.filter(|p| holds_pod_ip(p))
        .flat_map(|p| p["status"]["podIPs"].as_array().into_iter().flatten())
        .filter_map(|ip| ip["ip"].as_str()?.parse().ok())
        .collect()
}

impl SimKubelet {
    pub fn new(env: Env, node: &str, pod_cidr: Cidr) -> Self {
        Self {
            env,
            node: node.to_owned(),
            pod_cidr,
            ready_at: BTreeMap::new(),
        }
    }

    pub fn node_name(&self) -> &str {
        &self.node
    }

    fn now_rfc3339(&self) -> String {
        kubemu_api::object::rfc3339_now(&self.env)
    }

    fn node_object(&self) -> Value {
        let now = self.now_rfc3339();
        let cond = |t: &str, s: &str, reason: &str, msg: &str| json!({"type": t, "status": s, "lastHeartbeatTime": now, "lastTransitionTime": now, "reason": reason, "message": msg});
        let resources =
            json!({"cpu": "8", "memory": "16Gi", "pods": "110", "ephemeral-storage": "100Gi"});
        json!({
            "apiVersion": "v1",
            "kind": "Node",
            "metadata": {
                "name": self.node,
                "labels": {
                    "kubernetes.io/hostname": self.node,
                    "kubernetes.io/os": "linux",
                    "kubernetes.io/arch": "amd64",
                    "beta.kubernetes.io/os": "linux",
                    "beta.kubernetes.io/arch": "amd64",
                },
            },
            "spec": {"podCIDR": self.pod_cidr.to_string(), "podCIDRs": [self.pod_cidr.to_string()]},
            "status": {
                "capacity": resources,
                "allocatable": resources,
                "conditions": [
                    cond("MemoryPressure", "False", "KubeletHasSufficientMemory", "kubelet has sufficient memory available"),
                    cond("DiskPressure", "False", "KubeletHasNoDiskPressure", "kubelet has no disk pressure"),
                    cond("PIDPressure", "False", "KubeletHasSufficientPID", "kubelet has sufficient PID available"),
                    cond("Ready", "True", "KubeletReady", "kubelet is posting ready status (kubemu SimKubelet)"),
                ],
                "addresses": [
                    {"type": "InternalIP", "address": NODE_IP},
                    {"type": "Hostname", "address": self.node},
                ],
                "daemonEndpoints": {"kubeletEndpoint": {"Port": 10250}},
                "nodeInfo": {
                    "machineID": "kubemu",
                    "systemUUID": "kubemu",
                    "bootID": "kubemu",
                    "kernelVersion": "kubemu",
                    "osImage": "kubemu",
                    "containerRuntimeVersion": "kubemu://sim",
                    "kubeletVersion": kubemu_api_types::KUBERNETES_VERSION,
                    "kubeProxyVersion": "",
                    "operatingSystem": "linux",
                    "architecture": "amd64",
                },
            },
        })
    }

    /// The pod as Running, with IP `ip`.
    fn running(&self, pod: &Value, ip: &str) -> Value {
        let uid = object::uid(pod).unwrap_or("").to_owned();
        let now = self.now_rfc3339();
        let cid = |name: &str| format!("kubemu://{}-{name}", &uid[..uid.len().min(8)]);
        let containers: Vec<Value> = pod["spec"]["containers"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|c| {
                let name = c["name"].as_str().unwrap_or("");
                json!({
                    "name": name,
                    "image": c["image"],
                    "imageID": "",
                    "containerID": cid(name),
                    "ready": true,
                    "started": true,
                    "restartCount": 0,
                    "state": {"running": {"startedAt": now}},
                    "lastState": {},
                })
            })
            .collect();
        let inits: Vec<Value> = pod["spec"]["initContainers"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|c| {
                let name = c["name"].as_str().unwrap_or("");
                json!({
                    "name": name,
                    "image": c["image"],
                    "imageID": "",
                    "containerID": cid(name),
                    "ready": true,
                    "started": false,
                    "restartCount": 0,
                    "state": {"terminated": {"exitCode": 0, "reason": "Completed", "startedAt": now, "finishedAt": now, "containerID": cid(name)}},
                    "lastState": {},
                })
            })
            .collect();
        let cond = |t: &str| json!({"type": t, "status": "True", "lastProbeTime": null, "lastTransitionTime": now});
        let scheduled = pod["status"]["conditions"]
            .as_array()
            .and_then(|cs| cs.iter().find(|c| c["type"] == "PodScheduled").cloned())
            .unwrap_or_else(|| cond("PodScheduled"));
        let mut status = json!({
            "phase": "Running",
            "conditions": [
                cond("PodReadyToStartContainers"),
                cond("Initialized"),
                cond("Ready"),
                cond("ContainersReady"),
                scheduled,
            ],
            "hostIP": NODE_IP,
            "hostIPs": [{"ip": NODE_IP}],
            "podIP": ip,
            "podIPs": [{"ip": ip}],
            "startTime": now,
            "containerStatuses": containers,
            "qosClass": qos_class(pod),
        });
        if !inits.is_empty() {
            status["initContainerStatuses"] = json!(inits);
        }
        let mut p = pod.clone();
        p["status"] = status;
        p
    }
}

/// The pod's QoS class from its containers' requests and limits.
pub fn qos_class(pod: &Value) -> &'static str {
    let all: Vec<&Value> = ["initContainers", "containers"]
        .iter()
        .flat_map(|k| pod["spec"][*k].as_array().into_iter().flatten())
        .collect();
    let mut any = false;
    let mut guaranteed = true;
    for c in &all {
        let req = c["resources"]["requests"].as_object();
        let lim = c["resources"]["limits"].as_object();
        if req.is_some_and(|r| !r.is_empty()) || lim.is_some_and(|l| !l.is_empty()) {
            any = true;
        }
        for res in ["cpu", "memory"] {
            let l = lim.and_then(|l| l.get(res));
            let r = req.and_then(|r| r.get(res)).or(l);
            if l.is_none() || r != l {
                guaranteed = false;
            }
        }
    }
    match (any, guaranteed) {
        (false, _) => "BestEffort",
        (true, true) => "Guaranteed",
        (true, false) => "Burstable",
    }
}

impl Controller for SimKubelet {
    fn name(&self) -> &'static str {
        "kubelet"
    }

    fn reconcile(&mut self, view: View<'_>, now: MonoTime) -> Vec<Action> {
        let nodes = res("", "nodes");
        let pods = res("", "pods");
        let mut actions = Vec::new();
        if view.get(nodes, "", &self.node).is_none() {
            actions.push(Action::create(nodes, "", &self.node_object()));
        }
        let mine: Vec<&Value> = view
            .list(pods, None)
            .into_iter()
            .filter(|p| p["spec"]["nodeName"] == self.node.as_str())
            .collect();
        let live: Vec<String> = mine
            .iter()
            .filter_map(|p| object::uid(p))
            .map(str::to_owned)
            .collect();
        self.ready_at.retain(|u, _| live.contains(u));
        let mut used = pod_ips(view.list(pods, None).into_iter());

        let mut wake: Option<MonoTime> = None;
        for p in mine {
            let uid = object::uid(p).unwrap_or("").to_owned();
            if object::is_deleting(p) {
                // Containers stopped: confirm with a forced delete.
                if p["metadata"]["deletionGracePeriodSeconds"]
                    .as_i64()
                    .unwrap_or(0)
                    > 0
                {
                    actions.push(Action::delete(pods, p, "Background", Some(0)));
                }
                continue;
            }
            if p["status"]["phase"] != "Pending" {
                continue;
            }
            let delay = Duration::from_millis(100 + self.env.rng.below(900));
            let at = *self
                .ready_at
                .entry(uid)
                .or_insert(MonoTime(now.0 + delay.as_nanos() as u64));
            if now >= at {
                let ip = if p["spec"]["hostNetwork"] == true {
                    Some(NODE_IP.to_owned())
                } else {
                    let (first, last) = (self.pod_cidr.first_host(), self.pod_cidr.last_host());
                    let gateway_skipped = Ipv4Addr::from(u32::from(first) + 1);
                    kubemu_net::allocate(gateway_skipped, last, &used, self.env.rng.as_ref()).map(
                        |ip| {
                            used.insert(ip);
                            ip.to_string()
                        },
                    )
                };
                match ip {
                    Some(ip) => {
                        let running = self.running(p, &ip);
                        actions.push(Action::update(pods, &running, Some("status")));
                    }
                    // The pod CIDR is full: the pod waits, as a real
                    // kubelet's sandbox would.
                    None => {
                        let retry = MonoTime(now.0 + Duration::from_secs(1).as_nanos() as u64);
                        wake = Some(wake.map_or(retry, |w| w.min(retry)));
                    }
                }
            } else {
                wake = Some(wake.map_or(at, |w| w.min(at)));
            }
        }
        actions.extend(wake.map(Action::RequeueAt));
        actions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Manager;
    use kubemu_api::{Api, HttpRequest};
    use kubemu_env_sim::SimEnv;

    #[test]
    fn registers_runs_and_finishes_pods() {
        let sim = SimEnv::new(3);
        let mut api = Api::new(sim.env.clone());
        api.add_cluster("c");
        let cidr: Cidr = "10.244.0.0/30".parse().unwrap();
        let mut m = Manager::new(vec![Box::new(SimKubelet::new(sim.env.clone(), "n1", cidr))]);
        m.settle(&mut api, "c", 5);
        assert_eq!(
            api.handle("c", &HttpRequest::new("GET", "/api/v1/nodes/n1"))
                .status(),
            200
        );

        let pods = "/api/v1/namespaces/default/pods";
        let pod = json!({"metadata": {"name": "p"}, "spec": {"nodeName": "n1", "containers": [
            {"name": "c", "image": "i", "resources": {"limits": {"cpu": "1", "memory": "1Gi"}}}
        ]}});
        api.handle("c", &HttpRequest::new("POST", pods).json(&pod));
        let r = m.step(&mut api, "c");
        let wake = r.next_wake.expect("a pending pod requeues");
        sim.clock
            .advance(Duration::from_nanos(wake.0 - sim.env.clock.now().0));
        m.settle(&mut api, "c", 5);
        let p = api
            .handle("c", &HttpRequest::new("GET", &format!("{pods}/p")))
            .response()
            .value()
            .unwrap();
        assert_eq!(p["status"]["phase"], "Running");
        // A /30 has one address left after the gateway.
        assert_eq!(p["status"]["podIP"], "10.244.0.2");
        assert_eq!(p["status"]["qosClass"], "Guaranteed");
        assert_eq!(p["status"]["containerStatuses"][0]["ready"], true);

        // The CIDR is full: a second pod waits for p's IP.
        let mut q = pod.clone();
        q["metadata"]["name"] = json!("q");
        api.handle("c", &HttpRequest::new("POST", pods).json(&q));
        let get = |api: &mut Api, n: &str| {
            api.handle("c", &HttpRequest::new("GET", &format!("{pods}/{n}")))
                .response()
        };
        for _ in 0..3 {
            sim.clock.advance(Duration::from_secs(1));
            m.settle(&mut api, "c", 5);
        }
        assert_eq!(
            get(&mut api, "q").value().unwrap()["status"]["phase"],
            "Pending"
        );

        api.handle("c", &HttpRequest::new("DELETE", &format!("{pods}/p")));
        assert_eq!(get(&mut api, "p").status, 200, "graceful");
        m.settle(&mut api, "c", 5);
        assert_eq!(get(&mut api, "p").status, 404);
        sim.clock.advance(Duration::from_secs(1));
        m.settle(&mut api, "c", 5);
        let q = get(&mut api, "q").value().unwrap();
        assert_eq!(q["status"]["phase"], "Running");
        assert_eq!(q["status"]["podIP"], "10.244.0.2", "p's IP is free again");
    }

    #[test]
    fn qos() {
        let p = |res: Value| json!({"spec": {"containers": [{"resources": res}]}});
        assert_eq!(qos_class(&p(json!({}))), "BestEffort");
        assert_eq!(
            qos_class(&p(json!({"requests": {"cpu": "1"}}))),
            "Burstable"
        );
        assert_eq!(
            qos_class(&p(json!({"limits": {"cpu": "1", "memory": "1"}}))),
            "Guaranteed"
        );
    }
}
