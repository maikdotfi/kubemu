//! Per-resource write behaviour, as upstream registry strategies define it
//! (`pkg/registry/core/*/strategy.go`): what create resets, what an update
//! of the main resource or of `/status` may change, and how deletion goes.
//!
//! Field defaulting and per-resource validation are ADR 0013 work and live
//! elsewhere; this is only what shapes the write itself.

use kubemu_api_types::ResourceInfo;
use serde_json::{Value, json};

use crate::object::meta_mut;

/// Namespaces the apiserver creates and refuses to delete.
pub const SYSTEM_NAMESPACES: [&str; 4] =
    ["default", "kube-system", "kube-public", "kube-node-lease"];
pub const UNDELETABLE_NAMESPACES: [&str; 3] = ["default", "kube-system", "kube-public"];

fn is(r: &ResourceInfo, group: &str, plural: &str) -> bool {
    r.group == group && r.plural == plural
}

pub fn has_status(r: &ResourceInfo) -> bool {
    r.subresource("status").is_some()
}

/// `AllowCreateOnUpdate`: PUT to a missing object creates it. Checked
/// against the registry strategies by the oracle (`strategy` op).
pub fn allow_create_on_update(r: &ResourceInfo) -> bool {
    matches!(
        (r.group, r.plural),
        ("", "endpoints" | "events" | "services")
    )
}

/// Whether DELETE answers with the deleted object rather than a `Status`.
pub fn returns_deleted_object(r: &ResourceInfo) -> bool {
    is(r, "", "pods")
}

/// Whether `metadata.generation` is maintained: set to 1 on create, bumped
/// when [`generation_changes`] says so.
pub fn tracks_generation(r: &ResourceInfo) -> bool {
    is(r, "", "pods") || is(r, "discovery.k8s.io", "endpointslices")
}

/// Whether an update bumps `metadata.generation`: a pod's spec changed; an
/// endpoint slice changed anywhere outside metadata, or in its labels.
fn generation_changes(r: &ResourceInfo, new: &Value, old: &Value) -> bool {
    if is(r, "", "pods") {
        return new.get("spec") != old.get("spec");
    }
    let strip = |v: &Value| {
        let mut v = v.clone();
        if let Some(m) = v.as_object_mut() {
            m.remove("metadata");
        }
        v
    };
    strip(new) != strip(old) || new["metadata"].get("labels") != old["metadata"].get("labels")
}

/// `PrepareForCreate`: the parts of a new object the client does not set.
pub fn prepare_for_create(r: &ResourceInfo, obj: &mut Value) {
    if tracks_generation(r) {
        meta_mut(obj).insert("generation".into(), json!(1));
    }
    if has_status(r) && !is(r, "", "nodes") {
        // Status is the system's to set; nodes are the exception (the
        // kubelet registers with status).
        if let Some(m) = obj.as_object_mut() {
            m.remove("status");
        }
    }
    match (r.group, r.plural) {
        ("", "pods") => {
            obj["status"] = json!({"phase": "Pending"});
            // ServiceAccount admission.
            let spec = &mut obj["spec"];
            if spec.is_object() {
                let sa = spec
                    .get("serviceAccountName")
                    .or_else(|| spec.get("serviceAccount"))
                    .and_then(Value::as_str)
                    .filter(|s| !s.is_empty())
                    .unwrap_or("default")
                    .to_owned();
                spec["serviceAccountName"] = json!(sa);
                spec["serviceAccount"] = json!(sa);
            }
        }
        ("", "namespaces") => {
            obj["spec"] = json!({"finalizers": ["kubernetes"]});
            obj["status"] = json!({"phase": "Active"});
        }
        ("", "services") => {
            obj["status"] = json!({"loadBalancer": {}});
        }
        _ => {}
    }
}

/// `Canonicalize`, after validation: a namespace's name label, which
/// defaulting could not set while a `generateName` was still pending; and
/// what the Service store's read decorator (`defaultOnRead`) does to every
/// Service it returns, stored here so reads need nothing.
pub fn canonicalize(r: &ResourceInfo, obj: &mut Value) {
    if is(r, "", "services")
        && let Some(spec) = obj.get_mut("spec").and_then(Value::as_object_mut)
    {
        let ip = spec
            .get("clusterIP")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_owned();
        let no_ips = spec
            .get("clusterIPs")
            .and_then(Value::as_array)
            .is_none_or(Vec::is_empty);
        if !ip.is_empty() && no_ips {
            spec.insert("clusterIPs".into(), json!([ip]));
        }
        if spec.get("type").and_then(Value::as_str) == Some("ExternalName") {
            spec.remove("internalTrafficPolicy");
        }
    }
    if is(r, "", "namespaces") {
        let name = crate::object::name(obj).to_owned();
        let m = meta_mut(obj);
        let labels = m.entry("labels").or_insert_with(|| json!({}));
        if !labels.is_object() {
            *labels = json!({});
        }
        labels["kubernetes.io/metadata.name"] = json!(name);
    }
}

/// Whether nothing holds a deleting object back any more: no finalizers,
/// and for namespaces no `spec.finalizers` either.
pub fn finalized(r: &ResourceInfo, obj: &Value) -> bool {
    crate::object::finalizers(obj).is_empty()
        && !(is(r, "", "namespaces")
            && obj["spec"]["finalizers"]
                .as_array()
                .is_some_and(|f| !f.is_empty()))
}

/// `PrepareForUpdate`: `new` becomes what is stored. The main resource
/// cannot change status; `/status` changes nothing but status; a
/// namespace's `/finalize` changes nothing but `spec.finalizers`.
pub fn prepare_for_update(r: &ResourceInfo, new: &mut Value, old: &Value, sub: Option<&str>) {
    if sub == Some("finalize") {
        let f = new["spec"].get("finalizers").cloned();
        *new = old.clone();
        match f {
            Some(f) => new["spec"]["finalizers"] = f,
            None => {
                if let Some(s) = new["spec"].as_object_mut() {
                    s.remove("finalizers");
                }
            }
        }
        return;
    }
    if sub == Some("status") {
        let s = new.get("status").cloned();
        *new = old.clone();
        match s {
            Some(s) => new["status"] = s,
            None => {
                if let Some(m) = new.as_object_mut() {
                    m.remove("status");
                }
            }
        }
        return;
    }
    if has_status(r) {
        match old.get("status") {
            Some(s) => new["status"] = s.clone(),
            None => {
                if let Some(m) = new.as_object_mut() {
                    m.remove("status");
                }
            }
        }
    }
    if is(r, "", "services") {
        drop_type_dependent_fields(new, old);
    }
    if is(r, "", "namespaces") {
        // spec.finalizers changes only through /finalize.
        new["spec"] = old.get("spec").cloned().unwrap_or(json!({}));
    }
    if tracks_generation(r) {
        let g = old["metadata"]["generation"].as_i64().unwrap_or(1);
        let bump = generation_changes(r, new, old);
        meta_mut(new).insert("generation".into(), json!(if bump { g + 1 } else { g }));
    }
}

/// `dropTypeDependentFields`, the cluster IP part: a Service turning
/// ExternalName loses the cluster IPs, families and policy it had, unless
/// the update changed them too (validation then refuses it).
fn drop_type_dependent_fields(new: &mut Value, old: &Value) {
    let needs_ip = |v: &Value| v["spec"]["type"] != "ExternalName";
    if !needs_ip(old) || needs_ip(new) {
        return;
    }
    let list = |v: &Value, k: &str| v["spec"][k].as_array().cloned().unwrap_or_default();
    // The strategy's `sameClusterIPs` compares both fields.
    let same_ips = new["spec"]["clusterIP"].as_str() == old["spec"]["clusterIP"].as_str()
        && list(new, "clusterIPs") == list(old, "clusterIPs");
    let same_families = list(new, "ipFamilies") == list(old, "ipFamilies");
    let same_policy =
        new["spec"]["ipFamilyPolicy"].as_str() == old["spec"]["ipFamilyPolicy"].as_str();
    let Some(spec) = new.get_mut("spec").and_then(Value::as_object_mut) else {
        return;
    };
    if same_ips {
        spec.remove("clusterIP");
        spec.remove("clusterIPs");
    }
    if same_families {
        spec.remove("ipFamilies");
    }
    if same_policy {
        spec.remove("ipFamilyPolicy");
    }
}

/// How long a pod's deletion waits for its kubelet, or `None` to delete
/// at once.
pub fn pod_grace_period(pod: &Value, requested: Option<i64>) -> Option<i64> {
    let phase = pod["status"]["phase"].as_str().unwrap_or("");
    let scheduled = pod["spec"]["nodeName"]
        .as_str()
        .is_some_and(|n| !n.is_empty());
    if !scheduled || matches!(phase, "Succeeded" | "Failed") {
        return None;
    }
    // gracePeriodSeconds=0 is a forced deletion (kubectl sends it for
    // --force; without --force it turns 0 into 1 itself).
    let grace = requested
        .or_else(|| pod["spec"]["terminationGracePeriodSeconds"].as_i64())
        .unwrap_or(30);
    (grace > 0).then_some(grace)
}
