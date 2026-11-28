//! The garbage collector: ownerReferences across every stored resource.
//!
//! - **Background**: an object none of whose owners exist any more is
//!   deleted. References to missing owners are dropped while another
//!   owner remains.
//! - **Foreground** (`foregroundDeletion` finalizer on a deleting owner):
//!   its dependents are deleted (in the foreground too); once none
//!   remain, the finalizer is removed and the owner goes.
//! - **Orphan** (`orphan` finalizer): references to the owner are removed
//!   from its dependents, then the finalizer.
//!
//! An owner is identified by UID, as upstream does; a namesake with
//! another UID is not the owner.

use std::collections::{BTreeMap, BTreeSet};

use kubemu_api::object::{self, owner_refs};
use kubemu_api_types::ResourceInfo;
use kubemu_env::MonoTime;
use serde_json::{Value, json};

use crate::{Action, Controller, View, stored};

pub const FOREGROUND: &str = "foregroundDeletion";
pub const ORPHAN: &str = "orphan";

#[derive(Debug, Default)]
pub struct GarbageCollector;

fn ref_uid(r: &Value) -> &str {
    r["uid"].as_str().unwrap_or("")
}

fn without_finalizer(obj: &Value, f: &str) -> Value {
    let mut o = obj.clone();
    let fins: Vec<String> = object::finalizers(obj)
        .into_iter()
        .filter(|x| x != f)
        .collect();
    o["metadata"]["finalizers"] = json!(fins);
    o
}

fn with_owner_refs(obj: &Value, refs: Vec<Value>) -> Value {
    let mut o = obj.clone();
    let m = object::meta_mut(&mut o);
    if refs.is_empty() {
        m.remove("ownerReferences");
    } else {
        m.insert("ownerReferences".into(), Value::Array(refs));
    }
    o
}

impl Controller for GarbageCollector {
    fn name(&self) -> &'static str {
        "garbage-collector"
    }

    fn reconcile(&mut self, view: View<'_>, _now: MonoTime) -> Vec<Action> {
        let all: Vec<(&'static ResourceInfo, &Value)> = stored()
            .flat_map(|r| view.list(r, None).into_iter().map(move |v| (r, v)))
            .collect();
        let by_uid: BTreeMap<&str, &Value> = all
            .iter()
            .filter_map(|(_, v)| object::uid(v).map(|u| (u, *v)))
            .collect();
        // owner UID -> dependents
        let mut dependents: BTreeMap<&str, Vec<(&'static ResourceInfo, &Value)>> = BTreeMap::new();
        for (r, v) in &all {
            for o in owner_refs(v) {
                dependents.entry(ref_uid(o)).or_default().push((r, v));
            }
        }

        let mut actions = Vec::new();
        let mut touched: BTreeSet<&str> = BTreeSet::new();

        // Owners being deleted with foreground or orphan propagation.
        for (r, owner) in &all {
            if !object::is_deleting(owner) {
                continue;
            }
            let fins = object::finalizers(owner);
            let uid = object::uid(owner).unwrap_or("");
            let deps = dependents.get(uid).map_or(&[][..], Vec::as_slice);
            if fins.iter().any(|f| f == ORPHAN) {
                for (dr, d) in deps {
                    let keep: Vec<Value> = owner_refs(d)
                        .iter()
                        .filter(|o| ref_uid(o) != uid)
                        .cloned()
                        .collect();
                    actions.push(Action::update(dr, &with_owner_refs(d, keep), None));
                    touched.extend(object::uid(d));
                }
                if deps.is_empty() {
                    actions.push(Action::update(r, &without_finalizer(owner, ORPHAN), None));
                }
            } else if fins.iter().any(|f| f == FOREGROUND) {
                for (dr, d) in deps {
                    if !object::is_deleting(d) {
                        actions.push(Action::delete(dr, d, "Foreground", None));
                    }
                    touched.extend(object::uid(d));
                }
                if deps.is_empty() {
                    actions.push(Action::update(
                        r,
                        &without_finalizer(owner, FOREGROUND),
                        None,
                    ));
                }
            }
        }

        // Dependents whose owners are gone.
        for (r, v) in &all {
            let refs = owner_refs(v);
            if refs.is_empty() || object::uid(v).is_some_and(|u| touched.contains(u)) {
                continue;
            }
            let live: Vec<Value> = refs
                .iter()
                .filter(|o| by_uid.contains_key(ref_uid(o)))
                .cloned()
                .collect();
            if live.is_empty() {
                if !object::is_deleting(v) {
                    actions.push(Action::delete(r, v, "Background", None));
                }
            } else if live.len() < refs.len() {
                actions.push(Action::update(r, &with_owner_refs(v, live), None));
            }
        }
        actions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Manager, res};
    use kubemu_api::{Api, HttpRequest};
    use kubemu_env_sim::SimEnv;

    const CMS: &str = "/api/v1/namespaces/default/configmaps";

    fn setup() -> (Api, Manager) {
        let mut api = Api::new(SimEnv::new(1).env);
        api.add_cluster("c");
        (api, Manager::new(vec![Box::new(GarbageCollector)]))
    }

    fn create(api: &mut Api, name: &str, owners: &[&Value]) -> Value {
        let refs: Vec<Value> = owners
            .iter()
            .map(|o| json!({"apiVersion": "v1", "kind": "ConfigMap", "name": object::name(o), "uid": object::uid(o)}))
            .collect();
        let body = json!({"metadata": {"name": name, "ownerReferences": refs}});
        let r = api
            .handle("c", &HttpRequest::new("POST", CMS).json(&body))
            .response();
        assert_eq!(r.status, 201);
        r.value().unwrap()
    }

    fn exists(api: &mut Api, name: &str) -> bool {
        api.handle("c", &HttpRequest::new("GET", &format!("{CMS}/{name}")))
            .status()
            == 200
    }

    fn delete(api: &mut Api, name: &str, policy: &str) {
        let req = HttpRequest::new("DELETE", &format!("{CMS}/{name}"))
            .json(&json!({"propagationPolicy": policy}));
        assert_eq!(api.handle("c", &req).status(), 200);
    }

    #[test]
    fn background() {
        let (mut api, mut m) = setup();
        let a = create(&mut api, "a", &[]);
        let b = create(&mut api, "b", &[]);
        create(&mut api, "child", &[&a]);
        create(&mut api, "shared", &[&a, &b]);
        let mid = create(&mut api, "mid", &[&a]);
        create(&mut api, "grandchild", &[&mid]);
        delete(&mut api, "a", "Background");
        m.settle(&mut api, "c", 10);
        assert!(
            !exists(&mut api, "child")
                && !exists(&mut api, "mid")
                && !exists(&mut api, "grandchild")
        );
        assert!(exists(&mut api, "shared"), "another owner remains");
        let shared = api
            .store()
            .cluster("c")
            .unwrap()
            .get(&object::key(res("", "configmaps"), "default", "shared"))
            .unwrap()
            .value
            .clone();
        assert_eq!(
            owner_refs(&shared).len(),
            1,
            "the dangling reference is dropped"
        );
    }

    #[test]
    fn foreground_waits_for_dependents() {
        let (mut api, mut m) = setup();
        let a = create(&mut api, "a", &[]);
        let c = create(&mut api, "child", &[&a]);
        let mut held = create(&mut api, "held", &[&c]);
        held["metadata"]["finalizers"] = json!(["x.io/hold"]);
        api.handle(
            "c",
            &HttpRequest::new("PUT", &format!("{CMS}/held")).json(&held),
        );
        delete(&mut api, "a", "Foreground");
        m.settle(&mut api, "c", 10);
        assert!(
            exists(&mut api, "a") && exists(&mut api, "child"),
            "blocked by held"
        );
        let mut held = api
            .handle("c", &HttpRequest::new("GET", &format!("{CMS}/held")))
            .response()
            .value()
            .unwrap();
        held["metadata"]["finalizers"] = json!([]);
        api.handle(
            "c",
            &HttpRequest::new("PUT", &format!("{CMS}/held")).json(&held),
        );
        m.settle(&mut api, "c", 10);
        assert!(!exists(&mut api, "a") && !exists(&mut api, "child") && !exists(&mut api, "held"));
    }

    #[test]
    fn orphan_keeps_dependents() {
        let (mut api, mut m) = setup();
        let a = create(&mut api, "a", &[]);
        create(&mut api, "child", &[&a]);
        delete(&mut api, "a", "Orphan");
        m.settle(&mut api, "c", 10);
        assert!(!exists(&mut api, "a"));
        assert!(exists(&mut api, "child"));
    }
}
