//! Strategic merge patch, ported from `k8s.io/apimachinery`
//! `strategicpatch.StrategicMergeMapPatch` (the apply side only), as the
//! apiserver's PATCH handler calls it.
//!
//! Patch metadata (`patchStrategy`, `patchMergeKey`) comes from the
//! vendored OpenAPI schema (ADR 0012) instead of Go struct tags; the
//! extensions are generated from those tags, and the differential fuzz
//! checks they agree.
//!
//! The port keeps Go's behaviour where it leaks into results:
//! - values are typed as the apiserver decodes them: integers are `int64`,
//!   other numbers `float64`, and the two never compare equal;
//! - `==` on two maps or two lists panics in Go, and so does using one as
//!   a map key; the apiserver turns the panic into an error, and so do we;
//! - `sort.SliceStable` is ported exactly ([`stable_sort`]), because the
//!   comparator used for element order is not a strict weak order and the
//!   result depends on the algorithm;
//! - two in-place slice mutations that change the result are modelled
//!   (see [`Merged::original_after`]).
//!
//! Lookups differ from Go's struct-tag lookups in one unreachable case:
//! Go treats any pointer field (`*metav1.Time`, `*int32`) as a list when
//! both sides hold one there. The apiserver's original is a typed object,
//! so it never holds a list at a non-list field.
//!
//! One deliberate difference: Go walks patch maps in random order. Results
//! depend on the order only when a patch has both `$deleteFromPrimitiveList/k`
//! and `k` without `$setElementOrder/k`; we process keys in sorted order,
//! so the deletion list goes first.
//!
//! Error texts are close to Go's but not guaranteed identical.

use std::fmt;

use kubemu_api_types::{Schema, Ty};
use serde_json::{Map, Value};

const DIRECTIVE_MARKER: &str = "$patch";
const DELETE_DIRECTIVE: &str = "delete";
const REPLACE_DIRECTIVE: &str = "replace";
const MERGE_DIRECTIVE: &str = "merge";
const RETAIN_KEYS_STRATEGY: &str = "retainKeys";
const DELETE_FROM_PRIMITIVE_LIST_PREFIX: &str = "$deleteFromPrimitiveList";
const RETAIN_KEYS_DIRECTIVE: &str = "$retainKeys";
const SET_ELEMENT_ORDER_PREFIX: &str = "$setElementOrder";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SmpError(pub String);

impl fmt::Display for SmpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for SmpError {}

type R<T> = Result<T, SmpError>;

fn err<T>(msg: impl Into<String>) -> R<T> {
    Err(SmpError(msg.into()))
}

/// What Go panics on; the apiserver answers with an error.
fn panic<T>(what: &str) -> R<T> {
    err(format!("runtime error: {what}"))
}

/// The schema position a lookup is made from. `None` is a Go type with no
/// fields and no elements to look into (a byte in a `[]byte`).
type Meta = Option<Schema>;

/// Applies a strategic merge patch like `StrategicMergeMapPatch`, with the
/// patch metadata of `schema` (the object's kind).
pub fn strategic_merge_patch(original: &Value, patch: &Value, schema: Schema) -> R<Value> {
    let (Value::Object(o), Value::Object(p)) = (original, patch) else {
        return err("strategic merge patch needs a JSON object document and patch");
    };
    merge_map(o.clone(), p.clone(), Some(schema)).map(Value::Object)
}

// ---- Go value semantics -----------------------------------------------

/// The dynamic type of a decoded value (`reflect.TypeOf`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GoType {
    Nil,
    Bool,
    Int64,
    Float64,
    String,
    Slice,
    Map,
}

fn go_type(v: &Value) -> GoType {
    match v {
        Value::Null => GoType::Nil,
        Value::Bool(_) => GoType::Bool,
        Value::Number(n) if n.is_i64() => GoType::Int64,
        Value::Number(_) => GoType::Float64,
        Value::String(_) => GoType::String,
        Value::Array(_) => GoType::Slice,
        Value::Object(_) => GoType::Map,
    }
}

/// `a == b` on two `interface{}` values.
fn go_eq(a: &Value, b: &Value) -> R<bool> {
    let (ta, tb) = (go_type(a), go_type(b));
    if ta != tb {
        return Ok(false);
    }
    Ok(match ta {
        GoType::Slice | GoType::Map => return panic("comparing uncomparable type"),
        GoType::Nil => true,
        GoType::Bool | GoType::String => a == b,
        GoType::Int64 => a.as_i64() == b.as_i64(),
        GoType::Float64 => a.as_f64() == b.as_f64(),
    })
}

/// Using `v` as a key of a `map[interface{}]...`.
fn go_hashable(v: &Value) -> R<()> {
    match go_type(v) {
        GoType::Slice | GoType::Map => panic("hash of unhashable type"),
        _ => Ok(()),
    }
}

/// Whether `list` holds `v`, as a Go map built from `list` answers it.
fn go_set_contains(list: &[Value], v: &Value) -> R<bool> {
    go_hashable(v)?;
    for x in list {
        if go_eq(x, v)? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn go_set_check(list: &[Value]) -> R<()> {
    list.iter().try_for_each(go_hashable)
}

// ---- schema lookups ---------------------------------------------------

struct PatchMeta {
    strategies: Vec<String>,
    merge_key: String,
}

fn meta_of(field: Schema) -> PatchMeta {
    PatchMeta {
        strategies: field
            .patch_strategy()
            .unwrap_or("")
            .split(',')
            .map(str::to_owned)
            .collect(),
        merge_key: field.merge_key().unwrap_or("").to_owned(),
    }
}

/// `LookupPatchMetadataForStruct`.
fn lookup_struct(meta: Meta, key: &str) -> R<(Meta, PatchMeta)> {
    let Some(s) = meta else {
        return err("merging an object in json but data type is not struct");
    };
    match s.ty() {
        Ty::Object { .. } => match s.field(key) {
            Some(f) => Ok((Some(f), meta_of(f))),
            None => err(format!(
                "unable to find api field for the json field {key:?}"
            )),
        },
        // Go structs whose OpenAPI form has no properties: no such field.
        Ty::Quantity | Ty::IntOrString | Ty::Any => err(format!(
            "unable to find api field for the json field {key:?}"
        )),
        Ty::String if s.format() == Some("date-time") => err(format!(
            "unable to find api field for the json field {key:?}"
        )),
        _ => err("merging an object in json but data type is not struct"),
    }
}

/// `LookupPatchMetadataForSlice`.
fn lookup_slice(meta: Meta, key: &str) -> R<(Meta, PatchMeta)> {
    let (field, pm) = lookup_struct(meta, key)?;
    let field = field.expect("lookup_struct returns a schema");
    match field.ty() {
        Ty::Array(_) => Ok((field.items(), pm)),
        // `[]byte`: a slice of scalars.
        Ty::String if field.format() == Some("byte") => Ok((None, pm)),
        _ => err("expected slice or array type"),
    }
}

/// `extractRetainKeysPatchStrategy`: the strategy other than `retainKeys`.
fn strategy(pm: &PatchMeta) -> R<String> {
    let s = &pm.strategies;
    match s.len() {
        0 => Ok(String::new()),
        1 if s[0] == RETAIN_KEYS_STRATEGY => Ok(String::new()),
        1 => Ok(s[0].clone()),
        2 if s[0] == RETAIN_KEYS_STRATEGY => Ok(s[1].clone()),
        2 if s[1] == RETAIN_KEYS_STRATEGY => Ok(s[0].clone()),
        _ => err(format!("unexpected patch strategy: {s:?}")),
    }
}

// ---- maps -------------------------------------------------------------

fn merge_map(
    mut original: Map<String, Value>,
    mut patch: Map<String, Value>,
    schema: Meta,
) -> R<Map<String, Value>> {
    if let Some(v) = patch.get(DIRECTIVE_MARKER) {
        return handle_directive(v.clone(), patch);
    }
    apply_retain_keys(&mut original, &mut patch)?;
    merge_patch_into_original(&mut original, &mut patch, schema)?;

    for (k, patch_v) in patch {
        let (is_delete_list, k) = match k.strip_prefix(DELETE_FROM_PRIMITIVE_LIST_PREFIX) {
            Some(_) => {
                let key = extract_key(&k, DELETE_FROM_PRIMITIVE_LIST_PREFIX)?;
                (true, if key.is_empty() { k } else { key })
            }
            None => (false, k),
        };

        if patch_v.is_null() {
            original.remove(&k);
            continue;
        }

        let replace = match original.get(&k) {
            None => true,
            Some(o) => go_type(o) != go_type(&patch_v),
        };
        if replace {
            if !is_delete_list {
                let mut v = patch_v;
                discard_nulls(&mut v);
                match remove_directives(v) {
                    Some(v) => original.insert(k, v),
                    None => original.remove(&k),
                };
            }
            continue;
        }

        let cur = original.remove(&k).expect("checked above");
        let merged = match (cur, patch_v) {
            (Value::Object(o), Value::Object(p)) => {
                let (sub, pm) = lookup_struct(schema, &k)?;
                if strategy(&pm)? != REPLACE_DIRECTIVE {
                    Value::Object(merge_map(o, p, sub)?)
                } else {
                    Value::Object(p)
                }
            }
            (Value::Array(o), Value::Array(p)) => {
                let (sub, pm) = lookup_slice(schema, &k)?;
                let strategy = strategy(&pm)?;
                Value::Array(
                    merge_slice_handler(o, p, sub, &strategy, &pm.merge_key, is_delete_list)?
                        .merged,
                )
            }
            (_, p) => p,
        };
        original.insert(k, merged);
    }
    Ok(original)
}

fn handle_directive(directive: Value, mut patch: Map<String, Value>) -> R<Map<String, Value>> {
    if directive == REPLACE_DIRECTIVE {
        patch.remove(DIRECTIVE_MARKER);
        return Ok(patch);
    }
    if directive == DELETE_DIRECTIVE {
        return Ok(Map::new());
    }
    err(format!(
        "Unknown patch type: {directive} in map: {}",
        Value::Object(patch)
    ))
}

/// `extractKey`: the part after `prefix/`.
fn extract_key(s: &str, prefix: &str) -> R<String> {
    match s.split_once('/') {
        Some((p, rest)) if p == prefix => Ok(rest.to_owned()),
        _ => match prefix {
            DELETE_FROM_PRIMITIVE_LIST_PREFIX => err(
                "invalid patch format for a primitive list: the key must be $deleteFromPrimitiveList/<key>",
            ),
            SET_ELEMENT_ORDER_PREFIX => err(
                "invalid patch format for a $setElementOrder list: the key must be $setElementOrder/<key>",
            ),
            _ => err(format!("fail to find unknown prefix {prefix:?} in {s}")),
        },
    }
}

fn apply_retain_keys(original: &mut Map<String, Value>, patch: &mut Map<String, Value>) -> R<()> {
    let Some(retain) = patch.remove(RETAIN_KEYS_DIRECTIVE) else {
        return Ok(());
    };
    let Value::Array(retain) = retain else {
        return err("invalid patch format for retainKeys: the value must be a list");
    };
    go_set_check(&retain)?;
    let kept = |k: &str| retain.iter().any(|r| r.as_str() == Some(k));
    for (k, v) in patch.iter() {
        if v.is_null()
            || k.starts_with(DELETE_FROM_PRIMITIVE_LIST_PREFIX)
            || k.starts_with(SET_ELEMENT_ORDER_PREFIX)
        {
            continue;
        }
        if !kept(k) {
            return err(
                "invalid patch format for retainKeys: all fields in the patch must be in the list",
            );
        }
    }
    original.retain(|k, _| kept(k));
    Ok(())
}

fn discard_nulls(v: &mut Value) {
    match v {
        Value::Object(m) => {
            m.retain(|_, v| !v.is_null());
            m.values_mut().for_each(discard_nulls);
        }
        Value::Array(a) => a.iter_mut().for_each(discard_nulls),
        _ => {}
    }
}

/// `removeDirectives`: `None` when the value carries `$patch` and must go.
fn remove_directives(v: Value) -> Option<Value> {
    match v {
        Value::Object(m) => {
            if m.contains_key(DIRECTIVE_MARKER) {
                return None;
            }
            Some(Value::Object(
                m.into_iter()
                    .filter_map(|(k, v)| remove_directives(v).map(|v| (k, v)))
                    .collect(),
            ))
        }
        Value::Array(a) => Some(Value::Array(
            a.into_iter().filter_map(remove_directives).collect(),
        )),
        v => Some(v),
    }
}

/// `mergePatchIntoOriginal`: every `$setElementOrder/<key>` in the patch,
/// with the list it orders.
fn merge_patch_into_original(
    original: &mut Map<String, Value>,
    patch: &mut Map<String, Value>,
    schema: Meta,
) -> R<()> {
    let keys: Vec<String> = patch
        .keys()
        .filter(|k| k.starts_with(SET_ELEMENT_ORDER_PREFIX))
        .cloned()
        .collect();
    for key in keys {
        let Some(order) = patch.remove(&key) else {
            continue;
        };
        let Value::Array(order) = order else {
            return err(format!("expected []interface{{}}, got {order}"));
        };
        let original_key = extract_key(&key, SET_ELEMENT_ORDER_PREFIX)?;
        let original_list = match original.get(&original_key) {
            None => None,
            Some(Value::Array(a)) => Some(a.clone()),
            Some(other) => return err(format!("expected []interface{{}}, got {other}")),
        };
        let patch_list = match patch.get(&original_key) {
            None => None,
            Some(Value::Array(a)) => Some(a.clone()),
            Some(other) => return err(format!("expected []interface{{}}, got {other}")),
        };
        let (sub, pm) = lookup_slice(schema, &original_key)?;
        let strategy = strategy(&pm)?;
        let merge_key = pm.merge_key.as_str();
        validate_with_set_order(patch_list.as_deref().unwrap_or(&[]), &order, merge_key)?;

        // `originalFieldValue` as Go sees it after merging: the merge can
        // write through to the original list's backing array.
        let (merged, original_view) = match (original_list, patch_list.clone()) {
            (Some(o), None) => (o.clone(), o),
            (None, Some(p)) => (
                match remove_directives(Value::Array(p)) {
                    Some(Value::Array(a)) => a,
                    _ => unreachable!("lists always survive removeDirectives"),
                },
                Vec::new(),
            ),
            (Some(o), Some(p)) => {
                let m = merge_slice_handler(o, p, sub, &strategy, merge_key, false)?;
                (m.merged, m.original_after)
            }
            (None, None) => continue,
        };

        let (patch_items, server_only) = if merge_key.is_empty() {
            partition_primitives(&merged, &order)?
        } else {
            partition_maps(&merged, &order, merge_key)?
        };
        let kind = slice_element_type(&[&original_view, patch_list.as_deref().unwrap_or(&[])])?;
        let both = normalize_element_order(
            patch_items,
            server_only,
            &order,
            &original_view,
            merge_key,
            kind,
        )?;
        original.insert(original_key.clone(), Value::Array(both));
        patch.remove(&original_key);
    }
    Ok(())
}

fn validate_with_set_order(patch_list: &[Value], order: &[Value], merge_key: &str) -> R<()> {
    if order.is_empty() || patch_list.is_empty() {
        return Ok(());
    }
    let non_delete = if merge_key.is_empty() {
        patch_list.to_vec()
    } else {
        extract_to_delete_items(patch_list.to_vec())?.0
    };
    let (mut p, mut s) = (0, 0);
    while p < non_delete.len() && s < order.len() {
        if non_delete[p]
            .as_object()
            .is_some_and(|m| m.contains_key(DIRECTIVE_MARKER))
        {
            p += 1;
            continue;
        }
        if merge_key_value_equal(&non_delete[p], &order[s], merge_key)? {
            p += 1;
        }
        s += 1;
    }
    if p < non_delete.len() && s >= order.len() {
        return err("The order in patch list doesn't match $setElementOrder list");
    }
    Ok(())
}

fn merge_key_value_equal(left: &Value, right: &Value, merge_key: &str) -> R<bool> {
    if merge_key.is_empty() {
        return go_eq(left, right);
    }
    let (Value::Object(l), Value::Object(r)) = (left, right) else {
        return err("expected map[string]interface{}");
    };
    match (l.get(merge_key), r.get(merge_key)) {
        (Some(a), Some(b)) => go_eq(a, b),
        _ => err(format!(
            "map does not contain declared merge key: {merge_key}"
        )),
    }
}

// ---- slices -----------------------------------------------------------

struct Merged {
    merged: Vec<Value>,
    /// The original list as the caller's slice header sees it afterwards.
    /// Go merges into the original's backing array in two cases, and the
    /// caller reuses that view to order elements:
    /// - deleting elements of a list of maps shifts the rest left in
    ///   place, and appends may then fill the freed tail;
    /// - deduplicating a list of scalars when the patch adds nothing works
    ///   in place.
    original_after: Vec<Value>,
}

fn merge_slice_handler(
    original: Vec<Value>,
    patch: Vec<Value>,
    schema: Meta,
    strategy: &str,
    merge_key: &str,
    is_delete_list: bool,
) -> R<Merged> {
    if strategy == MERGE_DIRECTIVE || is_delete_list {
        merge_slice(original, patch, schema, merge_key, is_delete_list)
    } else {
        Ok(Merged {
            merged: patch,
            original_after: original,
        })
    }
}

/// `sliceElementType`, reduced to what callers use: whether elements are
/// maps.
fn slice_element_type(slices: &[&[Value]]) -> R<bool> {
    let mut prev = None;
    for v in slices.iter().flat_map(|s| s.iter()) {
        let t = go_type(v);
        match prev {
            None => {
                match t {
                    // `reflect.TypeOf(nil).Kind()`.
                    GoType::Nil => {
                        return panic("invalid memory address or nil pointer dereference");
                    }
                    GoType::Slice => return err("lists of lists are not supported"),
                    _ => {}
                }
                prev = Some(t);
            }
            Some(p) if p != t => return err("list element types are not identical"),
            Some(_) => {}
        }
    }
    match prev {
        None => err("no elements in any of the given slices"),
        Some(t) => Ok(t == GoType::Map),
    }
}

fn merge_slice(
    original: Vec<Value>,
    patch: Vec<Value>,
    schema: Meta,
    merge_key: &str,
    is_delete_list: bool,
) -> R<Merged> {
    if original.is_empty() && patch.is_empty() {
        return Ok(Merged {
            merged: original.clone(),
            original_after: original,
        });
    }
    let is_map = slice_element_type(&[&original, &patch])?;

    if !is_map {
        if is_delete_list {
            go_set_check(&patch)?;
            let mut kept = Vec::with_capacity(original.len());
            for v in &original {
                if !go_set_contains(&patch, v)? {
                    kept.push(v.clone());
                }
            }
            return Ok(Merged {
                merged: kept,
                original_after: original,
            });
        }
        // `append(original, patch...)` reallocates unless the patch is
        // empty; then the deduplication below runs on the original itself.
        let mut both = original.clone();
        both.extend(patch.iter().cloned());
        let merged = deduplicate_scalars(&mut both)?;
        let original_view = if patch.is_empty() { both } else { original };
        let (patch_items, server_only) = if merge_key.is_empty() {
            partition_primitives(&merged, &patch)?
        } else {
            partition_maps(&merged, &patch, merge_key)?
        };
        let merged = normalize_element_order(
            patch_items,
            server_only,
            &patch,
            &original_view,
            merge_key,
            false,
        )?;
        return Ok(Merged {
            merged,
            original_after: original_view,
        });
    }

    if merge_key.is_empty() {
        return err("cannot merge lists without merge key");
    }

    let mut s = BackedSlice::new(original);
    let mut replace = false;
    let mut rest = Vec::new();
    for v in &patch {
        let m = v.as_object().expect("all elements are maps");
        match m.get(DIRECTIVE_MARKER) {
            None => rest.push(v.clone()),
            Some(d) if d == DELETE_DIRECTIVE => match m.get(merge_key) {
                Some(mv) => s.delete_matching(merge_key, mv)?,
                None => {
                    return err(format!(
                        "map does not contain declared merge key: {merge_key}"
                    ));
                }
            },
            Some(d) if d == REPLACE_DIRECTIVE => replace = true,
            Some(d) if d == MERGE_DIRECTIVE => {
                return err("merging lists cannot yet be specified in the patch");
            }
            Some(d) => return err(format!("Unknown patch type: {d}")),
        }
    }
    let (merged, server_order, patch) = if replace {
        // The patch's own elements replace the list; nothing merges.
        (rest.clone(), rest, Vec::new())
    } else {
        let server_order_len = s.len;
        for v in &rest {
            let m = v.as_object().expect("all elements are maps");
            let Some(mv) = m.get(merge_key) else {
                return err(format!(
                    "map does not contain declared merge key: {merge_key}"
                ));
            };
            match find_map(s.view(), merge_key, mv)? {
                Some(i) => {
                    let Value::Object(o) = s.get(i).clone() else {
                        unreachable!()
                    };
                    let merged = merge_map(o, m.clone(), schema)?;
                    s.set(i, Value::Object(merged));
                }
                None => s.push(v.clone()),
            }
        }
        (
            s.view().to_vec(),
            s.backing[..server_order_len].to_vec(),
            rest,
        )
    };

    let (patch_items, server_only) = partition_maps(&merged, &patch, merge_key)?;
    let merged = normalize_element_order(
        patch_items,
        server_only,
        &patch,
        &server_order,
        merge_key,
        true,
    )?;
    Ok(Merged {
        merged,
        original_after: s.backing,
    })
}

/// A Go slice that starts out as the whole of an exactly-sized backing
/// array, so writes go through to it until an append reallocates.
struct BackedSlice {
    backing: Vec<Value>,
    len: usize,
    /// Set once an append outgrew the backing array.
    moved: Option<Vec<Value>>,
}

impl BackedSlice {
    fn new(v: Vec<Value>) -> Self {
        Self {
            len: v.len(),
            backing: v,
            moved: None,
        }
    }

    fn view(&self) -> &[Value] {
        match &self.moved {
            Some(m) => m,
            None => &self.backing[..self.len],
        }
    }

    fn get(&self, i: usize) -> &Value {
        &self.view()[i]
    }

    fn set(&mut self, i: usize, v: Value) {
        match &mut self.moved {
            Some(m) => m[i] = v,
            None => self.backing[i] = v,
        }
    }

    fn push(&mut self, v: Value) {
        match &mut self.moved {
            Some(m) => m.push(v),
            None if self.len < self.backing.len() => {
                self.backing[self.len] = v;
                self.len += 1;
            }
            None => {
                let mut m = self.backing[..self.len].to_vec();
                m.push(v);
                self.moved = Some(m);
            }
        }
    }

    /// `deleteMatchingEntries`: `append(s[:i], s[i+1:]...)` until no match.
    fn delete_matching(&mut self, merge_key: &str, value: &Value) -> R<()> {
        debug_assert!(self.moved.is_none());
        while let Some(i) = find_map(self.view(), merge_key, value)? {
            // `copy` shifts the tail left; the last slot keeps its value.
            for k in i..self.len - 1 {
                self.backing[k] = self.backing[k + 1].clone();
            }
            self.len -= 1;
        }
        Ok(())
    }
}

/// `findMapInSliceBasedOnKeyValue`: the index of the first map whose
/// `key` equals `value`.
fn find_map(list: &[Value], key: &str, value: &Value) -> R<Option<usize>> {
    for (i, v) in list.iter().enumerate() {
        let Value::Object(m) = v else {
            return err(format!("value for key {i} is not a map"));
        };
        if let Some(x) = m.get(key)
            && go_eq(x, value)?
        {
            return Ok(Some(i));
        }
    }
    Ok(None)
}

/// `deduplicateScalars`, in place on `s`; returns the deduplicated prefix.
fn deduplicate_scalars(s: &mut [Value]) -> R<Vec<Value>> {
    let mut length = s.len() as isize - 1;
    let mut i = 0isize;
    while i < length {
        let mut j = i + 1;
        while j <= length {
            if go_eq(&s[i as usize], &s[j as usize])? {
                s[j as usize] = s[length as usize].clone();
                length -= 1;
                j -= 1;
            }
            j += 1;
        }
        i += 1;
    }
    Ok(s[..(length + 1) as usize].to_vec())
}

fn partition_primitives(original: &[Value], by: &[Value]) -> R<(Vec<Value>, Vec<Value>)> {
    go_set_check(by)?;
    let (mut patch, mut server_only) = (Vec::new(), Vec::new());
    for v in original {
        if go_set_contains(by, v)? {
            patch.push(v.clone());
        } else {
            server_only.push(v.clone());
        }
    }
    Ok((patch, server_only))
}

fn partition_maps(
    original: &[Value],
    by: &[Value],
    merge_key: &str,
) -> R<(Vec<Value>, Vec<Value>)> {
    let (mut patch, mut server_only) = (Vec::new(), Vec::new());
    for v in original {
        let Value::Object(m) = v else {
            return err("expected map[string]interface{}");
        };
        let Some(mv) = m.get(merge_key) else {
            return err(format!(
                "map does not contain declared merge key: {merge_key}"
            ));
        };
        if find_map(by, merge_key, mv)?.is_some() {
            patch.push(v.clone());
        } else {
            server_only.push(v.clone());
        }
    }
    Ok((patch, server_only))
}

fn extract_to_delete_items(l: Vec<Value>) -> R<(Vec<Value>, Vec<Value>)> {
    let (mut non_delete, mut to_delete) = (Vec::new(), Vec::new());
    for v in l {
        let Value::Object(m) = &v else {
            return err("expected map[string]interface{}");
        };
        if m.get(DIRECTIVE_MARKER)
            .is_some_and(|d| d == DELETE_DIRECTIVE)
        {
            to_delete.push(v);
        } else {
            non_delete.push(v);
        }
    }
    Ok((non_delete, to_delete))
}

/// `index`: the position of `v` in `l`, comparing merge-key values for
/// lists of maps.
fn index(l: &[Value], v: &Value, merge_key: &str, is_map: bool) -> R<isize> {
    let key = |item: &Value| -> Value {
        if is_map {
            item.as_object()
                .and_then(|m| m.get(merge_key))
                .cloned()
                .unwrap_or(Value::Null)
        } else {
            item.clone()
        }
    };
    let want = key(v);
    for (i, x) in l.iter().enumerate() {
        if go_eq(&want, &key(x))? {
            return Ok(i as isize);
        }
    }
    Ok(-1)
}

fn normalize_element_order(
    patch: Vec<Value>,
    server_only: Vec<Value>,
    patch_order: &[Value],
    server_order: &[Value],
    merge_key: &str,
    is_map: bool,
) -> R<Vec<Value>> {
    let patch = normalize_slice_order(patch, patch_order, merge_key, is_map)?;
    let server_only = normalize_slice_order(server_only, server_order, merge_key, is_map)?;
    merge_sorted_slice(server_only, patch, server_order, merge_key, is_map)
}

fn normalize_slice_order(
    to_sort: Vec<Value>,
    order: &[Value],
    merge_key: &str,
    is_map: bool,
) -> R<Vec<Value>> {
    let (mut to_sort, to_delete) = if is_map {
        for list in [&to_sort[..], order] {
            for item in list {
                let Value::Object(m) = item else {
                    return err("expected map[string]interface{}");
                };
                if !m.contains_key(merge_key) {
                    return err(format!(
                        "map does not contain declared merge key: {merge_key}"
                    ));
                }
            }
        }
        extract_to_delete_items(to_sort)?
    } else {
        (to_sort, Vec::new())
    };
    stable_sort(&mut to_sort, |a, b| {
        let ia = index(order, a, merge_key, is_map)?;
        if ia >= 0 {
            let ib = index(order, b, merge_key, is_map)?;
            if ib >= 0 {
                return Ok(ia < ib);
            }
        }
        Ok(true)
    })?;
    to_sort.extend(to_delete);
    Ok(to_sort)
}

fn merge_sorted_slice(
    left: Vec<Value>,
    right: Vec<Value>,
    server_order: &[Value],
    merge_key: &str,
    is_map: bool,
) -> R<Vec<Value>> {
    let size = left.len() + right.len();
    let (mut i, mut j) = (0, 0);
    let mut s = Vec::with_capacity(size);
    for _ in 0..size {
        if i >= left.len() && j < right.len() {
            s.push(right[j].clone());
            j += 1;
        } else if j >= right.len() && i < left.len() {
            s.push(left[i].clone());
            i += 1;
        } else {
            let li = index(server_order, &left[i], merge_key, is_map)?;
            let ri = index(server_order, &right[j], merge_key, is_map)?;
            if li >= 0 && ri >= 0 && li < ri {
                s.push(left[i].clone());
                i += 1;
            } else {
                s.push(right[j].clone());
                j += 1;
            }
        }
    }
    Ok(s)
}

// ---- sort.SliceStable -------------------------------------------------

/// Go's `sort.SliceStable` (insertion sort on blocks of 20, then
/// `symMerge`), step for step. `less` may fail; the first failure aborts
/// the sort, as a panic would.
fn stable_sort<T>(data: &mut [T], less: impl FnMut(&T, &T) -> R<bool>) -> R<()> {
    let mut s = Sorter {
        data,
        less,
        failed: None,
    };
    s.stable();
    match s.failed {
        Some(e) => Err(e),
        None => Ok(()),
    }
}

struct Sorter<'a, T, F> {
    data: &'a mut [T],
    less: F,
    failed: Option<SmpError>,
}

impl<T, F: FnMut(&T, &T) -> R<bool>> Sorter<'_, T, F> {
    fn less(&mut self, i: usize, j: usize) -> bool {
        if self.failed.is_some() {
            return false;
        }
        match (self.less)(&self.data[i], &self.data[j]) {
            Ok(b) => b,
            Err(e) => {
                self.failed = Some(e);
                false
            }
        }
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.data.swap(i, j);
    }

    fn stable(&mut self) {
        let n = self.data.len();
        let mut block = 20;
        let (mut a, mut b) = (0, block);
        while b <= n {
            self.insertion_sort(a, b);
            a = b;
            b += block;
        }
        self.insertion_sort(a, n);
        while block < n {
            a = 0;
            b = 2 * block;
            while b <= n {
                self.sym_merge(a, a + block, b);
                a = b;
                b += 2 * block;
            }
            let m = a + block;
            if m < n {
                self.sym_merge(a, m, n);
            }
            block *= 2;
        }
    }

    fn insertion_sort(&mut self, a: usize, b: usize) {
        for i in a + 1..b {
            let mut j = i;
            while j > a && self.less(j, j - 1) {
                self.swap(j, j - 1);
                j -= 1;
            }
        }
    }

    fn sym_merge(&mut self, a: usize, m: usize, b: usize) {
        if m - a == 1 {
            let (mut i, mut j) = (m, b);
            while i < j {
                let h = (i + j) / 2;
                if self.less(h, a) {
                    i = h + 1;
                } else {
                    j = h;
                }
            }
            for k in a..i.saturating_sub(1) {
                self.swap(k, k + 1);
            }
            return;
        }
        if b - m == 1 {
            let (mut i, mut j) = (a, m);
            while i < j {
                let h = (i + j) / 2;
                if !self.less(m, h) {
                    i = h + 1;
                } else {
                    j = h;
                }
            }
            let mut k = m;
            while k > i {
                self.swap(k, k - 1);
                k -= 1;
            }
            return;
        }
        let mid = (a + b) / 2;
        let n = mid + m;
        let (mut start, mut r) = if m > mid { (n - b, mid) } else { (a, m) };
        let p = n - 1;
        while start < r {
            let c = (start + r) / 2;
            if !self.less(p - c, c) {
                start = c + 1;
            } else {
                r = c;
            }
        }
        let end = n - start;
        if start < m && m < end {
            self.rotate(start, m, end);
        }
        if a < start && start < mid {
            self.sym_merge(a, start, mid);
        }
        if mid < end && end < b {
            self.sym_merge(mid, end, b);
        }
    }

    fn rotate(&mut self, a: usize, m: usize, b: usize) {
        let (mut i, mut j) = (m - a, b - m);
        while i != j {
            if i > j {
                self.swap_range(m - i, m, j);
                i -= j;
            } else {
                self.swap_range(m - i, m + j - i, i);
                j -= i;
            }
        }
        self.swap_range(m - i, m, i);
    }

    fn swap_range(&mut self, a: usize, b: usize, n: usize) {
        for i in 0..n {
            self.swap(a + i, b + i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kubemu_api_types::resource;
    use serde_json::json;

    fn pod() -> Schema {
        resource("", "v1", "pods").unwrap().schema()
    }

    fn smp(original: Value, patch: Value) -> R<Value> {
        strategic_merge_patch(&original, &patch, pod())
    }

    #[test]
    fn merges_lists_by_key() {
        let original = json!({"spec": {"containers": [
            {"name": "a", "image": "a:1"},
            {"name": "b", "image": "b:1"},
        ]}});
        let out = smp(
            original.clone(),
            json!({"spec": {"containers": [{"name": "b", "image": "b:2"}, {"name": "c", "image": "c:1"}]}}),
        )
        .unwrap();
        assert_eq!(
            out["spec"]["containers"],
            json!([
                {"name": "a", "image": "a:1"},
                {"name": "b", "image": "b:2"},
                {"name": "c", "image": "c:1"},
            ])
        );
        let out = smp(
            original,
            json!({"spec": {"containers": [{"$patch": "delete", "name": "a"}]}}),
        )
        .unwrap();
        assert_eq!(
            out["spec"]["containers"],
            json!([{"name": "b", "image": "b:1"}])
        );
    }

    #[test]
    fn replace_and_delete_directives() {
        let original = json!({"metadata": {"labels": {"a": "1", "b": "2"}}});
        assert_eq!(
            smp(
                original.clone(),
                json!({"metadata": {"labels": {"$patch": "replace", "c": "3"}}})
            )
            .unwrap(),
            json!({"metadata": {"labels": {"c": "3"}}})
        );
        assert_eq!(
            smp(
                original.clone(),
                json!({"metadata": {"labels": {"a": null}}})
            )
            .unwrap(),
            json!({"metadata": {"labels": {"b": "2"}}})
        );
        assert_eq!(
            smp(original, json!({"$patch": "delete"})).unwrap(),
            json!({})
        );
    }

    #[test]
    fn primitive_lists() {
        let original = json!({"metadata": {"finalizers": ["a", "b"]}});
        assert_eq!(
            smp(
                original.clone(),
                json!({"metadata": {"finalizers": ["c", "a"]}})
            )
            .unwrap(),
            json!({"metadata": {"finalizers": ["c", "a", "b"]}})
        );
        assert_eq!(
            smp(
                original,
                json!({"metadata": {"$deleteFromPrimitiveList/finalizers": ["a"]}})
            )
            .unwrap(),
            json!({"metadata": {"finalizers": ["b"]}})
        );
        // Lists without a merge strategy are replaced.
        assert_eq!(
            smp(
                json!({"spec": {"containers": [{"name": "a", "args": ["x"]}]}}),
                json!({"spec": {"containers": [{"name": "a", "args": ["y"]}]}})
            )
            .unwrap(),
            json!({"spec": {"containers": [{"name": "a", "args": ["y"]}]}})
        );
    }

    #[test]
    fn go_semantics() {
        // Unknown field holding maps on both sides: lookup fails.
        assert!(smp(json!({"bogus": {}}), json!({"bogus": {"a": 1}})).is_err());
        // Mixed element types.
        assert!(
            smp(
                json!({"metadata": {"finalizers": ["a"]}}),
                json!({"metadata": {"finalizers": [1]}})
            )
            .is_err()
        );
        // In-place deduplication with an empty patch reorders.
        assert_eq!(
            smp(
                json!({"metadata": {"finalizers": ["a", "a", "b", "c"]}}),
                json!({"metadata": {"finalizers": []}})
            )
            .unwrap(),
            json!({"metadata": {"finalizers": ["a", "c", "b"]}})
        );
    }

    #[test]
    fn stable_sort_is_stable() {
        let mut v: Vec<(u8, usize)> = (0..100).map(|i| ((i * 7 % 5) as u8, i)).collect();
        stable_sort(&mut v, |a, b| Ok(a.0 < b.0)).unwrap();
        assert!(
            v.windows(2)
                .all(|w| w[0].0 < w[1].0 || (w[0].0 == w[1].0 && w[0].1 < w[1].1))
        );
    }
}
