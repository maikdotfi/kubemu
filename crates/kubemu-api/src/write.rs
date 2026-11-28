//! Writes: create, update (incl. `/status`), patch, delete and
//! deletecollection, with built-in admission (namespace lifecycle,
//! service accounts), preconditions, finalizers and graceful deletion.

use std::sync::Arc;

use kubemu_api_types::ResourceInfo;
use kubemu_store::{ObjectKey, StoreError};
use serde_json::{Map, Value, json};

use crate::decode::{FieldValidation, decode};
use crate::http::{HttpRequest, HttpResponse, body_type};
use crate::meta::{self, name_rule};
use crate::object::{self as o, key, meta_mut};
use crate::read::Filter;
use crate::status::{self, ApiError, ApiResult, Cause};
use crate::strategy;
use crate::{Api, Ctx};

fn parse_json(body: &[u8]) -> ApiResult<Value> {
    serde_json::from_slice(body).map_err(|e| {
        ApiError::bad_request(format!("couldn't get version/kind; json parse error: {e}"))
    })
}

fn check_body_type(req: &HttpRequest) -> ApiResult<()> {
    match body_type(req.content_type.as_deref()).as_str() {
        "application/json" | "" | "*/*" => Ok(()),
        other => Err(ApiError::unsupported_media_type(format!(
            "the body of the request was in an unknown format ({other}) - accepted media types include: application/json"
        ))),
    }
}

/// System-owned metadata a client may not set; carried over on update.
const SYSTEM_META: [&str; 6] = [
    "uid",
    "creationTimestamp",
    "deletionTimestamp",
    "deletionGracePeriodSeconds",
    "generation",
    "managedFields",
];

impl Api {
    /// Decodes an object for `ctx.r`: kind and apiVersion checked or filled
    /// in, unknown fields handled per `fieldValidation`.
    fn decode_object(&self, ctx: &Ctx<'_>, v: Value) -> ApiResult<(Value, Vec<String>)> {
        let r = ctx.r;
        let Value::Object(m) = &v else {
            return Err(ApiError::bad_request(
                "couldn't get version/kind; json parse error: expected an object",
            ));
        };
        let want = r.api_version();
        if let Some(av) = m
            .get("apiVersion")
            .and_then(Value::as_str)
            .filter(|s| !s.is_empty())
            && av != want
        {
            return Err(ApiError::bad_request(format!(
                "the API version in the data ({av}) does not match the expected API version ({want})"
            )));
        }
        if let Some(k) = m
            .get("kind")
            .and_then(Value::as_str)
            .filter(|s| !s.is_empty())
            && k != r.kind
        {
            return Err(ApiError::bad_request(format!(
                "the kind in the data ({k}) does not match the expected kind ({})",
                r.kind
            )));
        }
        let fv = FieldValidation::parse(ctx.query.get("fieldValidation"))?;
        let d = decode(v, r.schema(), r.kind, r.version)?;
        let warnings = d.check(fv, r.kind, r.version)?;
        let mut obj = d.value;
        obj["apiVersion"] = json!(want);
        obj["kind"] = json!(r.kind);
        Ok((obj, warnings))
    }

    /// Fills or checks `metadata.namespace` against the URL.
    fn set_namespace(&self, ctx: &Ctx<'_>, obj: &mut Value) -> ApiResult<()> {
        let m = meta_mut(obj);
        if !ctx.r.namespaced {
            m.remove("namespace");
            return Ok(());
        }
        match m
            .get("namespace")
            .and_then(Value::as_str)
            .filter(|s| !s.is_empty())
        {
            Some(ns) if ns != ctx.ns() => Err(ApiError::bad_request(
                "the namespace of the provided object does not match the namespace sent on the request",
            )),
            _ => {
                m.insert("namespace".into(), json!(ctx.ns()));
                Ok(())
            }
        }
    }

    /// NamespaceLifecycle admission for creates.
    fn admit_namespace(&self, ctx: &Ctx<'_>, name: &str) -> ApiResult<()> {
        if !ctx.r.namespaced {
            return Ok(());
        }
        let ns_res = kubemu_api_types::resource("", "v1", "namespaces").expect("served");
        let cs = self.cluster(ctx.cluster)?;
        match cs.get(&key(ns_res, "", ctx.ns())) {
            None => Err(ApiError::not_found(status::Subject {
                group: "",
                resource: "namespaces",
                name: ctx.ns(),
            })),
            Some(ns) if o::is_deleting(&ns.value) => Err(ApiError::forbidden(
                ctx.subject(name),
                &format!(
                    "unable to create new content in namespace {} because it is being terminated",
                    ctx.ns()
                ),
            )),
            Some(_) => Ok(()),
        }
    }

    /// Metadata rules every resource shares (ADR 0013).
    fn validate(&self, r: &ResourceInfo, obj: &Value) -> ApiResult<()> {
        let mut causes: Vec<Cause> = Vec::new();
        let name = o::name(obj);
        meta::validate_name(name, name_rule(r.group, r.plural), &mut causes);
        if r.namespaced {
            meta::validate_namespace_name(o::namespace(obj), &mut causes);
        }
        if let Some(Value::Object(l)) = o::meta(obj).and_then(|m| m.get("labels")) {
            meta::validate_labels(l, "metadata.labels", &mut causes);
        }
        if let Some(Value::Object(a)) = o::meta(obj).and_then(|m| m.get("annotations")) {
            meta::validate_annotations(a, &mut causes);
        }
        if causes.is_empty() {
            Ok(())
        } else {
            Err(ApiError::invalid(r.kind, r.group, name, causes))
        }
    }

    fn store_err(&self, ctx: &Ctx<'_>, name: &str, e: StoreError) -> ApiError {
        match e {
            StoreError::AlreadyExists(_) => ApiError::already_exists(ctx.subject(name)),
            StoreError::NotFound(_) => ApiError::not_found(ctx.subject(name)),
            StoreError::Conflict { .. } => ApiError::modified(ctx.subject(name)),
            other => ApiError::internal(other.to_string()),
        }
    }

    /// Writes `obj` with its resourceVersion set to the one the store is
    /// about to assign (every write bumps it by one).
    fn put(
        &mut self,
        ctx: &Ctx<'_>,
        k: ObjectKey,
        mut obj: Value,
        expected: Option<u64>,
        create: bool,
    ) -> ApiResult<Value> {
        let name = k.name.clone();
        let cs = self.cluster_mut(ctx.cluster)?;
        let rv = cs.resource_version() + 1;
        o::set_resource_version(&mut obj, rv);
        let shared = Arc::new(obj.clone());
        let res = if create {
            cs.create(k, shared)
        } else {
            cs.update(&k, shared, expected)
        };
        match res {
            Ok(got) => {
                debug_assert_eq!(got, rv);
                Ok(obj)
            }
            Err(e) => Err(self.store_err(ctx, &name, e)),
        }
    }

    /// `POST pods/<name>/binding`: how a scheduler assigns a node.
    fn bind(&mut self, ctx: &Ctx<'_>, req: &HttpRequest) -> ApiResult<HttpResponse> {
        let b = parse_json(&req.body)?;
        let name = ctx.name.clone().unwrap_or_default();
        let node = b["target"]["name"].as_str().unwrap_or("").to_owned();
        if node.is_empty() {
            return Err(ApiError::invalid(
                "Binding",
                "",
                &name,
                vec![meta::required("target.name", "")],
            ));
        }
        let k = key(ctx.r, ctx.ns(), &name);
        let old = self
            .cluster(ctx.cluster)?
            .get(&k)
            .map(|o| o.value.clone())
            .ok_or_else(|| ApiError::not_found(ctx.subject(&name)))?;
        if let Some(n) = old["spec"]["nodeName"].as_str().filter(|n| !n.is_empty()) {
            return Err(ApiError::conflict(
                ctx.subject(&name),
                &format!("pod {name} is already assigned to node {n:?}"),
            ));
        }
        if o::is_deleting(&old) {
            return Err(ApiError::conflict(
                ctx.subject(&name),
                "pod is being deleted",
            ));
        }
        let mut obj = (*old).clone();
        obj["spec"]["nodeName"] = json!(node);
        let mut conds: Vec<Value> = obj["status"]["conditions"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .filter(|c| c["type"] != "PodScheduled")
            .collect();
        conds.push(json!({
            "type": "PodScheduled",
            "status": "True",
            "lastProbeTime": null,
            "lastTransitionTime": meta::now_rfc3339(&self.env),
        }));
        obj["status"]["conditions"] = json!(conds);
        if !ctx.dry_run()? {
            let expected = o::resource_version(&old).and_then(|s| s.parse().ok());
            self.put(ctx, k, obj, expected, false)?;
        }
        Ok(HttpResponse::json(
            201,
            json!({"kind": "Status", "apiVersion": "v1", "metadata": {}, "status": "Success", "code": 201}),
        ))
    }

    pub(crate) fn create(&mut self, ctx: &Ctx<'_>, req: &HttpRequest) -> ApiResult<HttpResponse> {
        check_body_type(req)?;
        if ctx.subresource.as_deref() == Some("binding") {
            return self.bind(ctx, req);
        }
        let (obj, warnings) = self.decode_object(ctx, parse_json(&req.body)?)?;
        if ctx.r.group == "authorization.k8s.io" {
            return Ok(with_warnings(
                HttpResponse::json(201, review(obj)),
                warnings,
            ));
        }
        self.create_value(ctx, obj, warnings)
    }

    fn create_value(
        &mut self,
        ctx: &Ctx<'_>,
        mut obj: Value,
        warnings: Vec<String>,
    ) -> ApiResult<HttpResponse> {
        let r = ctx.r;
        self.set_namespace(ctx, &mut obj)?;
        let m = meta_mut(&mut obj);
        if m.get("resourceVersion")
            .and_then(Value::as_str)
            .is_some_and(|s| !s.is_empty())
        {
            return Err(ApiError::bad_request(
                "resourceVersion should not be set on objects to be created",
            ));
        }
        for f in SYSTEM_META {
            m.remove(f);
        }
        if m.get("name")
            .and_then(Value::as_str)
            .unwrap_or("")
            .is_empty()
        {
            match m
                .get("generateName")
                .and_then(Value::as_str)
                .filter(|s| !s.is_empty())
            {
                Some(prefix) => {
                    let n = meta::generate_name(prefix, &self.env);
                    m.insert("name".into(), json!(n));
                }
                None => {
                    return Err(ApiError::invalid(
                        r.kind,
                        r.group,
                        "",
                        vec![meta::required(
                            "metadata.name",
                            "name or generateName is required",
                        )],
                    ));
                }
            }
        }
        m.insert("uid".into(), json!(meta::uuid_v4(&self.env)));
        m.insert(
            "creationTimestamp".into(),
            json!(meta::now_rfc3339(&self.env)),
        );
        let name = o::name(&obj).to_owned();
        self.admit_namespace(ctx, &name)?;
        strategy::prepare_for_create(r, &mut obj);
        self.validate(r, &obj)?;
        if ctx.dry_run()? {
            return Ok(with_warnings(HttpResponse::json(201, obj), warnings));
        }
        let k = key(r, ctx.ns(), &name);
        let stored = self.put(ctx, k, obj, None, true)?;
        Ok(with_warnings(HttpResponse::json(201, stored), warnings))
    }

    pub(crate) fn update(&mut self, ctx: &Ctx<'_>, req: &HttpRequest) -> ApiResult<HttpResponse> {
        check_body_type(req)?;
        let (obj, warnings) = self.decode_object(ctx, parse_json(&req.body)?)?;
        self.update_value(ctx, obj, warnings, None)
    }

    /// The shared tail of update and patch. `patched_from` is the
    /// resourceVersion a patch was applied to.
    fn update_value(
        &mut self,
        ctx: &Ctx<'_>,
        mut obj: Value,
        warnings: Vec<String>,
        patched_from: Option<&str>,
    ) -> ApiResult<HttpResponse> {
        let r = ctx.r;
        let url_name = ctx.name.clone().unwrap_or_default();
        {
            let m = meta_mut(&mut obj);
            match m
                .get("name")
                .and_then(Value::as_str)
                .filter(|s| !s.is_empty())
            {
                Some(n) if n != url_name => {
                    return Err(ApiError::bad_request(format!(
                        "the name of the object ({n}) does not match the name on the URL ({url_name})"
                    )));
                }
                _ => {
                    m.insert("name".into(), json!(url_name));
                }
            }
        }
        self.set_namespace(ctx, &mut obj)?;
        let sub = ctx.subresource.as_deref();
        let k = key(r, ctx.ns(), &url_name);
        let Some(old) = self.cluster(ctx.cluster)?.get(&k).map(|o| o.value.clone()) else {
            if strategy::allow_create_on_update(r) && sub.is_none() && patched_from.is_none() {
                o::meta_mut(&mut obj).remove("resourceVersion");
                return self.create_value(ctx, obj, warnings);
            }
            return Err(ApiError::not_found(ctx.subject(&url_name)));
        };
        let old_rv = o::resource_version(&old).unwrap_or_default().to_owned();
        let sent_rv = o::resource_version(&obj).map(str::to_owned);
        let conflict = match patched_from {
            // A patch may set resourceVersion as a precondition.
            Some(from) => sent_rv.as_deref().is_some_and(|s| s != from) || from != old_rv,
            None => sent_rv.as_deref().is_some_and(|s| s != old_rv),
        };
        if conflict {
            return Err(ApiError::modified(ctx.subject(&url_name)));
        }
        if let Some(u) = o::uid(&obj)
            && !u.is_empty()
            && Some(u) != o::uid(&old)
        {
            return Err(ApiError::conflict(
                ctx.subject(&url_name),
                &format!(
                    "Precondition failed: UID in precondition: {u}, UID in object meta: {}",
                    o::uid(&old).unwrap_or_default()
                ),
            ));
        }
        {
            let old_meta = o::meta(&old).cloned().unwrap_or_default();
            let m = meta_mut(&mut obj);
            for f in SYSTEM_META {
                match old_meta.get(f) {
                    Some(v) => m.insert(f.into(), v.clone()),
                    None => m.remove(f),
                };
            }
            m.insert("resourceVersion".into(), json!(old_rv));
        }
        strategy::prepare_for_update(r, &mut obj, &old, sub);
        if o::is_deleting(&old) {
            let had = o::finalizers(&old);
            if o::finalizers(&obj).iter().any(|f| !had.contains(f)) {
                return Err(ApiError::invalid(
                    r.kind,
                    r.group,
                    &url_name,
                    vec![Cause {
                        reason: "FieldValueForbidden",
                        message: "Forbidden: no new finalizers can be added if the object is being deleted".into(),
                        field: "metadata.finalizers".into(),
                    }],
                ));
            }
        }
        self.validate(r, &obj)?;
        if obj == *old {
            return Ok(with_warnings(HttpResponse::json(200, obj), warnings));
        }
        if ctx.dry_run()? {
            return Ok(with_warnings(HttpResponse::json(200, obj), warnings));
        }
        let expected = old_rv.parse().ok();
        // The last finalizer of a deleting object is gone: delete it.
        if o::is_deleting(&obj)
            && o::finalizers(&obj).is_empty()
            && !self.pod_awaits_kubelet(r, &obj)
        {
            self.cluster_mut(ctx.cluster)?
                .delete(&k, expected)
                .map_err(|e| self.store_err(ctx, &url_name, e))?;
            return Ok(with_warnings(HttpResponse::json(200, obj), warnings));
        }
        let stored = self.put(ctx, k, obj, expected, false)?;
        Ok(with_warnings(HttpResponse::json(200, stored), warnings))
    }

    /// A deleting pod on a node stays until its kubelet confirms.
    fn pod_awaits_kubelet(&self, r: &ResourceInfo, obj: &Value) -> bool {
        r.group.is_empty()
            && r.plural == "pods"
            && obj["metadata"]["deletionGracePeriodSeconds"]
                .as_i64()
                .unwrap_or(0)
                > 0
    }

    pub(crate) fn patch(&mut self, ctx: &Ctx<'_>, req: &HttpRequest) -> ApiResult<HttpResponse> {
        let ty = body_type(req.content_type.as_deref());
        let patch = match ty.as_str() {
            "application/json-patch+json"
            | "application/merge-patch+json"
            | "application/strategic-merge-patch+json" => {
                serde_json::from_slice::<Value>(&req.body)
                    .map_err(|e| ApiError::bad_request(format!("error decoding patch: {e}")))?
            }
            "application/apply-patch+yaml" | "application/apply-patch+cbor" => {
                return Err(ApiError::unsupported_media_type(
                    "server-side apply is not supported by kubemu yet (planned for M3); use client-side apply",
                ));
            }
            other => {
                return Err(ApiError::unsupported_media_type(format!(
                    "the body of the request was in an unknown format ({other}) - accepted media types include: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json"
                )));
            }
        };
        let name = ctx.name.clone().unwrap_or_default();
        let current = self
            .cluster(ctx.cluster)?
            .get(&key(ctx.r, ctx.ns(), &name))
            .map(|o| (*o.value).clone())
            .ok_or_else(|| ApiError::not_found(ctx.subject(&name)))?;
        let from = o::resource_version(&current).unwrap_or_default().to_owned();
        let patched = match ty.as_str() {
            "application/json-patch+json" => {
                kubemu_patch::json_patch(&current, &patch).map_err(|e| match e {
                    kubemu_patch::JsonPatchError::Decode(m) => ApiError::bad_request(m),
                    kubemu_patch::JsonPatchError::TooLarge(_) => {
                        ApiError::new(413, "RequestEntityTooLarge", e.to_string())
                    }
                    kubemu_patch::JsonPatchError::Apply(m) => ApiError::unprocessable(m),
                })?
            }
            "application/merge-patch+json" => kubemu_patch::merge_patch(&current, &patch)
                .map_err(|e| ApiError::bad_request(e.to_string()))?,
            _ => kubemu_patch::strategic_merge_patch(&current, &patch, ctx.r.schema())
                .map_err(|e| ApiError::unprocessable(e.to_string()))?,
        };
        let (obj, warnings) = self.decode_object(ctx, patched)?;
        self.update_value(ctx, obj, warnings, Some(&from))
    }

    pub(crate) fn delete(&mut self, ctx: &Ctx<'_>, req: &HttpRequest) -> ApiResult<HttpResponse> {
        let opts = DeleteOptions::parse(ctx, &req.body)?;
        let name = ctx.name.clone().unwrap_or_default();
        self.delete_one(ctx, &name, &opts)
    }

    fn delete_one(
        &mut self,
        ctx: &Ctx<'_>,
        name: &str,
        opts: &DeleteOptions,
    ) -> ApiResult<HttpResponse> {
        let r = ctx.r;
        let k = key(r, ctx.ns(), name);
        let old = self
            .cluster(ctx.cluster)?
            .get(&k)
            .map(|o| o.value.clone())
            .ok_or_else(|| ApiError::not_found(ctx.subject(name)))?;
        if let Some(u) = &opts.uid
            && Some(u.as_str()) != o::uid(&old)
        {
            return Err(ApiError::conflict(
                ctx.subject(name),
                &format!(
                    "Precondition failed: UID in precondition: {u}, UID in object meta: {}",
                    o::uid(&old).unwrap_or_default()
                ),
            ));
        }
        if let Some(rv) = &opts.resource_version
            && Some(rv.as_str()) != o::resource_version(&old)
        {
            return Err(ApiError::conflict(
                ctx.subject(name),
                &format!(
                    "Precondition failed: ResourceVersion in precondition: {rv}, ResourceVersion in object meta: {}",
                    o::resource_version(&old).unwrap_or_default()
                ),
            ));
        }
        let is_ns = r.group.is_empty() && r.plural == "namespaces";
        if is_ns && strategy::UNDELETABLE_NAMESPACES.contains(&name) {
            return Err(ApiError::forbidden(
                ctx.subject(name),
                "this namespace may not be deleted",
            ));
        }

        let mut obj = (*old).clone();
        let mut fins = o::finalizers(&obj);
        if let Some(f) = opts.propagation_finalizer()
            && !fins.iter().any(|x| x == f)
        {
            fins.push(f.to_owned());
        }
        let is_pod = r.group.is_empty() && r.plural == "pods";
        let forced = is_pod && opts.grace_period == Some(0);
        let grace = if is_pod {
            strategy::pod_grace_period(&obj, opts.grace_period)
        } else {
            None
        };
        // Namespaces always go through the namespace controller.
        // Namespaces wait for the namespace controller (spec.finalizers).
        let ns_held = is_ns && !strategy::finalized(r, &json!({"spec": obj["spec"]}));
        let pending = ns_held || !fins.is_empty() || grace.is_some();
        let dry = ctx.dry_run()?;
        if !pending {
            if !dry {
                let expected = o::resource_version(&old).and_then(|s| s.parse().ok());
                self.cluster_mut(ctx.cluster)?
                    .delete(&k, expected)
                    .map_err(|e| self.store_err(ctx, name, e))?;
            }
            let body = if strategy::returns_deleted_object(r) {
                (*old).clone()
            } else {
                status::success(ctx.subject(name), o::uid(&old))
            };
            return Ok(HttpResponse::json(200, body));
        }

        let now_secs = self.env.clock.wall().0 / 1_000_000_000;
        {
            let m = meta_mut(&mut obj);
            if !m.contains_key("deletionTimestamp") {
                let g = grace.unwrap_or(0);
                m.insert(
                    "deletionTimestamp".into(),
                    json!(meta::rfc3339(now_secs + g as u64)),
                );
                m.insert("deletionGracePeriodSeconds".into(), json!(g));
            } else if let Some(g) = grace.or(forced.then_some(0)) {
                // A shorter grace period moves the deadline in; a forced
                // (grace 0) delete means the kubelet is done.
                let cur = m
                    .get("deletionGracePeriodSeconds")
                    .and_then(Value::as_i64)
                    .unwrap_or(g);
                if g < cur {
                    m.insert(
                        "deletionTimestamp".into(),
                        json!(meta::rfc3339(now_secs + g as u64)),
                    );
                    m.insert("deletionGracePeriodSeconds".into(), json!(g));
                }
            }
            if !fins.is_empty() {
                m.insert("finalizers".into(), json!(fins));
            }
        }
        if is_ns {
            obj["status"]["phase"] = json!("Terminating");
        }
        if obj == *old || dry {
            return Ok(HttpResponse::json(200, obj));
        }
        let expected = o::resource_version(&old).and_then(|s| s.parse().ok());
        let stored = self.put(ctx, k, obj, expected, false)?;
        Ok(HttpResponse::json(200, stored))
    }

    pub(crate) fn delete_collection(
        &mut self,
        ctx: &Ctx<'_>,
        req: &HttpRequest,
    ) -> ApiResult<HttpResponse> {
        let opts = DeleteOptions::parse(ctx, &req.body)?;
        let filter = Filter::parse(&ctx.query, ctx.r.group, ctx.r.plural)?;
        let names: Vec<String> = self
            .cluster(ctx.cluster)?
            .list(&o::storage_resource(ctx.r), ctx.namespace.as_deref())
            .0
            .into_iter()
            .filter(|(_, v)| filter.matches(&v.value))
            .map(|(k, _)| k.name.clone())
            .collect();
        let mut items = Vec::new();
        for n in names {
            match self.delete_one(ctx, &n, &opts) {
                Ok(r) => items.extend(r.value().filter(|v| v["kind"] == ctx.r.kind)),
                Err(e) if e.code == 404 => {}
                Err(e) => return Err(e),
            }
        }
        let rv = self.cluster(ctx.cluster)?.resource_version();
        Ok(HttpResponse::json(
            200,
            json!({
                "kind": format!("{}List", ctx.r.kind),
                "apiVersion": ctx.r.api_version(),
                "metadata": {"resourceVersion": rv.to_string()},
                "items": items.iter().map(o::as_list_item).collect::<Vec<_>>(),
            }),
        ))
    }
}

/// Self-subject reviews: there is no authorization in M1, so everything is
/// allowed (ADR 0011). The review is answered, never stored.
fn review(mut obj: Value) -> Value {
    if let Some(m) = obj.get_mut("metadata").and_then(Value::as_object_mut) {
        m.insert("creationTimestamp".into(), Value::Null);
    }
    obj["status"] = if obj["kind"] == "SelfSubjectAccessReview" {
        json!({"allowed": true, "reason": "kubemu has no authorization: every client is cluster-admin"})
    } else {
        json!({
            "resourceRules": [{"verbs": ["*"], "apiGroups": ["*"], "resources": ["*"]}],
            "nonResourceRules": [{"verbs": ["*"], "nonResourceURLs": ["*"]}],
            "incomplete": false,
        })
    };
    obj
}

fn with_warnings(mut r: HttpResponse, w: Vec<String>) -> HttpResponse {
    r.warnings = w;
    r
}

/// `DeleteOptions`, from the body and the query.
#[derive(Debug, Default)]
struct DeleteOptions {
    grace_period: Option<i64>,
    propagation: Option<String>,
    orphan_dependents: Option<bool>,
    uid: Option<String>,
    resource_version: Option<String>,
}

impl DeleteOptions {
    fn parse(ctx: &Ctx<'_>, body: &[u8]) -> ApiResult<Self> {
        let b: Map<String, Value> = if body.iter().all(u8::is_ascii_whitespace) {
            Map::new()
        } else {
            match parse_json(body)? {
                Value::Object(m) => m,
                _ => return Err(ApiError::bad_request("DeleteOptions must be an object")),
            }
        };
        let q = &ctx.query;
        let s = |k: &str| b.get(k).and_then(Value::as_str).map(str::to_owned);
        let o = Self {
            grace_period: b
                .get("gracePeriodSeconds")
                .and_then(Value::as_i64)
                .or(q.int("gracePeriodSeconds")?),
            propagation: s("propagationPolicy")
                .or_else(|| q.get("propagationPolicy").map(str::to_owned)),
            orphan_dependents: b
                .get("orphanDependents")
                .and_then(Value::as_bool)
                .or(q.get("orphanDependents").map(|v| v == "true")),
            uid: b
                .get("preconditions")
                .and_then(|p| p.get("uid"))
                .and_then(Value::as_str)
                .map(str::to_owned),
            resource_version: b
                .get("preconditions")
                .and_then(|p| p.get("resourceVersion"))
                .and_then(Value::as_str)
                .map(str::to_owned),
        };
        if let Some(p) = &o.propagation
            && !matches!(p.as_str(), "Orphan" | "Background" | "Foreground")
        {
            return Err(ApiError::invalid(
                "DeleteOptions",
                "",
                "",
                vec![Cause {
                    reason: "FieldValueNotSupported",
                    message: format!(
                        "Unsupported value: {p:?}: supported values: \"Foreground\", \"Background\", \"Orphan\""
                    ),
                    field: "propagationPolicy".into(),
                }],
            ));
        }
        if o.propagation.is_some() && o.orphan_dependents.is_some() {
            return Err(ApiError::invalid(
                "DeleteOptions",
                "",
                "",
                vec![Cause {
                    reason: "FieldValueInvalid",
                    message: "Invalid value: \"null\": orphanDependents and deletionPropagation cannot be both set".into(),
                    field: "propagationPolicy".into(),
                }],
            ));
        }
        Ok(o)
    }

    /// The finalizer the requested propagation adds, if any.
    fn propagation_finalizer(&self) -> Option<&'static str> {
        match (self.propagation.as_deref(), self.orphan_dependents) {
            (Some("Orphan"), _) | (None, Some(true)) => Some("orphan"),
            (Some("Foreground"), _) => Some("foregroundDeletion"),
            _ => None,
        }
    }
}
