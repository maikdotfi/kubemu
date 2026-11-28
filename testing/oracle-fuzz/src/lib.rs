//! Differential fuzzing against the Go oracle (README §10).
//!
//! Inputs come from a seeded [`Xoshiro256StarStar`], one seed per case, so
//! any mismatch replays with `--seed <case> --cases 1`. Each case goes to
//! both kubemu and the oracle process; results (or error texts) must agree.

use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Child, ChildStdin, ChildStdout};

use kubemu_api_types::{RESOURCES, Schema, Ty};
use kubemu_env::Xoshiro256StarStar;
use serde_json::{Map, Value, json};

/// A running `oracle-go` process speaking line-delimited JSON.
#[derive(Debug)]
pub struct Oracle {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl Oracle {
    pub fn spawn(binary: &Path) -> std::io::Result<Self> {
        let mut child = kubemu_env_real::spawn_piped(binary, &[])?;
        let stdin = child.stdin.take().expect("piped stdin");
        let stdout = BufReader::new(child.stdout.take().expect("piped stdout"));
        let mut o = Self {
            child,
            stdin,
            stdout,
        };
        match o.call("ping", Value::Null)? {
            Ok(v) if v == json!("pong") => Ok(o),
            other => Err(std::io::Error::other(format!(
                "oracle ping failed: {other:?}"
            ))),
        }
    }

    /// Sends one request. The outer error is transport; the inner one is
    /// the oracle's own error text.
    pub fn call(&mut self, op: &str, payload: Value) -> std::io::Result<Result<Value, String>> {
        let req = json!({"op": op, "payload": payload});
        writeln!(self.stdin, "{req}")?;
        self.stdin.flush()?;
        let mut line = String::new();
        if self.stdout.read_line(&mut line)? == 0 {
            return Err(std::io::Error::other("oracle closed its stdout"));
        }
        let resp: Value = serde_json::from_str(&line).map_err(std::io::Error::other)?;
        Ok(match resp.get("error").and_then(Value::as_str) {
            Some(e) => Err(e.to_owned()),
            None => Ok(resp.get("result").cloned().unwrap_or(Value::Null)),
        })
    }
}

impl Drop for Oracle {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// Random JSON shaped to make merges interesting: few distinct keys so
/// document and patch collide, plenty of nulls, nested objects in arrays.
#[derive(Debug)]
pub struct Gen(Xoshiro256StarStar);

const KEYS: [&str; 5] = ["a", "b", "c", "d", "e"];
const STRINGS: [&str; 5] = ["", "x", "<&>", "é\"\\", "null"];

impl Gen {
    pub fn new(seed: u64) -> Self {
        Self(Xoshiro256StarStar::seed_from_u64(seed))
    }

    fn below(&mut self, n: u64) -> u64 {
        if n == 0 {
            return 0;
        }
        // Modulo bias is irrelevant for input generation.
        self.0.next_u64() % n
    }

    pub fn value(&mut self, depth: u32) -> Value {
        let kinds = if depth == 0 { 5 } else { 8 };
        match self.below(kinds) {
            0 | 1 => Value::Null,
            2 => Value::Bool(self.below(2) == 1),
            3 => match self.below(3) {
                0 => json!(self.below(100)),
                1 => json!(-(self.below(100) as i64)),
                _ => json!(self.below(100) as f64 + 0.5),
            },
            4 => json!(STRINGS[self.below(STRINGS.len() as u64) as usize]),
            5 => Value::Array((0..self.below(4)).map(|_| self.value(depth - 1)).collect()),
            _ => Value::Object(self.object(depth - 1)),
        }
    }

    pub fn object(&mut self, depth: u32) -> Map<String, Value> {
        (0..self.below(5))
            .map(|_| {
                let k = KEYS[self.below(KEYS.len() as u64) as usize].to_owned();
                (k, self.value(depth))
            })
            .collect()
    }

    /// Mostly objects at the top, like real documents and patches.
    pub fn top(&mut self, depth: u32) -> Value {
        if self.below(10) < 8 {
            Value::Object(self.object(depth))
        } else {
            self.value(depth)
        }
    }
}

/// What one case found: `Ok(true)` if both sides applied the patch,
/// `Ok(false)` if both rejected it, `Err` with a description on mismatch.
pub type CaseResult = std::io::Result<Result<bool, String>>;

/// One merge-patch case.
pub fn merge_case(oracle: &mut Oracle, seed: u64) -> CaseResult {
    let mut g = Gen::new(seed);
    let doc = g.top(3);
    let patch = g.top(3);
    let ours = kubemu_patch::merge_patch(&doc, &patch).map_err(|e| e.to_string());
    let theirs = oracle.call("merge", json!({"original": doc, "patch": patch}))?;
    // Merge patch error texts are part of the contract: compare them.
    Ok(if ours == theirs {
        Ok(ours.is_ok())
    } else {
        Err(format!(
            "merge mismatch\n  doc:    {doc}\n  patch:  {patch}\n  kubemu: {ours:?}\n  oracle: {theirs:?}"
        ))
    })
}

/// Results agree when both succeed with equal values (numbers compared by
/// value: Go re-encodes `2.0` as `2`), or both fail. Error texts are not
/// compared for JSON patch and SMP: Go formats values into them with `%v`.
fn agree(ours: &Result<Value, String>, theirs: &Result<Value, String>) -> bool {
    match (ours, theirs) {
        (Ok(a), Ok(b)) => same_json(a, b),
        (Err(_), Err(_)) => true,
        _ => false,
    }
}

fn same_json(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => x == y || x.as_f64() == y.as_f64(),
        (Value::Array(x), Value::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(a, b)| same_json(a, b))
        }
        (Value::Object(x), Value::Object(y)) => {
            x.len() == y.len()
                && x.iter()
                    .all(|(k, v)| y.get(k).is_some_and(|w| same_json(v, w)))
        }
        _ => a == b,
    }
}

fn mismatch(
    op: &str,
    doc: &Value,
    patch: &Value,
    ours: &Result<Value, String>,
    theirs: &Result<Value, String>,
) -> Result<bool, String> {
    if agree(ours, theirs) {
        Ok(ours.is_ok())
    } else {
        Err(format!(
            "{op} mismatch\n  doc:    {doc}\n  patch:  {patch}\n  kubemu: {ours:?}\n  oracle: {theirs:?}"
        ))
    }
}

/// One JSON patch case.
pub fn jsonpatch_case(oracle: &mut Oracle, seed: u64) -> CaseResult {
    let mut g = Gen::new(seed);
    let doc = g.jp_doc();
    let patch = Value::Array((0..1 + g.below(3)).map(|_| g.jp_op(&doc)).collect());
    let ours = kubemu_patch::json_patch(&doc, &patch).map_err(|e| e.to_string());
    let theirs = oracle.call("jsonpatch", json!({"original": doc, "patch": patch}))?;
    Ok(mismatch("jsonpatch", &doc, &patch, &ours, &theirs))
}

/// One strategic merge patch case, on a served kind.
pub fn smp_case(oracle: &mut Oracle, seed: u64) -> CaseResult {
    let mut g = Gen::new(seed);
    // Pods have most of the interesting patch metadata; weight them.
    let r = if g.below(2) == 0 {
        kubemu_api_types::resource("", "v1", "pods").expect("pods are served")
    } else {
        &RESOURCES[g.below(RESOURCES.len() as u64) as usize]
    };
    let schema = r.schema();
    let doc = Value::Object(g.object_of(schema, 4, false));
    let patch = Value::Object(g.patch_object(schema, doc.as_object(), 4));
    let ours = kubemu_patch::strategic_merge_patch(&doc, &patch, schema).map_err(|e| e.to_string());
    let theirs = oracle.call(
        "smp",
        json!({"apiVersion": r.api_version(), "kind": r.kind, "original": doc, "patch": patch}),
    )?;
    Ok(mismatch(
        &format!("smp {}", r.kind),
        &doc,
        &patch,
        &ours,
        &theirs,
    ))
}

/// JSON patch inputs.
impl Gen {
    fn chance(&mut self, num: u64, den: u64) -> bool {
        self.below(den) < num
    }

    fn pick<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        &items[self.below(items.len() as u64) as usize]
    }

    /// Documents for JSON patch: like [`top`](Self::top), sometimes with a
    /// key that needs `~` escaping.
    fn jp_doc(&mut self) -> Value {
        let mut doc = self.top(3);
        if let Value::Object(m) = &mut doc
            && self.chance(1, 4)
        {
            let v = self.value(2);
            m.insert("x/y~z".into(), v);
        }
        doc
    }

    /// A JSON pointer: usually to something in `doc` (or just past it), so
    /// operations mostly apply.
    fn jp_path(&mut self, doc: &Value) -> String {
        let mut path = String::new();
        let mut cur = Some(doc);
        loop {
            let stop = self.chance(1, 3);
            let seg = match cur {
                Some(Value::Object(m)) if !m.is_empty() && !self.chance(1, 8) => {
                    let k = m
                        .keys()
                        .nth(self.below(m.len() as u64) as usize)
                        .expect("in range")
                        .clone();
                    cur = m.get(&k);
                    k
                }
                Some(Value::Array(a)) if !self.chance(1, 8) => {
                    let n = a.len() as i64;
                    let i = *self.pick(&[0, 0, 1, n - 1, n - 1, n, n + 1, -1, -n, -n - 1]);
                    cur = usize::try_from(i).ok().and_then(|i| a.get(i));
                    if self.chance(1, 8) {
                        "-".into()
                    } else {
                        i.to_string()
                    }
                }
                _ => {
                    cur = None;
                    (*self.pick(&["a", "b", "z", "0", "-", "x/y~z", ""])).to_owned()
                }
            };
            path.push('/');
            path.push_str(&seg.replace('~', "~0").replace('/', "~1"));
            if stop || cur.is_none_or(|v| !(v.is_object() || v.is_array())) {
                break;
            }
        }
        match self.below(30) {
            0 => String::new(),
            1 => path.trim_start_matches('/').to_owned(),
            _ => path,
        }
    }

    fn jp_op(&mut self, doc: &Value) -> Value {
        let op = *self.pick(&[
            "add", "remove", "replace", "move", "copy", "test", "test", "bogus",
        ]);
        let mut m = Map::new();
        if !self.chance(1, 60) {
            m.insert("op".into(), json!(op));
        }
        if !self.chance(1, 60) {
            let p = self.jp_path(doc);
            m.insert("path".into(), json!(p));
        }
        if matches!(op, "move" | "copy") || self.chance(1, 20) {
            let p = self.jp_path(doc);
            m.insert("from".into(), json!(p));
        }
        if matches!(op, "add" | "replace" | "test") && !self.chance(1, 10) {
            // For test, often the value already there.
            let v = match (op, m.get("path").and_then(Value::as_str)) {
                ("test", Some(p)) if self.chance(1, 2) => {
                    doc.pointer(p).cloned().unwrap_or(Value::Null)
                }
                _ => self.value(2),
            };
            m.insert("value".into(), v);
        }
        if self.chance(1, 80) {
            return self.value(1);
        }
        Value::Object(m)
    }
}

/// Strategic merge patch inputs, shaped by the kind's schema.
impl Gen {
    /// Fields to visit, weighted towards those carrying patch metadata
    /// or structure.
    fn some_fields(&mut self, schema: Schema, n: u64) -> Vec<(&'static str, Schema)> {
        let fields: Vec<(&'static str, Schema)> = schema.fields().collect();
        if fields.is_empty() {
            return Vec::new();
        }
        let weight = |s: Schema| match s.ty() {
            _ if s.patch_strategy().is_some() => 12,
            Ty::Object { .. } | Ty::Array(_) | Ty::Map(_) => 3,
            _ => 1,
        };
        let total: u64 = fields.iter().map(|(_, s)| weight(*s)).sum();
        let mut out: Vec<(&'static str, Schema)> = Vec::new();
        for _ in 0..n {
            let mut r = self.below(total);
            for (name, s) in &fields {
                let w = weight(*s);
                if r < w {
                    if !out.iter().any(|(n, _)| n == name) {
                        out.push((name, *s));
                    }
                    break;
                }
                r -= w;
            }
        }
        out
    }

    fn scalar_of(&mut self, schema: Schema, junk: bool) -> Value {
        if junk && self.chance(1, 25) {
            return self.value(1);
        }
        match schema.ty() {
            Ty::String => json!(*self.pick(&["x", "y", "z"])),
            Ty::Integer => json!(self.below(3)),
            Ty::Number => json!(self.below(3) as f64 + 0.5),
            Ty::Boolean => json!(self.chance(1, 2)),
            Ty::IntOrString | Ty::Quantity => {
                if self.chance(1, 2) {
                    json!(self.below(3))
                } else {
                    json!(*self.pick(&["1", "x"]))
                }
            }
            _ => self.value(1),
        }
    }

    /// A merge-key value of the key field's type, from a small set so
    /// original and patch collide. With `junk`, sometimes of another type.
    fn key_value(&mut self, field: Option<Schema>, junk: bool) -> Value {
        let int = field.is_some_and(|f| matches!(f.ty(), Ty::Integer));
        match (int, junk && self.chance(1, 8)) {
            (true, false) | (false, true) => json!(1 + self.below(4)),
            (false, false) | (true, true) => json!(*self.pick(&["k1", "k2", "k3", "k4"])),
        }
    }

    /// A value of `schema`. Documents are well-typed (`junk == false`):
    /// the apiserver's original is always a typed object, so lookups on
    /// mistyped originals are unreachable there. Patches are not.
    fn value_of(&mut self, schema: Schema, depth: u32, junk: bool) -> Value {
        if junk && self.chance(1, 40) {
            return self.value(1);
        }
        match schema.ty() {
            Ty::Object { .. } if depth > 0 => {
                Value::Object(self.object_of(schema, depth - 1, junk))
            }
            Ty::Map(_) if depth > 0 => {
                let values = schema.values().expect("maps have values");
                Value::Object(
                    (0..self.below(3))
                        .map(|_| {
                            (
                                (*self.pick(&["a", "b", "c"])).to_owned(),
                                self.value_of(values, depth - 1, junk),
                            )
                        })
                        .collect(),
                )
            }
            Ty::Array(_) if depth > 0 => {
                let items = schema.items().expect("arrays have items");
                let key = schema.merge_key();
                Value::Array(
                    (0..self.below(4))
                        .map(|_| {
                            let mut v = self.value_of(items, depth - 1, junk);
                            if let (Some(k), Value::Object(m)) = (key, &mut v)
                                && !self.chance(1, 20)
                            {
                                m.insert(k.to_owned(), self.key_value(items.field(k), junk));
                            }
                            v
                        })
                        .collect(),
                )
            }
            Ty::Object { .. } | Ty::Map(_) => json!({}),
            Ty::Array(_) => json!([]),
            Ty::Any => self.value(1),
            _ => self.scalar_of(schema, junk),
        }
    }

    fn object_of(&mut self, schema: Schema, depth: u32, junk: bool) -> Map<String, Value> {
        let mut m = Map::new();
        let n = 1 + self.below(4);
        for (name, s) in self.some_fields(schema, n) {
            m.insert(name.to_owned(), self.value_of(s, depth, junk));
        }
        if junk && self.chance(1, 15) {
            m.insert((*self.pick(&KEYS)).to_owned(), self.value(1));
        }
        m
    }

    /// A patch for an object of `schema`, often touching what `original`
    /// has. Two shapes are never generated, because Go's own answer
    /// depends on things outside the inputs:
    /// - `$deleteFromPrimitiveList/k` with `k` but no `$setElementOrder/k`
    ///   (Go's random map order decides which applies first);
    /// - one merge-key value on two non-directive elements of a patch list
    ///   (Go aliases the first into the result and edits it in place,
    ///   through slices whose capacity is a decoder detail).
    fn patch_object(
        &mut self,
        schema: Schema,
        original: Option<&Map<String, Value>>,
        depth: u32,
    ) -> Map<String, Value> {
        let mut p = Map::new();
        match self.below(40) {
            0 => {
                p.insert(
                    "$patch".into(),
                    json!(*self.pick(&["replace", "delete", "merge", "bogus"])),
                );
            }
            1 => {
                let keys: Vec<Value> = self
                    .some_fields(schema, 3)
                    .iter()
                    .map(|(n, _)| json!(n))
                    .collect();
                p.insert("$retainKeys".into(), Value::Array(keys));
            }
            _ => {}
        }
        let n = self.below(4);
        let mut fields = self.some_fields(schema, n);
        if let Some(o) = original {
            for k in o.keys() {
                if self.chance(1, 2)
                    && let Some(s) = schema.field(k)
                    && !fields.iter().any(|(n, _)| n == k)
                {
                    fields.push((schema.fields().find(|(n, _)| n == k).expect("field").0, s));
                }
            }
        }
        for (name, s) in fields {
            let orig = original.and_then(|o| o.get(name));
            if self.chance(1, 12) {
                p.insert(name.to_owned(), Value::Null);
                continue;
            }
            match s.ty() {
                Ty::Array(_) if depth > 0 => self.patch_list(&mut p, name, s, orig, depth - 1),
                Ty::Object { .. } if depth > 0 => {
                    let v = self.patch_object(s, orig.and_then(Value::as_object), depth - 1);
                    p.insert(name.to_owned(), Value::Object(v));
                }
                Ty::Map(_) if depth > 0 => {
                    let mut m = Map::new();
                    for _ in 0..self.below(3) {
                        let k = (*self.pick(&["a", "b", "c"])).to_owned();
                        let v = if self.chance(1, 4) {
                            Value::Null
                        } else {
                            self.value_of(s.values().expect("map"), 1, true)
                        };
                        m.insert(k, v);
                    }
                    if self.chance(1, 15) {
                        m.insert("$patch".into(), json!("replace"));
                    }
                    p.insert(name.to_owned(), Value::Object(m));
                }
                _ => {
                    let v = self.value_of(s, depth.min(1), true);
                    p.insert(name.to_owned(), v);
                }
            }
        }
        if self.chance(1, 20) {
            p.insert((*self.pick(&KEYS)).to_owned(), self.value(1));
        }
        p
    }

    fn patch_list(
        &mut self,
        p: &mut Map<String, Value>,
        name: &str,
        schema: Schema,
        orig: Option<&Value>,
        depth: u32,
    ) {
        let items = schema.items().expect("arrays have items");
        let orig_items: &[Value] = orig.and_then(Value::as_array).map_or(&[], Vec::as_slice);
        let key = schema.merge_key();
        let key_field = key.and_then(|k| items.field(k));
        let mut list = Vec::new();
        let mut used = Vec::new();
        for _ in 0..self.below(4) {
            match (key, self.below(10)) {
                (Some(k), 0) => {
                    list.push(json!({"$patch": "delete", k: self.key_value(key_field, true)}))
                }
                (_, 1) if self.chance(1, 3) => {
                    list.push(json!({"$patch": *self.pick(&["replace", "merge", "bogus"])}))
                }
                (Some(k), _) => {
                    // Often an element the original has, patched.
                    let kv = match orig_items.get(self.below(orig_items.len() as u64 + 1) as usize)
                    {
                        Some(Value::Object(o)) if o.contains_key(k) && self.chance(2, 3) => {
                            o[k].clone()
                        }
                        _ => self.key_value(key_field, true),
                    };
                    if used.contains(&kv) {
                        continue;
                    }
                    used.push(kv.clone());
                    let orig_el = orig_items
                        .iter()
                        .find(|e| e.get(k) == Some(&kv))
                        .and_then(Value::as_object);
                    let mut el = self.patch_object(items, orig_el, depth);
                    el.remove("$patch");
                    el.insert(k.to_owned(), kv);
                    list.push(Value::Object(el));
                }
                (None, _) => list.push(match items.ty() {
                    Ty::Object { .. } => Value::Object(self.patch_object(items, None, depth)),
                    _ => self.value_of(items, depth, true),
                }),
            }
        }

        let primitive = !matches!(items.ty(), Ty::Object { .. });
        let mut set_order = false;
        if self.chance(1, 4) {
            // A permutation of some elements of the result, as kubectl sends.
            let mut order: Vec<Value> = list
                .iter()
                .chain(orig_items)
                .filter(|v| !v.as_object().is_some_and(|m| m.contains_key("$patch")))
                .map(|v| match key {
                    Some(k) => v.get(k).map_or(json!({}), |kv| json!({k: kv})),
                    None => v.clone(),
                })
                .collect();
            order.dedup();
            for i in (1..order.len()).rev() {
                let j = self.below(i as u64 + 1) as usize;
                order.swap(i, j);
            }
            order.truncate(order.len().saturating_sub(self.below(2) as usize));
            p.insert(format!("$setElementOrder/{name}"), Value::Array(order));
            set_order = true;
        }
        let with_list = !list.is_empty() || self.chance(1, 2);
        if primitive && self.chance(1, 4) && (set_order || !with_list) {
            let del: Vec<Value> = orig_items
                .iter()
                .filter(|_| self.chance(1, 2))
                .cloned()
                .collect();
            p.insert(
                format!("$deleteFromPrimitiveList/{name}"),
                Value::Array(del),
            );
        }
        if with_list {
            p.insert(name.to_owned(), Value::Array(list));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generator_is_seeded() {
        let a: Vec<Value> = (0..50).map(|s| Gen::new(s).top(3)).collect();
        let b: Vec<Value> = (0..50).map(|s| Gen::new(s).top(3)).collect();
        assert_eq!(a, b);
        assert!(a.iter().any(|v| v.to_string().contains("null")));
    }
}
