//! Built-in controllers (README §12.1 step 5), as ADR 0001 shapes them:
//! `reconcile(view, now) -> Vec<Action>`. A controller reads a consistent
//! [`View`] of one cluster and returns [`Action`]s: API requests, and when
//! to be woken again. It never writes itself. The [`Manager`] carries the
//! actions out through `kubemu-api`, so controller writes get the same
//! admission, validation and optimistic concurrency as any client's.
//!
//! Controllers are level-triggered and stateless where they can be: each
//! round compares the whole view with what should be and asks for the
//! difference. A failed request (a conflict, an object gone) is simply
//! retried by the next round. That is O(objects) per round, which is fine
//! at test scale; informers and work queues can replace the full scan
//! later without changing the shape.
//!
//! - [`gc::GarbageCollector`]: ownerReferences; background, foreground and
//!   orphan deletion.
//! - [`namespace::NamespaceController`]: empties terminating namespaces,
//!   then finalizes them.
//! - [`serviceaccount::ServiceAccountController`]: a `default`
//!   ServiceAccount in every namespace.
//! - [`endpoints::EndpointsController`]: Endpoints and EndpointSlices from
//!   Services and their ready pods.
//! - [`kubelet::SimKubelet`]: a logical node and a fake kubelet that runs
//!   pods assigned to it (Running/Ready after a seeded delay) and finishes
//!   their graceful deletion.
//!
//! The scheduler lives in `kubemu-scheduler`.

pub mod endpoints;
pub mod gc;
pub mod kubelet;
pub mod namespace;
pub mod serviceaccount;

use std::fmt;

use kubemu_api::object::{self, Obj, path};
use kubemu_api::{Api, HttpRequest};
use kubemu_api_types::{RESOURCES, ResourceInfo, resource};
use kubemu_env::MonoTime;
use kubemu_store::ClusterStore;
use serde_json::{Value, json};

/// What a controller asks the driver to do.
#[derive(Clone, Debug)]
pub enum Action {
    /// An API request against the cluster.
    Request(HttpRequest),
    /// Reconcile again at this time even if nothing changes.
    RequeueAt(MonoTime),
}

impl Action {
    pub fn create(r: &ResourceInfo, ns: &str, body: &Value) -> Self {
        Self::Request(HttpRequest::new("POST", &path(r, ns, None, None)).json(body))
    }

    /// PUT `obj` (with its resourceVersion as the precondition) to the
    /// object or one of its subresources.
    pub fn update(r: &ResourceInfo, obj: &Value, sub: Option<&str>) -> Self {
        let p = path(r, object::namespace(obj), Some(object::name(obj)), sub);
        Self::Request(HttpRequest::new("PUT", &p).json(obj))
    }

    pub fn post(r: &ResourceInfo, ns: &str, name: &str, sub: &str, body: &Value) -> Self {
        Self::Request(HttpRequest::new("POST", &path(r, ns, Some(name), Some(sub))).json(body))
    }

    /// DELETE with a UID precondition, so a recreated object of the same
    /// name is left alone.
    pub fn delete(r: &ResourceInfo, obj: &Value, propagation: &str, grace: Option<i64>) -> Self {
        let p = path(r, object::namespace(obj), Some(object::name(obj)), None);
        let mut opts = json!({"propagationPolicy": propagation});
        if let Some(uid) = object::uid(obj) {
            opts["preconditions"] = json!({"uid": uid});
        }
        if let Some(g) = grace {
            opts["gracePeriodSeconds"] = json!(g);
        }
        Self::Request(HttpRequest::new("DELETE", &p).json(&opts))
    }
}

/// A served resource by group and plural (the version kubemu serves).
pub fn res(group: &str, plural: &str) -> &'static ResourceInfo {
    RESOURCES
        .iter()
        .find(|r| r.group == group && r.plural == plural)
        .unwrap_or_else(|| panic!("{plural}.{group} is not served"))
}

/// Resources whose objects are stored (not virtual reviews).
pub fn stored() -> impl Iterator<Item = &'static ResourceInfo> {
    RESOURCES
        .iter()
        .filter(|r| r.group != "authorization.k8s.io")
}

/// A consistent, read-only view of one cluster.
#[derive(Clone, Copy)]
pub struct View<'a> {
    cs: &'a ClusterStore<Obj>,
}

impl fmt::Debug for View<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "View(rv={})", self.cs.resource_version())
    }
}

impl<'a> View<'a> {
    pub fn new(cs: &'a ClusterStore<Obj>) -> Self {
        Self { cs }
    }

    /// Objects of `r`, in one namespace or all, in key order.
    pub fn list(&self, r: &ResourceInfo, ns: Option<&str>) -> Vec<&'a Value> {
        self.cs
            .list(&object::storage_resource(r), ns)
            .0
            .into_iter()
            .map(|(_, o)| o.value.as_ref())
            .collect()
    }

    pub fn get(&self, r: &ResourceInfo, ns: &str, name: &str) -> Option<&'a Value> {
        self.cs
            .get(&object::key(r, ns, name))
            .map(|o| o.value.as_ref())
    }

    pub fn resource_version(&self) -> u64 {
        self.cs.resource_version()
    }
}

/// A controller: reads a view, returns actions.
pub trait Controller: Send + fmt::Debug {
    fn name(&self) -> &'static str;
    fn reconcile(&mut self, view: View<'_>, now: MonoTime) -> Vec<Action>;
}

/// The controllers of `kubemu up` minus the scheduler, which lives in
/// `kubemu-scheduler`.
pub fn standard(env: &kubemu_env::Env, cluster: &str) -> Vec<Box<dyn Controller>> {
    vec![
        Box::new(namespace::NamespaceController),
        Box::new(serviceaccount::ServiceAccountController),
        Box::new(gc::GarbageCollector),
        Box::new(kubelet::SimKubelet::new(
            env.clone(),
            &format!("{cluster}-node"),
        )),
        Box::new(endpoints::EndpointsController),
    ]
}

/// What one round did.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Round {
    /// Requests sent (successful or not).
    pub requests: usize,
    /// Requests that changed something (2xx other than no-ops are not
    /// told apart; non-2xx are excluded).
    pub succeeded: usize,
    /// The earliest requeue any controller asked for.
    pub next_wake: Option<MonoTime>,
}

/// Runs controllers against one cluster and carries out their actions.
#[derive(Debug)]
pub struct Manager {
    controllers: Vec<Box<dyn Controller>>,
    wake: Vec<Option<MonoTime>>,
}

impl Manager {
    pub fn new(controllers: Vec<Box<dyn Controller>>) -> Self {
        let wake = vec![None; controllers.len()];
        Self { controllers, wake }
    }

    pub fn len(&self) -> usize {
        self.controllers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.controllers.is_empty()
    }

    pub fn name(&self, i: usize) -> &'static str {
        self.controllers[i].name()
    }

    /// Runs controller `i` once and applies its actions.
    pub fn step_one(&mut self, i: usize, api: &mut Api, cluster: &str) -> Round {
        let now = api.env().clock.now();
        let actions = match api.store().cluster(cluster) {
            Ok(cs) => self.controllers[i].reconcile(View::new(cs), now),
            Err(_) => return Round::default(),
        };
        let mut round = Round::default();
        self.wake[i] = None;
        for a in actions {
            match a {
                Action::Request(req) => {
                    round.requests += 1;
                    let status = api.handle(cluster, &req).status();
                    if (200..300).contains(&status) {
                        round.succeeded += 1;
                    }
                }
                Action::RequeueAt(t) => {
                    self.wake[i] = Some(self.wake[i].map_or(t, |w| w.min(t)));
                }
            }
        }
        round.next_wake = self.next_wake();
        round
    }

    /// Runs every controller once, in order.
    pub fn step(&mut self, api: &mut Api, cluster: &str) -> Round {
        let mut total = Round::default();
        for i in 0..self.controllers.len() {
            let r = self.step_one(i, api, cluster);
            total.requests += r.requests;
            total.succeeded += r.succeeded;
        }
        total.next_wake = self.next_wake();
        total
    }

    /// Rounds until one changes nothing (or `max_rounds`). Returns the
    /// number of successful requests.
    pub fn settle(&mut self, api: &mut Api, cluster: &str, max_rounds: usize) -> usize {
        let mut n = 0;
        for _ in 0..max_rounds {
            let r = self.step(api, cluster);
            n += r.succeeded;
            if r.succeeded == 0 {
                break;
            }
        }
        n
    }

    pub fn next_wake(&self) -> Option<MonoTime> {
        self.wake.iter().flatten().min().copied()
    }
}

/// Labels as a map, for selector matching.
pub(crate) fn labels(v: &Value) -> &serde_json::Map<String, Value> {
    static EMPTY: std::sync::OnceLock<serde_json::Map<String, Value>> = std::sync::OnceLock::new();
    object::labels(v)
        .and_then(Value::as_object)
        .unwrap_or_else(|| EMPTY.get_or_init(serde_json::Map::new))
}

/// A map selector (`spec.selector` of a Service): every pair must match.
pub(crate) fn selects(selector: &serde_json::Map<String, Value>, v: &Value) -> bool {
    let l = labels(v);
    selector.iter().all(|(k, want)| l.get(k) == Some(want))
}

/// Namespaces that exist and are not terminating.
pub(crate) fn active_namespaces<'a>(view: View<'a>) -> Vec<&'a Value> {
    view.list(resource("", "v1", "namespaces").expect("served"), None)
        .into_iter()
        .filter(|ns| !object::is_deleting(ns))
        .collect()
}
