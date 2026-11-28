//! The synchronous API frontend (ADR 0011): everything the apiserver does
//! between parsing HTTP and touching storage, as a plain state machine.
//!
//! [`Api::handle`] takes an [`HttpRequest`] for a cluster and returns a
//! [`Reply`]: a full response, or a [`WatchCursor`] the caller drains with
//! [`Api::watch_poll`]. The tokio shell (`kubemu-apiserver`) and the DST
//! harness (`testing/sim`) both drive it this way; nothing here knows
//! about sockets, threads or real time. Time and randomness (timestamps,
//! UIDs, generated names, watch timeouts) come from [`Env`].
//!
//! Every handled request is recorded in the event trace ([`Api::take_trace`]).
//!
//! Served: discovery (legacy and aggregated v2), OpenAPI v2/v3, `/version`,
//! and get/list/watch/create/update/patch/delete/deletecollection on the
//! resources in `kubemu-api-types` (T0), with `/status`, label and field
//! selectors, resourceVersion semantics, pagination, finalizers,
//! graceful pod deletion and namespace lifecycle. Not yet: per-resource
//! defaulting and validation (ADR 0013, §12.1 step 7), server-side apply
//! (M3), Table output, protobuf (ADR 0010: never).

mod decode;
mod discovery;
mod http;
mod meta;
pub mod object;
mod read;
mod selector;
mod status;
mod strategy;
mod watch;
mod write;

use std::collections::VecDeque;
use std::time::Duration;

use kubemu_api_types::{RESOURCES, ResourceInfo, resource};
use kubemu_env::{Env, MonoTime};
use kubemu_store::{ClusterStore, ResourceVersion, Store};
use serde_json::{Value, json};

pub use decode::FieldValidation;
pub use http::{Body, HttpRequest, HttpResponse, Query, Reply};
pub use object::Obj;
pub use status::{ApiError, ApiResult, Cause};
pub use watch::{WatchBatch, WatchCursor};

use http::{Target, accepts_json, parse_path, wants_aggregated_discovery};
use object::{as_list_item, key, storage_resource};
use read::{Filter, parse_rv};
use status::Subject;

/// Watch event log retained per cluster.
pub const LOG_CAPACITY: usize = 10_000;

/// The frontend over every cluster in one store.
#[derive(Debug)]
pub struct Api {
    env: Env,
    store: Store<Obj>,
    log_capacity: usize,
    trace: Vec<String>,
}

/// One request, resolved against the resource table.
pub(crate) struct Ctx<'a> {
    pub cluster: &'a str,
    pub r: &'static ResourceInfo,
    pub namespace: Option<String>,
    pub name: Option<String>,
    pub subresource: Option<String>,
    pub query: Query,
}

impl Ctx<'_> {
    pub fn subject<'s>(&'s self, name: &'s str) -> Subject<'s> {
        Subject {
            group: self.r.group,
            resource: self.r.plural,
            name,
        }
    }

    pub fn ns(&self) -> &str {
        self.namespace.as_deref().unwrap_or("")
    }

    pub fn dry_run(&self) -> ApiResult<bool> {
        match self.query.all("dryRun") {
            [] => Ok(false),
            v if v.iter().all(|x| x == "All") => Ok(true),
            v => Err(ApiError::bad_request(format!(
                "dryRun: Unsupported value: {:?}: supported values: \"All\"",
                v.join(",")
            ))),
        }
    }
}

impl Api {
    pub fn new(env: Env) -> Self {
        Self::with_log_capacity(env, LOG_CAPACITY)
    }

    pub fn with_log_capacity(env: Env, log_capacity: usize) -> Self {
        Self {
            env,
            store: Store::new(),
            log_capacity,
            trace: Vec::new(),
        }
    }

    pub fn env(&self) -> &Env {
        &self.env
    }

    pub fn store(&self) -> &Store<Obj> {
        &self.store
    }

    /// Creates an empty cluster with the namespaces the apiserver creates
    /// at startup. Returns `false` if it already exists.
    pub fn add_cluster(&mut self, name: &str) -> bool {
        if self.store.cluster(name).is_ok() {
            return false;
        }
        *self.store.add_cluster(name) = ClusterStore::new(self.log_capacity);
        let ns = resource("", "v1", "namespaces").expect("namespaces are served");
        for n in strategy::SYSTEM_NAMESPACES {
            let body = json!({"apiVersion": "v1", "kind": "Namespace", "metadata": {"name": n}});
            let req = HttpRequest::new("POST", "/api/v1/namespaces").json(&body);
            let ctx = Ctx {
                cluster: name,
                r: ns,
                namespace: None,
                name: None,
                subresource: None,
                query: Query::default(),
            };
            self.create(&ctx, &req)
                .expect("system namespaces are valid");
        }
        self.trace(format!("cluster {name} created"));
        true
    }

    /// Forks cluster `from` into a new cluster `to` in O(1) (ADR 0002).
    pub fn fork_cluster(&mut self, from: &str, to: &str) -> ApiResult<()> {
        self.store
            .fork(from, to)
            .map(|_| ())
            .map_err(|e| ApiError::bad_request(e.to_string()))?;
        self.trace(format!("cluster {to} forked from {from}"));
        Ok(())
    }

    /// The event trace since the last call.
    pub fn take_trace(&mut self) -> Vec<String> {
        std::mem::take(&mut self.trace)
    }

    fn trace(&mut self, line: String) {
        let t = self.env.clock.now().0;
        self.trace.push(format!("t={t} {line}"));
    }

    fn now(&self) -> MonoTime {
        self.env.clock.now()
    }

    pub(crate) fn cluster(&self, name: &str) -> ApiResult<&ClusterStore<Obj>> {
        self.store
            .cluster(name)
            .map_err(|_| ApiError::not_found_path(format!("cluster {name}")))
    }

    pub(crate) fn cluster_mut(&mut self, name: &str) -> ApiResult<&mut ClusterStore<Obj>> {
        self.store
            .cluster_mut(name)
            .map_err(|_| ApiError::not_found_path(format!("cluster {name}")))
    }

    /// Handles one request against `cluster`.
    pub fn handle(&mut self, cluster: &str, req: &HttpRequest) -> Reply {
        let reply = self
            .route(cluster, req)
            .unwrap_or_else(|e| Reply::Response(HttpResponse::error(&e)));
        let rv = self
            .store
            .cluster(cluster)
            .map_or(0, ClusterStore::resource_version);
        let q = if req.query.is_empty() {
            String::new()
        } else {
            format!("?{}", req.query)
        };
        let what = match &reply {
            Reply::Response(r) => r.status.to_string(),
            Reply::Watch(c) => format!("watch from {}", c.rv),
        };
        self.trace(format!(
            "{cluster} {} {}{q} -> {what} rv={rv}",
            req.method, req.path
        ));
        reply
    }

    /// Advances a watch: the events since the last poll, and whether the
    /// stream is over.
    pub fn watch_poll(&mut self, c: &mut WatchCursor) -> WatchBatch {
        let now = self.now();
        match self.store.cluster(&c.cluster) {
            Ok(cs) => c.poll(cs, now),
            Err(_) => WatchBatch {
                events: Vec::new(),
                closed: true,
            },
        }
    }

    fn route(&mut self, cluster: &str, req: &HttpRequest) -> ApiResult<Reply> {
        let target = parse_path(&req.path)
            .ok_or_else(|| ApiError::not_found_path(format!("{} {}", req.method, req.path)))?;
        let p = match target {
            Target::Resource(p) => p,
            other => {
                if req.method != "GET" {
                    return Err(ApiError::method_not_allowed(format!(
                        "the server does not allow this method on the requested resource ({} {})",
                        req.method, req.path
                    )));
                }
                return self.non_resource(other, req).map(Reply::Response);
            }
        };
        accepts_json(req.accept.as_deref())?;
        self.cluster(cluster)?;
        let r = resource(&p.group, &p.version, &p.resource)
            .ok_or_else(|| ApiError::not_found_path(format!("{} {}", req.method, req.path)))?;
        if p.namespace.is_some() && !r.namespaced {
            return Err(ApiError::not_found_path(format!(
                "{} {}",
                req.method, req.path
            )));
        }
        if let Some(s) = &p.subresource
            && r.subresource(s).is_none()
        {
            return Err(ApiError::not_found_path(format!(
                "{} {}",
                req.method, req.path
            )));
        }
        let mut ctx = Ctx {
            cluster,
            r,
            namespace: p.namespace.clone(),
            name: p.name.clone(),
            subresource: p.subresource.clone(),
            query: Query::parse(&req.query),
        };
        let verb = match (req.method.as_str(), &ctx.name) {
            ("GET", _) if p.legacy_watch => "watch",
            ("GET", Some(_)) => "get",
            ("GET", None) if ctx.query.bool("watch")? => "watch",
            ("GET", None) => "list",
            ("POST", None) => "create",
            ("POST", Some(_)) if ctx.subresource.is_some() => "create",
            ("PUT", Some(_)) => "update",
            ("PATCH", Some(_)) => "patch",
            ("DELETE", Some(_)) => "delete",
            ("DELETE", None) => "deletecollection",
            _ => "",
        };
        let allowed = match &ctx.subresource {
            Some(s) => r.subresource(s).is_some_and(|s| s.verbs.contains(&verb)),
            None => r.has_verb(verb),
        };
        // Across all namespaces, a namespaced resource only lists and watches.
        let cross_ns = r.namespaced && ctx.namespace.is_none() && !matches!(verb, "list" | "watch");
        if !allowed || cross_ns {
            return Err(ApiError::method_not_allowed(format!(
                "the server does not allow this method on the requested resource ({} {})",
                req.method, req.path
            )));
        }
        let ok = |status, v| Ok(Reply::Response(HttpResponse::json(status, v)));
        match verb {
            "get" => ok(200, self.get(&ctx)?),
            "list" => ok(200, self.list(&ctx)?),
            "watch" => {
                if p.legacy_watch
                    && let Some(n) = ctx.name.take()
                {
                    let fs = ctx.query.get("fieldSelector").unwrap_or("").to_owned();
                    let sel = if fs.is_empty() {
                        format!("metadata.name={n}")
                    } else {
                        format!("{fs},metadata.name={n}")
                    };
                    ctx.query = Query::parse(&format!(
                        "{}&fieldSelector={}",
                        req.query,
                        sel.replace('&', "%26")
                    ));
                }
                self.watch(&ctx).map(Reply::Watch)
            }
            "create" => self.create(&ctx, req).map(Reply::Response),
            "update" => self.update(&ctx, req).map(Reply::Response),
            "patch" => self.patch(&ctx, req).map(Reply::Response),
            "delete" => self.delete(&ctx, req).map(Reply::Response),
            "deletecollection" => self.delete_collection(&ctx, req).map(Reply::Response),
            _ => unreachable!("verbs are checked above"),
        }
    }

    fn non_resource(&self, t: Target, req: &HttpRequest) -> ApiResult<HttpResponse> {
        let accept = req.accept.as_deref();
        if let Target::RootDiscovery { core } = t
            && wants_aggregated_discovery(accept)
        {
            return Ok(discovery::aggregated(core));
        }
        if !matches!(t, Target::Health) {
            accepts_json(accept)?;
        }
        Ok(match t {
            Target::RootDiscovery { core: true } => {
                HttpResponse::json(200, discovery::api_versions())
            }
            Target::RootDiscovery { core: false } => {
                HttpResponse::json(200, discovery::api_group_list())
            }
            Target::GroupDiscovery { group } => {
                HttpResponse::json(200, discovery::api_group(&group)?)
            }
            Target::ResourceList { group, version } => {
                HttpResponse::json(200, discovery::api_resource_list(&group, &version)?)
            }
            Target::Version => HttpResponse::json(200, discovery::version()),
            Target::Health => HttpResponse {
                status: 200,
                content_type: "text/plain; charset=utf-8".into(),
                body: Body::Text("ok".into()),
                warnings: Vec::new(),
            },
            Target::OpenApiV2 => discovery::openapi_v2(),
            Target::OpenApiV3(p) => discovery::openapi_v3(p.as_deref())?,
            Target::Resource(_) => unreachable!("handled by the caller"),
        })
    }

    fn get(&self, ctx: &Ctx<'_>) -> ApiResult<Value> {
        let cs = self.cluster(ctx.cluster)?;
        if let Some(rv) = parse_rv(ctx.query.get("resourceVersion"))?
            && rv > cs.resource_version()
        {
            return Err(ApiError::too_large_resource_version(
                rv,
                cs.resource_version(),
            ));
        }
        let name = ctx.name.as_deref().unwrap_or_default();
        cs.get(&key(ctx.r, ctx.ns(), name))
            .map(|o| (*o.value).clone())
            .ok_or_else(|| ApiError::not_found(ctx.subject(name)))
    }

    fn list(&self, ctx: &Ctx<'_>) -> ApiResult<Value> {
        let cs = self.cluster(ctx.cluster)?;
        let filter = Filter::parse(&ctx.query, ctx.r.group, ctx.r.plural)?;
        let page = read::list(
            cs,
            &storage_resource(ctx.r),
            ctx.namespace.as_deref(),
            &ctx.query,
            &filter,
        )?;
        let mut meta = json!({"resourceVersion": page.rv.to_string()});
        if let Some(c) = page.cont {
            meta["continue"] = json!(c);
        }
        if let Some(n) = page.remaining {
            meta["remainingItemCount"] = json!(n);
        }
        Ok(json!({
            "kind": format!("{}List", ctx.r.kind),
            "apiVersion": ctx.r.api_version(),
            "metadata": meta,
            "items": page.items.iter().map(|v| as_list_item(v)).collect::<Vec<_>>(),
        }))
    }

    fn watch(&mut self, ctx: &Ctx<'_>) -> ApiResult<WatchCursor> {
        let q = &ctx.query;
        let filter = Filter::parse(q, ctx.r.group, ctx.r.plural)?;
        let send_initial = q.bool("sendInitialEvents")?;
        let rv_match = q.get("resourceVersionMatch").unwrap_or("");
        let bookmarks = q.bool("allowWatchBookmarks")?;
        if send_initial && rv_match != "NotOlderThan" {
            return Err(ApiError::bad_request(
                "sendInitialEvents is forbidden for watch unless resourceVersionMatch is NotOlderThan",
            ));
        }
        if !send_initial && !rv_match.is_empty() {
            return Err(ApiError::bad_request(
                "resourceVersionMatch is forbidden for watch unless sendInitialEvents is provided",
            ));
        }
        let rv = parse_rv(q.get("resourceVersion"))?;
        let timeout = match q.int("timeoutSeconds")? {
            Some(t) if t > 0 => Duration::from_secs(t as u64),
            _ => Duration::from_secs(1800 + self.env.rng.below(1800)),
        };
        let now = self.now();
        let resource = storage_resource(ctx.r);
        let cs = self.cluster(ctx.cluster)?;
        let mut cursor = WatchCursor {
            cluster: ctx.cluster.to_owned(),
            resource: resource.clone(),
            namespace: ctx.namespace.clone(),
            filter,
            api_version: ctx.r.api_version(),
            kind: ctx.r.kind.to_owned(),
            rv: 0,
            pending: VecDeque::new(),
            bookmarks,
            last_bookmark: now,
            deadline: MonoTime(now.0.saturating_add(timeout.as_nanos() as u64)),
            done: false,
        };
        if send_initial || matches!(rv, None | Some(0)) {
            if let Some(n) = rv
                && n > cs.resource_version()
            {
                return Err(ApiError::too_large_resource_version(
                    n,
                    cs.resource_version(),
                ));
            }
            let snap = read::snapshot(cs, &resource, ctx.namespace.as_deref(), None)?;
            cursor.rv = snap.rv;
            for v in snap.objects.values().filter(|v| cursor.filter.matches(v)) {
                cursor
                    .pending
                    .push_back(WatchCursor::event("ADDED", (**v).clone()));
            }
            if send_initial {
                let b = cursor.bookmark(true);
                cursor.pending.push_back(b);
            }
        } else {
            cursor.rv = rv.unwrap_or_default();
        }
        Ok(cursor)
    }
}

/// All served resources, for callers enumerating them (controllers, GC).
pub fn served() -> &'static [ResourceInfo] {
    RESOURCES
}

/// The resourceVersion of a cluster, for callers that wait on changes.
pub fn cluster_resource_version(api: &Api, cluster: &str) -> Option<ResourceVersion> {
    api.store
        .cluster(cluster)
        .ok()
        .map(ClusterStore::resource_version)
}

#[cfg(test)]
mod tests;
