//! JSON patch (RFC 6902) as Kubernetes applies it.
//!
//! The apiserver uses `gopkg.in/evanphx/json-patch.v4` (`DecodePatch`, then
//! `Apply`), which differs from the RFC in many places. We follow the
//! library, not the RFC:
//! - `replace` of a missing object member adds it; only arrays check.
//! - Negative array indices count from the end (`-1` is the last element;
//!   for `add`, `-1` appends).
//! - `test` treats a missing member and `null` alike, and compares scalars
//!   by their JSON text (so `1` and `1.0` differ).
//! - `copy` of a missing member copies `null`.
//! - A path's first segment is ignored (`a/b` addresses `/b`), and only
//!   `replace` and `test` accept the root path `""`.
//!
//! The library keeps values lazily and distinguishes states the JSON text
//! does not; they are modelled explicitly in [`Node`]. Where the library
//! panics (nil map writes, comparing arrays holding `null`), the apiserver
//! returns a 500; we return an error.
//!
//! Error texts are close to the library's but not guaranteed identical.

use std::collections::BTreeMap;
use std::fmt;

use serde_json::{Map, Value};

/// The apiserver rejects longer patches (`maxJSONPatchOperations`).
pub const MAX_OPERATIONS: usize = 10_000;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JsonPatchError {
    /// The patch is not a JSON array of objects (HTTP 400).
    Decode(String),
    /// More than [`MAX_OPERATIONS`] operations (HTTP 413).
    TooLarge(usize),
    /// An operation failed (HTTP 422).
    Apply(String),
}

impl fmt::Display for JsonPatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Decode(e) | Self::Apply(e) => f.write_str(e),
            Self::TooLarge(n) => write!(
                f,
                "The allowed maximum operations in a JSON patch is {MAX_OPERATIONS}, got {n}"
            ),
        }
    }
}

impl std::error::Error for JsonPatchError {}

/// A value slot as the library holds it. `None` in a container is a nil
/// pointer: a `null` read from the document, or an `add` without `value`.
type Link = Option<Node>;

#[derive(Clone, Debug)]
enum Node {
    /// A node whose raw JSON is nil: `"value": null` in an operation.
    /// Unlike a nil pointer, it cannot be walked through and `test`s equal
    /// only to another `null`.
    RawNull,
    /// A non-null scalar, as JSON text. Comparisons use the text, and
    /// copies re-encode it with Go's HTML escaping.
    Scalar(String),
    /// An object. `None` is a nil map: what a copied `null` becomes when a
    /// path walks through it. Reads find nothing; writes panic.
    Doc(Option<BTreeMap<String, Link>>),
    Ary(Vec<Link>),
}

impl Node {
    /// A value nested in a document or an operation value.
    fn link(v: &Value) -> Link {
        match v {
            Value::Null => None,
            _ => Some(Self::top(v)),
        }
    }

    /// A value at the top of a lazy node (an operation's `value`, or a
    /// deep copy): `null` there is raw JSON text, not a nil pointer.
    fn top(v: &Value) -> Self {
        match v {
            Value::Null => Self::Doc(None),
            Value::Object(m) => Self::Doc(Some(
                m.iter().map(|(k, v)| (k.clone(), Self::link(v))).collect(),
            )),
            Value::Array(a) => Self::Ary(a.iter().map(Self::link).collect()),
            _ => Self::Scalar(v.to_string()),
        }
    }

    fn to_value(link: &Link) -> Value {
        match link {
            None | Some(Self::RawNull) | Some(Self::Doc(None)) => Value::Null,
            Some(Self::Scalar(raw)) => serde_json::from_str(raw).expect("scalar text is JSON"),
            Some(Self::Doc(Some(m))) => Value::Object(
                m.iter()
                    .map(|(k, v)| (k.clone(), Self::to_value(v)))
                    .collect::<Map<_, _>>(),
            ),
            Some(Self::Ary(a)) => Value::Array(a.iter().map(Self::to_value).collect()),
        }
    }

    /// `deepCopy`: marshal (escaping HTML characters in strings, as
    /// `json.Marshal` does) and wrap the text in a fresh lazy node.
    fn deep_copy(link: &Link) -> Link {
        link.as_ref().map(|n| match n {
            Self::RawNull | Self::Doc(None) => Self::Doc(None),
            n => n.copy_nested(),
        })
    }

    /// A copy as re-reading the marshalled text gives it: nested `null`s
    /// become nil pointers.
    fn copy_link(link: &Link) -> Link {
        match link {
            None | Some(Self::RawNull) | Some(Self::Doc(None)) => None,
            Some(n) => Some(n.copy_nested()),
        }
    }

    fn copy_nested(&self) -> Self {
        match self {
            Self::Scalar(raw) => Self::Scalar(html_escape(raw)),
            Self::Doc(Some(m)) => Self::Doc(Some(
                m.iter()
                    .map(|(k, v)| (k.clone(), Self::copy_link(v)))
                    .collect(),
            )),
            Self::Ary(a) => Self::Ary(a.iter().map(Self::copy_link).collect()),
            Self::RawNull | Self::Doc(None) => unreachable!("handled by callers"),
        }
    }

    /// The object view `tryDoc` would give, if any.
    fn as_doc(&self) -> Option<Option<&BTreeMap<String, Link>>> {
        match self {
            Self::Doc(m) => Some(m.as_ref()),
            _ => None,
        }
    }

    /// `lazyNode.equal`. A comparison the library would panic on counts as
    /// unequal: both fail the operation.
    fn equal(&self, o: &Node) -> bool {
        match self {
            Self::RawNull | Self::Scalar(_) => match (self, o) {
                (Self::RawNull, Self::RawNull) => true,
                (Self::Scalar(a), Self::Scalar(b)) => a == b,
                _ => false,
            },
            Self::Doc(n) => {
                let Some(o) = o.as_doc() else { return false };
                let (n, o) = (n.as_ref(), o);
                let len = |m: Option<&BTreeMap<String, Link>>| m.map_or(0, BTreeMap::len);
                if len(n) != len(o) {
                    return false;
                }
                n.into_iter()
                    .flatten()
                    .all(|(k, v)| match (v, o.and_then(|o| o.get(k))) {
                        (_, None) => false,
                        (None, Some(None)) => true,
                        (Some(v), Some(Some(ov))) => v.equal(ov),
                        _ => false,
                    })
            }
            Self::Ary(n) => {
                let o = match o {
                    Self::Ary(a) => a.as_slice(),
                    // `tryAry` on a raw `null` gives a nil slice; only a
                    // nil-map node can still hold that text here.
                    Self::Doc(None) => &[],
                    _ => return false,
                };
                n.len() == o.len()
                    && n.iter().zip(o).all(|(a, b)| match (a, b) {
                        (Some(a), Some(b)) => a.equal(b),
                        // The library dereferences the nil element: panic.
                        _ => false,
                    })
            }
        }
    }
}

/// What `encoding/json` does to text it re-encodes: `<`, `>`, `&`, U+2028
/// and U+2029 become `\u` escapes.
fn html_escape(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for c in raw.chars() {
        match c {
            '<' => out.push_str("\\u003c"),
            '>' => out.push_str("\\u003e"),
            '&' => out.push_str("\\u0026"),
            '\u{2028}' => out.push_str("\\u2028"),
            '\u{2029}' => out.push_str("\\u2029"),
            c => out.push(c),
        }
    }
    out
}

/// `strconv.Atoi`.
fn atoi(s: &str) -> Result<i64, String> {
    s.parse::<i64>()
        .map_err(|_| format!("strconv.Atoi: parsing {s:?}: invalid syntax"))
}

fn invalid_index(idx: i64) -> String {
    format!("Unable to access invalid index: {idx}: invalid index referenced")
}

/// Normalises an index the way `get`, `set` and `remove` do.
fn norm_index(idx: i64, len: usize) -> Result<usize, String> {
    let len = len as i64;
    let i = if idx < 0 {
        if idx < -len {
            return Err(invalid_index(idx));
        }
        idx + len
    } else {
        idx
    };
    Ok(i as usize)
}

const NIL_MAP: &str = "assignment to entry in nil map";

impl Node {
    fn get(&self, key: &str) -> Result<Option<&Node>, String> {
        match self {
            Self::Doc(m) => Ok(m.as_ref().and_then(|m| m.get(key)).and_then(Option::as_ref)),
            Self::Ary(a) => {
                let idx = atoi(key)?;
                let i = norm_index(idx, a.len())?;
                match a.get(i) {
                    Some(l) => Ok(l.as_ref()),
                    None => Err(invalid_index(i as i64)),
                }
            }
            _ => unreachable!("containers are docs or arrays"),
        }
    }

    /// Like [`get`](Self::get) but returns the slot, for `move`.
    fn take(&mut self, key: &str) -> Result<Link, String> {
        Ok(self.get(key)?.cloned())
    }

    fn set(&mut self, key: &str, val: Link) -> Result<(), String> {
        match self {
            Self::Doc(None) => Err(NIL_MAP.into()),
            Self::Doc(Some(m)) => {
                m.insert(key.to_owned(), val);
                Ok(())
            }
            Self::Ary(a) => {
                let i = norm_index(atoi(key)?, a.len())?;
                a[i] = val;
                Ok(())
            }
            _ => unreachable!(),
        }
    }

    fn add(&mut self, key: &str, val: Link) -> Result<(), String> {
        match self {
            Self::Doc(_) => self.set(key, val),
            Self::Ary(a) => {
                if key == "-" {
                    a.push(val);
                    return Ok(());
                }
                let idx = atoi(key)
                    .map_err(|e| format!("value was not a proper array index: '{key}': {e}"))?;
                let size = a.len() as i64 + 1;
                if idx >= size {
                    return Err(invalid_index(idx));
                }
                let i = if idx < 0 {
                    if idx < -size {
                        return Err(invalid_index(idx));
                    }
                    idx + size
                } else {
                    idx
                };
                a.insert(i as usize, val);
                Ok(())
            }
            _ => unreachable!(),
        }
    }

    fn remove(&mut self, key: &str) -> Result<(), String> {
        match self {
            Self::Doc(m) => match m.as_mut().and_then(|m| m.remove(key)) {
                Some(_) => Ok(()),
                None => Err(format!(
                    "Unable to remove nonexistent key: {key}: missing value"
                )),
            },
            Self::Ary(a) => {
                let idx = atoi(key)?;
                if idx >= a.len() as i64 {
                    return Err(invalid_index(idx));
                }
                let i = norm_index(idx, a.len())?;
                a.remove(i);
                Ok(())
            }
            _ => unreachable!(),
        }
    }
}

/// RFC 6901 unescaping in one left-to-right pass, like `strings.Replacer`.
fn decode_key(k: &str) -> String {
    let mut out = String::with_capacity(k.len());
    let mut rest = k;
    while let Some(i) = rest.find('~') {
        out.push_str(&rest[..i]);
        let tail = &rest[i..];
        if let Some(t) = tail.strip_prefix("~1") {
            out.push('/');
            rest = t;
        } else if let Some(t) = tail.strip_prefix("~0") {
            out.push('~');
            rest = t;
        } else {
            out.push('~');
            rest = &tail[1..];
        }
    }
    out.push_str(rest);
    out
}

/// `findObject`: the container holding the last path segment, and that
/// segment, unescaped.
fn find<'a>(root: &'a mut Node, path: &str) -> Option<(&'a mut Node, String)> {
    let split: Vec<&str> = path.split('/').collect();
    if split.len() < 2 {
        return None;
    }
    let mut cur = root;
    for part in &split[1..split.len() - 1] {
        let key = decode_key(part);
        let next = match cur {
            Node::Doc(m) => m.as_mut()?.get_mut(&key)?.as_mut()?,
            Node::Ary(a) => {
                let i = norm_index(atoi(&key).ok()?, a.len()).ok()?;
                a.get_mut(i)?.as_mut()?
            }
            _ => return None,
        };
        match next {
            Node::Doc(_) | Node::Ary(_) => cur = next,
            Node::RawNull | Node::Scalar(_) => return None,
        }
    }
    Some((cur, decode_key(split[split.len() - 1])))
}

struct Op<'a>(&'a Map<String, Value>);

impl Op<'_> {
    fn kind(&self) -> &str {
        self.0
            .get("op")
            .and_then(Value::as_str)
            .unwrap_or("unknown")
    }

    fn string(&self, field: &str) -> Result<&str, String> {
        match self.0.get(field) {
            None | Some(Value::Null) => {
                Err(format!("operation missing {field} field: missing value"))
            }
            Some(Value::String(s)) => Ok(s),
            Some(other) => Err(format!(
                "json: cannot unmarshal {} into Go value of type string",
                go_kind(other)
            )),
        }
    }

    /// `Operation.value()`: `None` without a `value` member.
    fn value(&self) -> Option<Node> {
        self.0.get("value").map(|v| match v {
            Value::Null => Node::RawNull,
            v => Node::top(v),
        })
    }
}

fn go_kind(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

/// Applies a JSON patch to `doc` like `jsonpatch.DecodePatch(patch)` then
/// `Apply(doc)`.
pub fn json_patch(doc: &Value, patch: &Value) -> Result<Value, JsonPatchError> {
    let ops = decode(patch)?;
    let mut root = match doc {
        Value::Array(a) => Node::Ary(a.iter().map(Node::link).collect()),
        Value::Object(_) | Value::Null => Node::top(doc),
        other => {
            return Err(JsonPatchError::Apply(format!(
                "json: cannot unmarshal {} into Go value of type jsonpatch.partialDoc",
                go_kind(other)
            )));
        }
    };
    for op in ops {
        apply(&mut root, op.map(Op)).map_err(JsonPatchError::Apply)?;
    }
    Ok(Node::to_value(&Some(root)))
}

/// `DecodePatch`: an array of objects (or nulls, which fail when applied).
fn decode(patch: &Value) -> Result<Vec<Option<&Map<String, Value>>>, JsonPatchError> {
    let Value::Array(ops) = patch else {
        return Err(JsonPatchError::Decode(format!(
            "json: cannot unmarshal {} into Go value of type jsonpatch.Patch",
            go_kind(patch)
        )));
    };
    let ops = ops
        .iter()
        .map(|op| match op {
            Value::Object(m) => Ok(Some(m)),
            Value::Null => Ok(None),
            other => Err(JsonPatchError::Decode(format!(
                "json: cannot unmarshal {} into Go value of type jsonpatch.Operation",
                go_kind(other)
            ))),
        })
        .collect::<Result<Vec<_>, _>>()?;
    if ops.len() > MAX_OPERATIONS {
        return Err(JsonPatchError::TooLarge(ops.len()));
    }
    Ok(ops)
}

fn apply(root: &mut Node, op: Option<Op<'_>>) -> Result<(), String> {
    let Some(op) = op else {
        return Err("Unexpected kind: unknown".into());
    };
    match op.kind() {
        "add" => {
            let path = op
                .string("path")
                .map_err(|_| "add operation failed to decode path: missing value".to_owned())?;
            let (con, key) = find(root, path).ok_or_else(|| missing("add", path))?;
            con.add(&key, op.value())
                .map_err(|e| format!("error in add for path: '{path}': {e}"))
        }
        "remove" => {
            let path = op
                .string("path")
                .map_err(|_| "remove operation failed to decode path: missing value".to_owned())?;
            let (con, key) = find(root, path).ok_or_else(|| missing("remove", path))?;
            con.remove(&key)
                .map_err(|e| format!("error in remove for path: '{path}': {e}"))
        }
        "replace" => {
            let path = op.string("path")?;
            if path.is_empty() {
                return match op.value() {
                    // The library dereferences the missing value: panic.
                    None => Err("invalid memory address or nil pointer dereference".into()),
                    Some(v @ (Node::Doc(Some(_)) | Node::Ary(_))) => {
                        *root = v;
                        Ok(())
                    }
                    Some(_) => Err("replace operation value must be object or array".into()),
                };
            }
            let (con, key) = find(root, path).ok_or_else(|| {
                format!(
                    "replace operation does not apply: doc is missing path: {path}: missing value"
                )
            })?;
            con.get(&key).map_err(|_| {
                format!(
                    "replace operation does not apply: doc is missing key: {path}: missing value"
                )
            })?;
            con.set(&key, op.value())
                .map_err(|e| format!("error in remove for path: '{path}': {e}"))
        }
        "move" => {
            let from = op.string("from")?;
            let (con, key) = find(root, from).ok_or_else(|| {
                format!(
                    "move operation does not apply: doc is missing from path: {from}: missing value"
                )
            })?;
            let val = con
                .take(&key)
                .map_err(|e| format!("error in move for path: '{key}': {e}"))?;
            con.remove(&key)
                .map_err(|e| format!("error in move for path: '{key}': {e}"))?;
            let path = op.string("path")?;
            let (con, key) = find(root, path).ok_or_else(|| {
                format!("move operation does not apply: doc is missing destination path: {path}: missing value")
            })?;
            con.add(&key, val)
                .map_err(|e| format!("error in move for path: '{path}': {e}"))
        }
        "test" => {
            let path = op.string("path")?;
            let failed = || format!("testing value {path} failed: test failed");
            let want = op.value();
            if path.is_empty() {
                return match want {
                    Some(w) if root.equal(&w) => Ok(()),
                    _ => Err(failed()),
                };
            }
            let (con, key) = find(root, path).ok_or_else(|| {
                format!("test operation does not apply: is missing path: {path}: missing value")
            })?;
            let have = con
                .get(&key)
                .map_err(|e| format!("error in test for path: '{path}': {e}"))?;
            match (have, want) {
                (None, None | Some(Node::RawNull)) => Ok(()),
                (Some(h), Some(w)) if h.equal(&w) => Ok(()),
                _ => Err(failed()),
            }
        }
        "copy" => {
            let from = op.string("from")?;
            let (con, key) = find(root, from).ok_or_else(|| {
                format!(
                    "copy operation does not apply: doc is missing from path: {from}: missing value"
                )
            })?;
            let val = Node::deep_copy(
                &con.get(&key)
                    .map_err(|e| format!("error in copy for from: '{from}': {e}"))?
                    .cloned(),
            );
            let path = op
                .string("path")
                .map_err(|_| "copy operation failed to decode path: missing value".to_owned())?;
            let (con, key) = find(root, path).ok_or_else(|| {
                format!("copy operation does not apply: doc is missing destination path: {path}: missing value")
            })?;
            con.add(&key, val)
                .map_err(|e| format!("error while adding value during copy: {e}"))
        }
        other => Err(format!("Unexpected kind: {other}")),
    }
}

fn missing(op: &str, path: &str) -> String {
    format!("{op} operation does not apply: doc is missing path: \"{path}\": missing value")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn ok(doc: Value, patch: Value) -> Value {
        json_patch(&doc, &patch).unwrap()
    }

    fn err(doc: Value, patch: Value) {
        assert!(json_patch(&doc, &patch).is_err(), "{doc} {patch}");
    }

    #[test]
    fn rfc_operations() {
        let doc = json!({"a": {"b": [1, 2]}, "c": "x"});
        assert_eq!(
            ok(
                doc.clone(),
                json!([
                    {"op": "add", "path": "/a/b/1", "value": 9},
                    {"op": "remove", "path": "/c"},
                    {"op": "replace", "path": "/a/b/0", "value": {"k": null}},
                    {"op": "copy", "from": "/a/b", "path": "/d"},
                    {"op": "move", "from": "/d", "path": "/e"},
                    {"op": "test", "path": "/e/1", "value": 9},
                ])
            ),
            json!({"a": {"b": [{"k": null}, 9, 2]}, "e": [{"k": null}, 9, 2]})
        );
        err(
            doc.clone(),
            json!([{"op": "test", "path": "/c", "value": "y"}]),
        );
        err(doc, json!([{"op": "bogus", "path": "/c"}]));
    }

    #[test]
    fn library_quirks() {
        // replace on a missing member adds it.
        assert_eq!(
            ok(
                json!({}),
                json!([{"op": "replace", "path": "/a", "value": 1}])
            ),
            json!({"a": 1})
        );
        // Negative indices count from the end.
        assert_eq!(
            ok(
                json!({"l": [1, 2, 3]}),
                json!([{"op": "remove", "path": "/l/-1"}])
            ),
            json!({"l": [1, 2]})
        );
        assert_eq!(
            ok(
                json!({"l": [1]}),
                json!([{"op": "add", "path": "/l/-1", "value": 2}])
            ),
            json!({"l": [1, 2]})
        );
        // test: missing equals null.
        ok(
            json!({}),
            json!([{"op": "test", "path": "/a", "value": null}]),
        );
        ok(json!({"a": null}), json!([{"op": "test", "path": "/a"}]));
        // test compares scalar text.
        err(
            json!({"a": 1}),
            json!([{"op": "test", "path": "/a", "value": 1.0}]),
        );
        // copy of a missing member copies null.
        assert_eq!(
            ok(
                json!({}),
                json!([{"op": "copy", "from": "/x", "path": "/y"}])
            ),
            json!({"y": null})
        );
        // A copied raw null walks like an empty, unwritable object.
        err(
            json!({}),
            json!([
                {"op": "add", "path": "/a", "value": null},
                {"op": "copy", "from": "/a", "path": "/b"},
                {"op": "add", "path": "/b/c", "value": 1},
            ]),
        );
        // Copies are re-encoded with HTML escaping, and test compares text.
        err(
            json!({"a": "<"}),
            json!([
                {"op": "copy", "from": "/a", "path": "/b"},
                {"op": "test", "path": "/b", "value": "<"},
            ]),
        );
        assert_eq!(
            ok(
                json!({"a": {"b": "<"}}),
                json!([{"op": "copy", "from": "/a", "path": "/c"}])
            ),
            json!({"a": {"b": "<"}, "c": {"b": "<"}})
        );
        // The first path segment is ignored.
        assert_eq!(
            ok(json!({}), json!([{"op": "add", "path": "x/y", "value": 1}])),
            json!({"y": 1})
        );
        // Root: only replace and test.
        assert_eq!(
            ok(
                json!({"a": 1}),
                json!([{"op": "replace", "path": "", "value": [1]}])
            ),
            json!([1])
        );
        err(json!({}), json!([{"op": "add", "path": "", "value": 1}]));
        // Escapes.
        assert_eq!(
            ok(
                json!({}),
                json!([{"op": "add", "path": "/a~1b~0c~01", "value": 1}])
            ),
            json!({"a/b~c~1": 1})
        );
    }

    #[test]
    fn decode_errors() {
        assert!(matches!(
            json_patch(&json!({}), &json!({})),
            Err(JsonPatchError::Decode(_))
        ));
        assert!(matches!(
            json_patch(&json!({}), &json!([1])),
            Err(JsonPatchError::Decode(_))
        ));
        assert!(matches!(
            json_patch(&json!({}), &json!([null])),
            Err(JsonPatchError::Apply(_))
        ));
    }
}
