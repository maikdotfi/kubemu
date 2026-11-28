//! Namespace lifecycle: a terminating namespace has its content deleted;
//! once it is empty, the `kubernetes` entry of `spec.finalizers` is
//! removed through `/finalize`, and the namespace goes.

use kubemu_api::object;
use kubemu_env::MonoTime;
use serde_json::{Value, json};

use crate::{Action, Controller, View, res, stored};

#[derive(Debug, Default)]
pub struct NamespaceController;

impl Controller for NamespaceController {
    fn name(&self) -> &'static str {
        "namespace"
    }

    fn reconcile(&mut self, view: View<'_>, _now: MonoTime) -> Vec<Action> {
        let nsr = res("", "namespaces");
        let mut actions = Vec::new();
        for ns in view.list(nsr, None) {
            if !object::is_deleting(ns) {
                continue;
            }
            let name = object::name(ns);
            let mut empty = true;
            for r in stored().filter(|r| r.namespaced) {
                for v in view.list(r, Some(name)) {
                    empty = false;
                    if !object::is_deleting(v) {
                        actions.push(Action::delete(r, v, "Background", None));
                    }
                }
            }
            let fins: Vec<Value> = ns["spec"]["finalizers"]
                .as_array()
                .cloned()
                .unwrap_or_default();
            if empty && fins.iter().any(|f| f == "kubernetes") {
                let mut n = ns.clone();
                n["spec"]["finalizers"] = json!(
                    fins.into_iter()
                        .filter(|f| f != "kubernetes")
                        .collect::<Vec<_>>()
                );
                actions.push(Action::update(nsr, &n, Some("finalize")));
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

    #[test]
    fn terminating_namespaces_empty_and_go() {
        let mut api = Api::new(SimEnv::new(1).env);
        api.add_cluster("c");
        let mut m = Manager::new(vec![Box::new(NamespaceController)]);
        let post = |api: &mut Api, p: &str, b: Value| {
            api.handle("c", &HttpRequest::new("POST", p).json(&b))
                .status()
        };
        assert_eq!(
            post(
                &mut api,
                "/api/v1/namespaces",
                json!({"metadata": {"name": "t"}})
            ),
            201
        );
        assert_eq!(
            post(
                &mut api,
                "/api/v1/namespaces/t/configmaps",
                json!({"metadata": {"name": "a"}})
            ),
            201
        );
        assert_eq!(
            post(
                &mut api,
                "/api/v1/namespaces/t/secrets",
                json!({"metadata": {"name": "s"}})
            ),
            201
        );
        api.handle("c", &HttpRequest::new("DELETE", "/api/v1/namespaces/t"));
        m.settle(&mut api, "c", 10);
        assert_eq!(
            api.handle("c", &HttpRequest::new("GET", "/api/v1/namespaces/t"))
                .status(),
            404
        );
        assert_eq!(
            api.handle(
                "c",
                &HttpRequest::new("GET", "/api/v1/namespaces/t/configmaps/a")
            )
            .status(),
            404
        );
    }
}
