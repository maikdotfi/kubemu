//! A `default` ServiceAccount in every active namespace.

use kubemu_env::MonoTime;
use serde_json::json;

use crate::{Action, Controller, View, active_namespaces, res};

#[derive(Debug, Default)]
pub struct ServiceAccountController;

impl Controller for ServiceAccountController {
    fn name(&self) -> &'static str {
        "serviceaccount"
    }

    fn reconcile(&mut self, view: View<'_>, _now: MonoTime) -> Vec<Action> {
        let sa = res("", "serviceaccounts");
        active_namespaces(view)
            .into_iter()
            .map(kubemu_api::object::name)
            .filter(|ns| view.get(sa, ns, "default").is_none())
            .map(|ns| Action::create(sa, ns, &json!({"metadata": {"name": "default"}})))
            .collect()
    }
}
