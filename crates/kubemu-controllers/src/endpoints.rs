//! Endpoints and EndpointSlices for Services with a selector, from the
//! pods it selects (upstream `endpoint` and `endpointslice` controllers).
//!
//! - Pods count once they have an IP and are not finished. Ready pods are
//!   `addresses`, others `notReadyAddresses`; terminating pods are left
//!   out of Endpoints but listed in slices with `terminating: true`.
//! - Named target ports are resolved per pod, so pods are grouped by the
//!   ports they resolve to: one Endpoints subset and one slice per group.
//! - Writes happen only when something differs, so a quiet cluster stays
//!   quiet.
//!
//! Endpoints carry no owner and are deleted here when their Service goes;
//! slices are owned by their Service and left to the garbage collector.

use std::collections::BTreeMap;

use kubemu_api::object;
use kubemu_env::MonoTime;
use serde_json::{Map, Value, json};

use crate::{Action, Controller, View, res, selects};

pub const SERVICE_NAME_LABEL: &str = "kubernetes.io/service-name";
pub const SLICE_MANAGED_BY: &str = "endpointslice.kubernetes.io/managed-by";
pub const SLICE_CONTROLLER: &str = "endpointslice-controller.k8s.io";
pub const ENDPOINTS_MANAGED_BY: &str = "endpoints.kubernetes.io/managed-by";
pub const ENDPOINTS_CONTROLLER: &str = "endpoint-controller";

#[derive(Debug, Default)]
pub struct EndpointsController;

/// A service port resolved for one pod: (name, port, protocol).
type Port = (String, i64, String);

struct Member<'a> {
    pod: &'a Value,
    ip: String,
    ready: bool,
    terminating: bool,
}

fn is_ready(pod: &Value) -> bool {
    pod["status"]["conditions"]
        .as_array()
        .into_iter()
        .flatten()
        .any(|c| c["type"] == "Ready" && c["status"] == "True")
}

/// The pod's ports for the service's ports; named target ports the pod
/// does not have are skipped.
fn resolve(svc: &Value, pod: &Value) -> Vec<Port> {
    let mut out = Vec::new();
    for sp in svc["spec"]["ports"].as_array().into_iter().flatten() {
        let name = sp["name"].as_str().unwrap_or("").to_owned();
        let proto = sp["protocol"].as_str().unwrap_or("TCP").to_owned();
        let port = match &sp["targetPort"] {
            Value::Number(n) => n.as_i64(),
            Value::String(s) => match s.parse::<i64>() {
                Ok(n) => Some(n),
                Err(_) => pod["spec"]["containers"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .flat_map(|c| c["ports"].as_array().into_iter().flatten())
                    .find(|p| {
                        p["name"] == s.as_str() && p["protocol"].as_str().unwrap_or("TCP") == proto
                    })
                    .and_then(|p| p["containerPort"].as_i64()),
            },
            _ => sp["port"].as_i64(),
        };
        if let Some(port) = port {
            out.push((name, port, proto));
        }
    }
    out
}

fn target_ref(pod: &Value) -> Value {
    json!({
        "kind": "Pod",
        "namespace": object::namespace(pod),
        "name": object::name(pod),
        "uid": object::uid(pod),
    })
}

fn node_name(pod: &Value) -> Option<&str> {
    pod["spec"]["nodeName"].as_str().filter(|n| !n.is_empty())
}

fn service_labels(svc: &Value, extra: &[(&str, &str)]) -> Map<String, Value> {
    let mut l = crate::labels(svc).clone();
    if svc["spec"]["clusterIP"] == "None" {
        l.insert("service.kubernetes.io/headless".into(), json!(""));
    }
    for (k, v) in extra {
        l.insert((*k).into(), json!(v));
    }
    l
}

fn subsets(groups: &BTreeMap<Vec<Port>, Vec<Member<'_>>>) -> Vec<Value> {
    let mut out = Vec::new();
    for (ports, members) in groups {
        let addr = |m: &Member<'_>| {
            let mut a = json!({"ip": m.ip, "targetRef": target_ref(m.pod)});
            if let Some(n) = node_name(m.pod) {
                a["nodeName"] = json!(n);
            }
            a
        };
        let mut ready: Vec<&Member<'_>> = members
            .iter()
            .filter(|m| m.ready && !m.terminating)
            .collect();
        let mut not_ready: Vec<&Member<'_>> = members
            .iter()
            .filter(|m| !m.ready && !m.terminating)
            .collect();
        if ready.is_empty() && not_ready.is_empty() {
            continue;
        }
        ready.sort_by(|a, b| a.ip.cmp(&b.ip));
        not_ready.sort_by(|a, b| a.ip.cmp(&b.ip));
        let mut s = json!({
            "ports": ports.iter().map(|(n, p, proto)| {
                let mut e = json!({"port": p, "protocol": proto});
                if !n.is_empty() {
                    e["name"] = json!(n);
                }
                e
            }).collect::<Vec<_>>(),
        });
        if !ready.is_empty() {
            s["addresses"] = ready.into_iter().map(addr).collect();
        }
        if !not_ready.is_empty() {
            s["notReadyAddresses"] = not_ready.into_iter().map(addr).collect();
        }
        out.push(s);
    }
    out
}

fn slice_body(ports: &[Port], members: &[Member<'_>]) -> Value {
    let mut eps: Vec<Value> = members
        .iter()
        .map(|m| {
            let mut e = json!({
                "addresses": [m.ip],
                "conditions": {
                    "ready": m.ready && !m.terminating,
                    "serving": m.ready,
                    "terminating": m.terminating,
                },
                "targetRef": target_ref(m.pod),
            });
            if let Some(n) = node_name(m.pod) {
                e["nodeName"] = json!(n);
            }
            e
        })
        .collect();
    eps.sort_by(|a, b| a["addresses"][0].as_str().cmp(&b["addresses"][0].as_str()));
    let mut s = json!({
        "addressType": "IPv4",
        "ports": ports.iter().map(|(n, p, proto)| json!({"name": n, "port": p, "protocol": proto})).collect::<Vec<_>>(),
    });
    if !eps.is_empty() {
        s["endpoints"] = json!(eps);
    }
    s
}

impl Controller for EndpointsController {
    fn name(&self) -> &'static str {
        "endpoints"
    }

    fn reconcile(&mut self, view: View<'_>, _now: MonoTime) -> Vec<Action> {
        let svcr = res("", "services");
        let epr = res("", "endpoints");
        let slr = res("discovery.k8s.io", "endpointslices");
        let podr = res("", "pods");
        let mut actions = Vec::new();

        for svc in view.list(svcr, None) {
            let ns = object::namespace(svc);
            let name = object::name(svc);
            let Some(selector) = svc["spec"]["selector"]
                .as_object()
                .filter(|s| !s.is_empty())
            else {
                continue;
            };
            if object::is_deleting(svc) {
                continue;
            }
            let mut groups: BTreeMap<Vec<Port>, Vec<Member<'_>>> = BTreeMap::new();
            for pod in view.list(podr, Some(ns)) {
                let ip = pod["status"]["podIP"].as_str().unwrap_or("");
                let phase = pod["status"]["phase"].as_str().unwrap_or("");
                if ip.is_empty()
                    || matches!(phase, "Succeeded" | "Failed")
                    || !selects(selector, pod)
                {
                    continue;
                }
                groups.entry(resolve(svc, pod)).or_default().push(Member {
                    pod,
                    ip: ip.to_owned(),
                    ready: is_ready(pod),
                    terminating: object::is_deleting(pod),
                });
            }

            // Endpoints.
            let labels = service_labels(svc, &[(ENDPOINTS_MANAGED_BY, ENDPOINTS_CONTROLLER)]);
            let subs = subsets(&groups);
            match view.get(epr, ns, name) {
                None => {
                    let mut e = json!({"metadata": {"name": name, "labels": labels}});
                    if !subs.is_empty() {
                        e["subsets"] = json!(subs);
                    }
                    actions.push(Action::create(epr, ns, &e));
                }
                Some(cur) => {
                    let cur_subs = cur.get("subsets").cloned().unwrap_or(json!([]));
                    if cur_subs != json!(subs) || crate::labels(cur) != &labels {
                        let mut e = cur.clone();
                        e["metadata"]["labels"] = json!(labels);
                        match subs.is_empty() {
                            true => {
                                if let Some(m) = e.as_object_mut() {
                                    m.remove("subsets");
                                }
                            }
                            false => e["subsets"] = json!(subs),
                        }
                        actions.push(Action::update(epr, &e, None));
                    }
                }
            }

            // EndpointSlices: one per port group, or one empty placeholder.
            let slice_labels = service_labels(
                svc,
                &[
                    (SERVICE_NAME_LABEL, name),
                    (SLICE_MANAGED_BY, SLICE_CONTROLLER),
                ],
            );
            let owner = json!([{
                "apiVersion": "v1",
                "kind": "Service",
                "name": name,
                "uid": object::uid(svc),
                "controller": true,
                "blockOwnerDeletion": true,
            }]);
            let mut desired: Vec<Value> = groups.iter().map(|(p, m)| slice_body(p, m)).collect();
            if desired.is_empty() {
                desired.push(slice_body(&[], &[]));
            }
            let existing: Vec<&Value> = view
                .list(slr, Some(ns))
                .into_iter()
                .filter(|s| {
                    let l = crate::labels(s);
                    l.get(SERVICE_NAME_LABEL) == Some(&json!(name))
                        && l.get(SLICE_MANAGED_BY) == Some(&json!(SLICE_CONTROLLER))
                })
                .filter(|s| !object::is_deleting(s))
                .collect();
            for (i, want) in desired.iter().enumerate() {
                match existing.get(i) {
                    Some(cur) => {
                        let same = ["addressType", "endpoints", "ports"]
                            .iter()
                            .all(|k| cur.get(*k) == want.get(*k))
                            && crate::labels(cur) == &slice_labels
                            && cur["metadata"]["ownerReferences"] == owner;
                        if !same {
                            let mut s = (*cur).clone();
                            for k in ["addressType", "endpoints", "ports"] {
                                match want.get(k) {
                                    Some(v) => s[k] = v.clone(),
                                    None => {
                                        if let Some(m) = s.as_object_mut() {
                                            m.remove(k);
                                        }
                                    }
                                }
                            }
                            s["metadata"]["labels"] = json!(slice_labels);
                            s["metadata"]["ownerReferences"] = owner.clone();
                            actions.push(Action::update(slr, &s, None));
                        }
                    }
                    None => {
                        let mut s = want.clone();
                        s["metadata"] = json!({
                            "generateName": format!("{name}-"),
                            "labels": slice_labels,
                            "ownerReferences": owner,
                        });
                        actions.push(Action::create(slr, ns, &s));
                    }
                }
            }
            for extra in existing.iter().skip(desired.len()) {
                actions.push(Action::delete(slr, extra, "Background", None));
            }
        }

        // Endpoints we manage whose Service is gone or has no selector.
        for ep in view.list(epr, None) {
            if crate::labels(ep).get(ENDPOINTS_MANAGED_BY) != Some(&json!(ENDPOINTS_CONTROLLER))
                || object::is_deleting(ep)
            {
                continue;
            }
            let svc = view.get(svcr, object::namespace(ep), object::name(ep));
            let selected = svc.is_some_and(|s| {
                s["spec"]["selector"]
                    .as_object()
                    .is_some_and(|m| !m.is_empty())
            });
            if !selected {
                actions.push(Action::delete(epr, ep, "Background", None));
            }
        }
        actions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Manager;
    use kubemu_api::{Api, HttpRequest};
    use kubemu_env_sim::SimEnv;

    fn post(api: &mut Api, p: &str, b: Value) -> u16 {
        api.handle("c", &HttpRequest::new("POST", p).json(&b))
            .status()
    }

    fn get(api: &mut Api, p: &str) -> Value {
        api.handle("c", &HttpRequest::new("GET", p))
            .response()
            .value()
            .unwrap()
    }

    fn running_pod(api: &mut Api, name: &str, ip: &str, ready: bool, app: &str) {
        let pods = "/api/v1/namespaces/default/pods";
        post(
            api,
            pods,
            json!({"metadata": {"name": name, "labels": {"app": app}}, "spec": {"containers": [
                {"name": "c", "image": "i", "ports": [{"name": "http", "containerPort": 8080}]}
            ]}}),
        );
        let mut p = get(api, &format!("{pods}/{name}"));
        p["status"] = json!({"phase": "Running", "podIP": ip, "conditions": [{"type": "Ready", "status": if ready { "True" } else { "False" }}]});
        assert_eq!(
            api.handle(
                "c",
                &HttpRequest::new("PUT", &format!("{pods}/{name}/status")).json(&p)
            )
            .status(),
            200
        );
    }

    #[test]
    fn endpoints_follow_pods() {
        let mut api = Api::new(SimEnv::new(1).env);
        api.add_cluster("c");
        let mut m = Manager::new(vec![Box::new(EndpointsController)]);
        post(
            &mut api,
            "/api/v1/namespaces/default/services",
            json!({"metadata": {"name": "web"}, "spec": {
                "selector": {"app": "web"}, "ports": [{"name": "http", "port": 80, "targetPort": "http", "protocol": "TCP"}]
            }}),
        );
        running_pod(&mut api, "b", "10.0.0.2", true, "web");
        running_pod(&mut api, "a", "10.0.0.1", false, "web");
        running_pod(&mut api, "other", "10.0.0.9", true, "db");
        m.settle(&mut api, "c", 5);

        let ep = get(&mut api, "/api/v1/namespaces/default/endpoints/web");
        assert_eq!(ep["subsets"][0]["addresses"][0]["ip"], "10.0.0.2");
        assert_eq!(ep["subsets"][0]["notReadyAddresses"][0]["ip"], "10.0.0.1");
        assert_eq!(
            ep["subsets"][0]["ports"][0],
            json!({"name": "http", "port": 8080, "protocol": "TCP"})
        );
        let slices = get(
            &mut api,
            "/apis/discovery.k8s.io/v1/namespaces/default/endpointslices?labelSelector=kubernetes.io/service-name=web",
        );
        let s = &slices["items"][0];
        assert!(s["metadata"]["name"].as_str().unwrap().starts_with("web-"));
        assert_eq!(s["endpoints"].as_array().unwrap().len(), 2);
        assert_eq!(s["endpoints"][0]["conditions"]["ready"], false);
        assert_eq!(s["metadata"]["ownerReferences"][0]["kind"], "Service");

        // Quiet once in sync.
        assert_eq!(m.step(&mut api, "c").requests, 0);

        api.handle(
            "c",
            &HttpRequest::new("DELETE", "/api/v1/namespaces/default/services/web"),
        );
        m.settle(&mut api, "c", 5);
        assert_eq!(
            api.handle(
                "c",
                &HttpRequest::new("GET", "/api/v1/namespaces/default/endpoints/web")
            )
            .status(),
            404
        );
    }
}
