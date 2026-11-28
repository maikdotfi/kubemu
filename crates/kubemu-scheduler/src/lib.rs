//! The scheduler (README §2.2): trivial placement with no capacity
//! accounting (README §0: requests and limits are not enforced).
//!
//! A pod for `default-scheduler` without a node goes to a node that is
//! Ready, schedulable, matches its `nodeSelector` and whose `NoSchedule`
//! taints it tolerates; the least loaded wins, ties broken by the Env's
//! RNG. It is bound through `pods/binding`, as upstream does. A pod no
//! node fits gets `PodScheduled=False` with reason `Unschedulable`.
//!
//! Not yet: node affinity, pod (anti-)affinity, topology spread,
//! preemption.

use std::collections::BTreeMap;

use kubemu_api::object;
use kubemu_controllers::{Action, Controller, View, res};
use kubemu_env::{Env, MonoTime};
use serde_json::{Value, json};

pub const NAME: &str = "default-scheduler";

#[derive(Debug)]
pub struct Scheduler {
    env: Env,
}

impl Scheduler {
    pub fn new(env: Env) -> Self {
        Self { env }
    }
}

fn ready(node: &Value) -> bool {
    node["status"]["conditions"]
        .as_array()
        .into_iter()
        .flatten()
        .any(|c| c["type"] == "Ready" && c["status"] == "True")
}

fn tolerates(pod: &Value, taint: &Value) -> bool {
    pod["spec"]["tolerations"]
        .as_array()
        .into_iter()
        .flatten()
        .any(|t| {
            let key_ok = t["key"].as_str().unwrap_or("").is_empty() && t["operator"] == "Exists"
                || t["key"] == taint["key"];
            let op_ok = match t["operator"].as_str().unwrap_or("Equal") {
                "Exists" => true,
                _ => t["value"].as_str().unwrap_or("") == taint["value"].as_str().unwrap_or(""),
            };
            let effect_ok = t["effect"]
                .as_str()
                .is_none_or(|e| e.is_empty() || taint["effect"] == e);
            key_ok && op_ok && effect_ok
        })
}

/// Why `node` cannot take `pod`, if it cannot.
fn misfit(pod: &Value, node: &Value) -> Option<&'static str> {
    if !ready(node) {
        return Some("node(s) were not ready");
    }
    if node["spec"]["unschedulable"] == true {
        return Some("node(s) were unschedulable");
    }
    let labels = object::labels(node);
    if let Some(sel) = pod["spec"]["nodeSelector"].as_object()
        && !sel
            .iter()
            .all(|(k, v)| labels.and_then(|l| l.get(k)) == Some(v))
    {
        return Some("node(s) didn't match Pod's node affinity/selector");
    }
    let untolerated = node["spec"]["taints"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|t| matches!(t["effect"].as_str(), Some("NoSchedule" | "NoExecute")))
        .any(|t| !tolerates(pod, t));
    if untolerated {
        return Some("node(s) had untolerated taint(s)");
    }
    None
}

fn unschedulable_message(n: usize, reasons: &BTreeMap<&str, usize>) -> String {
    let parts: Vec<String> = reasons.iter().map(|(r, c)| format!("{c} {r}")).collect();
    format!("0/{n} nodes are available: {}.", parts.join(", "))
}

impl Controller for Scheduler {
    fn name(&self) -> &'static str {
        "scheduler"
    }

    fn reconcile(&mut self, view: View<'_>, _now: MonoTime) -> Vec<Action> {
        let pods = res("", "pods");
        let nodes = view.list(res("", "nodes"), None);
        let all_pods = view.list(pods, None);
        let mut load: BTreeMap<&str, usize> = BTreeMap::new();
        for p in &all_pods {
            if let Some(n) = p["spec"]["nodeName"].as_str().filter(|n| !n.is_empty()) {
                *load.entry(n).or_default() += 1;
            }
        }
        let mut actions = Vec::new();
        for pod in all_pods {
            let unbound = pod["spec"]["nodeName"].as_str().unwrap_or("").is_empty();
            let mine = pod["spec"]["schedulerName"].as_str().unwrap_or(NAME) == NAME;
            if !unbound || !mine || object::is_deleting(pod) {
                continue;
            }
            let mut reasons: BTreeMap<&str, usize> = BTreeMap::new();
            let mut fits: Vec<&str> = Vec::new();
            for n in &nodes {
                match misfit(pod, n) {
                    Some(r) => *reasons.entry(r).or_default() += 1,
                    None => fits.push(object::name(n)),
                }
            }
            let ns = object::namespace(pod);
            let name = object::name(pod);
            if fits.is_empty() {
                let msg = unschedulable_message(nodes.len(), &reasons);
                let cur = pod["status"]["conditions"]
                    .as_array()
                    .and_then(|cs| cs.iter().find(|c| c["type"] == "PodScheduled"));
                if cur.is_none_or(|c| c["message"] != msg.as_str()) {
                    let mut p = pod.clone();
                    let mut conds: Vec<Value> = p["status"]["conditions"]
                        .as_array()
                        .cloned()
                        .unwrap_or_default()
                        .into_iter()
                        .filter(|c| c["type"] != "PodScheduled")
                        .collect();
                    conds.push(json!({
                        "type": "PodScheduled",
                        "status": "False",
                        "reason": "Unschedulable",
                        "message": msg,
                        "lastProbeTime": null,
                        "lastTransitionTime": object::rfc3339_now(&self.env),
                    }));
                    p["status"]["conditions"] = json!(conds);
                    actions.push(Action::update(pods, &p, Some("status")));
                }
                continue;
            }
            let least = fits
                .iter()
                .map(|n| load.get(n).copied().unwrap_or(0))
                .min()
                .unwrap_or(0);
            let best: Vec<&str> = fits
                .into_iter()
                .filter(|n| load.get(n).copied().unwrap_or(0) == least)
                .collect();
            let node = best[self.env.rng.below(best.len() as u64) as usize];
            *load.entry(node).or_default() += 1;
            let binding = json!({
                "apiVersion": "v1",
                "kind": "Binding",
                "metadata": {"name": name, "namespace": ns},
                "target": {"apiVersion": "v1", "kind": "Node", "name": node},
            });
            actions.push(Action::post(pods, ns, name, "binding", &binding));
        }
        actions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kubemu_api::{Api, HttpRequest};
    use kubemu_controllers::Manager;
    use kubemu_controllers::kubelet::SimKubelet;
    use kubemu_env_sim::SimEnv;

    #[test]
    fn binds_to_fitting_nodes() {
        let sim = SimEnv::new(2);
        let mut api = Api::new(sim.env.clone());
        api.add_cluster("c");
        let mut m = Manager::new(vec![
            Box::new(SimKubelet::new(sim.env.clone(), "n1")),
            Box::new(Scheduler::new(sim.env.clone())),
        ]);
        m.settle(&mut api, "c", 3);
        let pods = "/api/v1/namespaces/default/pods";
        let post = |api: &mut Api, b: Value| {
            api.handle("c", &HttpRequest::new("POST", pods).json(&b))
                .status()
        };
        post(
            &mut api,
            json!({"metadata": {"name": "ok"}, "spec": {"containers": [{"name": "c", "image": "i"}]}}),
        );
        post(
            &mut api,
            json!({"metadata": {"name": "picky"}, "spec": {"nodeSelector": {"disk": "ssd"}, "containers": [{"name": "c", "image": "i"}]}}),
        );
        post(
            &mut api,
            json!({"metadata": {"name": "other"}, "spec": {"schedulerName": "custom", "containers": [{"name": "c", "image": "i"}]}}),
        );
        m.settle(&mut api, "c", 3);
        let get = |api: &mut Api, n: &str| {
            api.handle("c", &HttpRequest::new("GET", &format!("{pods}/{n}")))
                .response()
                .value()
                .unwrap()
        };
        assert_eq!(get(&mut api, "ok")["spec"]["nodeName"], "n1");
        let picky = get(&mut api, "picky");
        assert!(picky["spec"]["nodeName"].is_null());
        assert_eq!(picky["status"]["conditions"][0]["reason"], "Unschedulable");
        assert_eq!(
            picky["status"]["conditions"][0]["message"],
            "0/1 nodes are available: 1 node(s) didn't match Pod's node affinity/selector."
        );
        assert!(get(&mut api, "other")["spec"]["nodeName"].is_null());
        assert_eq!(m.step(&mut api, "c").requests, 0, "quiet once settled");
    }

    #[test]
    fn taints() {
        let t = json!({"key": "k", "value": "v", "effect": "NoSchedule"});
        let pod = |tol: Value| json!({"spec": {"tolerations": tol}});
        assert!(!tolerates(&pod(json!([])), &t));
        assert!(tolerates(
            &pod(json!([{"key": "k", "operator": "Equal", "value": "v"}])),
            &t
        ));
        assert!(tolerates(&pod(json!([{"operator": "Exists"}])), &t));
        assert!(!tolerates(&pod(json!([{"key": "k", "value": "w"}])), &t));
    }
}
