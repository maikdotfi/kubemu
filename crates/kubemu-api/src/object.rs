//! Helpers on JSON objects: metadata access and storage keys.

use std::sync::Arc;

use kubemu_api_types::ResourceInfo;
use kubemu_store::ObjectKey;
use serde_json::{Map, Value, json};

/// What the store holds. Every stored object carries its own
/// `metadata.resourceVersion`, so a previous state is self-describing.
pub type Obj = Arc<Value>;

/// The store's resource key: `pods`, or `endpointslices.discovery.k8s.io`.
pub fn storage_resource(r: &ResourceInfo) -> String {
    if r.group.is_empty() {
        r.plural.to_owned()
    } else {
        format!("{}.{}", r.plural, r.group)
    }
}

pub fn key(r: &ResourceInfo, namespace: &str, name: &str) -> ObjectKey {
    ObjectKey::new(storage_resource(r), namespace, name)
}

pub fn meta(v: &Value) -> Option<&Map<String, Value>> {
    v.get("metadata").and_then(Value::as_object)
}

pub fn meta_mut(v: &mut Value) -> &mut Map<String, Value> {
    if !v.get("metadata").is_some_and(Value::is_object) {
        v["metadata"] = json!({});
    }
    v["metadata"].as_object_mut().expect("just ensured")
}

pub fn meta_str<'a>(v: &'a Value, field: &str) -> Option<&'a str> {
    meta(v).and_then(|m| m.get(field)).and_then(Value::as_str)
}

pub fn name(v: &Value) -> &str {
    meta_str(v, "name").unwrap_or_default()
}

pub fn namespace(v: &Value) -> &str {
    meta_str(v, "namespace").unwrap_or_default()
}

pub fn uid(v: &Value) -> Option<&str> {
    meta_str(v, "uid")
}

pub fn labels(v: &Value) -> Option<&Value> {
    meta(v).and_then(|m| m.get("labels"))
}

pub fn finalizers(v: &Value) -> Vec<String> {
    meta(v)
        .and_then(|m| m.get("finalizers"))
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

pub fn is_deleting(v: &Value) -> bool {
    meta(v).is_some_and(|m| m.get("deletionTimestamp").is_some_and(|t| !t.is_null()))
}

pub fn resource_version(v: &Value) -> Option<&str> {
    meta_str(v, "resourceVersion").filter(|s| !s.is_empty())
}

pub fn set_resource_version(v: &mut Value, rv: u64) {
    meta_mut(v).insert("resourceVersion".into(), json!(rv.to_string()));
}

/// A copy of `v` with `metadata.resourceVersion` set: DELETED watch events
/// carry the deletion's resourceVersion.
pub fn with_resource_version(v: &Value, rv: u64) -> Value {
    let mut v = v.clone();
    set_resource_version(&mut v, rv);
    v
}

/// Drops `kind` and `apiVersion`, as items of a typed list have none.
pub fn as_list_item(v: &Value) -> Value {
    let mut v = v.clone();
    if let Some(m) = v.as_object_mut() {
        m.remove("kind");
        m.remove("apiVersion");
    }
    v
}
