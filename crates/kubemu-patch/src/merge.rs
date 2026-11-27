//! JSON merge patch (RFC 7386) as Kubernetes applies it.
//!
//! The apiserver uses `gopkg.in/evanphx/json-patch.v4` `MergePatch`, which
//! differs from the RFC in a few places. We match the library:
//! - Nulls are pruned only from patch values that are inserted fresh (the
//!   key was missing or null) or that replace a non-object. A non-object
//!   patch value replacing an object is taken as is, nulls and all.
//! - Pruning removes null members of objects but keeps null array elements.
//! - A top-level array patch is returned (pruned) whatever the document is;
//!   a scalar or null patch is an error, and so is a null document.

use std::fmt;

use serde_json::{Map, Value};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MergePatchError {
    BadDocument,
    BadPatch,
}

impl fmt::Display for MergePatchError {
    /// Same text as the Go library, so the oracle can compare errors.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::BadDocument => "Invalid JSON Document",
            Self::BadPatch => "Invalid JSON Patch",
        })
    }
}

impl std::error::Error for MergePatchError {}

/// Applies `patch` to `doc` like `jsonpatch.MergePatch(doc, patch)`.
pub fn merge_patch(doc: &Value, patch: &Value) -> Result<Value, MergePatchError> {
    if doc.is_null() {
        return Err(MergePatchError::BadDocument);
    }
    match (doc, patch) {
        (Value::Object(d), Value::Object(p)) => {
            let mut d = d.clone();
            merge_docs(&mut d, p);
            Ok(Value::Object(d))
        }
        (_, Value::Object(p)) => {
            let mut p = p.clone();
            prune_object(&mut p);
            Ok(Value::Object(p))
        }
        (_, Value::Array(_)) => {
            let mut p = patch.clone();
            prune(&mut p);
            Ok(p)
        }
        _ => Err(MergePatchError::BadPatch),
    }
}

fn merge_docs(doc: &mut Map<String, Value>, patch: &Map<String, Value>) {
    for (k, v) in patch {
        if v.is_null() {
            doc.remove(k);
            continue;
        }
        match doc.get_mut(k) {
            Some(cur) if !cur.is_null() => merge(cur, v),
            _ => {
                let mut v = v.clone();
                prune(&mut v);
                doc.insert(k.clone(), v);
            }
        }
    }
}

/// `merge(cur, patch)` from the library, writing the result into `cur`.
fn merge(cur: &mut Value, patch: &Value) {
    match (cur.as_object_mut(), patch) {
        (Some(c), Value::Object(p)) => merge_docs(c, p),
        (Some(_), _) => *cur = patch.clone(),
        (None, _) => {
            let mut p = patch.clone();
            prune(&mut p);
            *cur = p;
        }
    }
}

fn prune(v: &mut Value) {
    match v {
        Value::Object(o) => prune_object(o),
        Value::Array(a) => a.iter_mut().for_each(prune),
        _ => {}
    }
}

fn prune_object(o: &mut Map<String, Value>) {
    o.retain(|_, v| !v.is_null());
    o.values_mut().for_each(prune);
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn rfc_basics() {
        let doc = json!({"a": 1, "b": {"c": 2, "d": 3}, "e": [1]});
        let patch = json!({"a": null, "b": {"c": null, "x": {"y": null}}, "e": [null]});
        assert_eq!(
            merge_patch(&doc, &patch).unwrap(),
            json!({"b": {"d": 3, "x": {}}, "e": [null]})
        );
    }

    #[test]
    fn library_quirks() {
        // A non-object replacing an object keeps its nulls.
        assert_eq!(
            merge_patch(&json!({"a": {}}), &json!({"a": [{"b": null}]})).unwrap(),
            json!({"a": [{"b": null}]})
        );
        // Replacing a non-object prunes.
        assert_eq!(
            merge_patch(&json!({"a": 1}), &json!({"a": [{"b": null}]})).unwrap(),
            json!({"a": [{}]})
        );
        // Top-level array patch wins over any document.
        assert_eq!(
            merge_patch(&json!({"a": 1}), &json!([{"b": null}, null])).unwrap(),
            json!([{}, null])
        );
        // A non-object document is replaced by the pruned patch.
        assert_eq!(
            merge_patch(&json!([1]), &json!({"a": null, "b": 1})).unwrap(),
            json!({"b": 1})
        );
    }

    #[test]
    fn errors() {
        assert_eq!(
            merge_patch(&Value::Null, &json!({})),
            Err(MergePatchError::BadDocument)
        );
        assert_eq!(
            merge_patch(&json!({}), &Value::Null),
            Err(MergePatchError::BadPatch)
        );
        assert_eq!(
            merge_patch(&json!({}), &json!(1)),
            Err(MergePatchError::BadPatch)
        );
    }
}
