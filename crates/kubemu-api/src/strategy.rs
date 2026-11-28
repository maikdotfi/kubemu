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

/// `AllowCreateOnUpdate`: PUT to a missing object creates it.
pub fn allow_create_on_update(r: &ResourceInfo) -> bool {
    matches!(
        (r.group, r.plural),
        ("", "endpoints" | "events") | ("discovery.k8s.io", "endpointslices")
    )
}

/// Whether DELETE answers with the deleted object rather than a `Status`.
pub fn returns_deleted_object(r: &ResourceInfo) -> bool {
    is(r, "", "pods")
}

/// Whether `metadata.generation` is maintained (bumped on spec changes).
pub fn tracks_generation(r: &ResourceInfo) -> bool {
    is(r, "", "pods")
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

/// `PrepareForUpdate`: `new` becomes what is stored. The main resource
/// cannot change status; `/status` changes nothing but status.
pub fn prepare_for_update(r: &ResourceInfo, new: &mut Value, old: &Value, status: bool) {
    if status {
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
    if is(r, "", "namespaces") {
        // spec.finalizers changes only through /finalize.
        new["spec"] = old.get("spec").cloned().unwrap_or(json!({}));
    }
    if tracks_generation(r) {
        let g = old["metadata"]["generation"].as_i64().unwrap_or(1);
        let bump = new.get("spec") != old.get("spec");
        meta_mut(new).insert("generation".into(), json!(if bump { g + 1 } else { g }));
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
