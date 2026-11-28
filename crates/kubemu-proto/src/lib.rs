//! Protobuf request bodies of built-in types, decoded to JSON (ADR 0014).
//!
//! Typed client-go clients (kubectl's typed commands, Helm's release
//! storage) send `application/vnd.kubernetes.protobuf` bodies and never fall
//! back to JSON. kubemu keeps JSON everywhere else (ADR 0010), so this crate
//! turns such a body into the JSON Go would produce from it: protobuf → Go
//! struct → `encoding/json`. The rest of the request pipeline then treats it
//! as any JSON body.
//!
//! The one entry point is [`decode`]. It is driven by a wire table generated
//! from the Go types (`wiretable.json`, `mise run wiretable`), which carries
//! what the `.proto` files leave out: pointers, `omitempty`/`omitzero`,
//! inlined structs and self-marshalling types (`metav1.Time`, `Quantity`,
//! `IntOrString`, `RawExtension`). Checked against Go by the `proto` fuzz op.
//!
//! What it does not do: encode protobuf (responses stay JSON), or decode
//! custom resources (clients send those as JSON; a real apiserver refuses
//! protobuf for them too). Quantities pass through as sent; Go clients send
//! them canonical, and the pipeline canonicalizes and checks them anyway.

mod table;
mod wire;

use std::collections::BTreeMap;
use std::fmt;

use serde_json::{Map, Number, Value};

use table::{Field, Ty, table};
use wire::Raw;

/// The media type of a Kubernetes protobuf body.
pub const CONTENT_TYPE: &str = "application/vnd.kubernetes.protobuf";

const MAGIC: &[u8; 4] = b"k8s\0";

/// Deeper than any built-in type nests; stops hostile input.
const MAX_DEPTH: u32 = 64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// Not a well-formed protobuf body (a 400).
    Malformed(String),
    /// The envelope names a kind with no protobuf encoding, such as a
    /// custom resource (a 415, as a real apiserver answers).
    UnknownKind { api_version: String, kind: String },
    /// Well-formed, but beyond what kubemu decodes (a 415).
    Unsupported(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Malformed(m) | Error::Unsupported(m) => f.write_str(m),
            Error::UnknownKind { api_version, kind } => write!(
                f,
                "no kind {kind:?} is registered for version {api_version:?} in scheme"
            ),
        }
    }
}

/// The Kubernetes version the wire table was generated from.
pub fn kubernetes_version() -> &'static str {
    &table().kubernetes
}

/// Decodes a protobuf body (the `k8s\0` magic and a `runtime.Unknown`
/// envelope) to JSON, with `apiVersion` and `kind` set. An envelope without
/// them takes the defaults, as the apiserver's decoder does.
pub fn decode(body: &[u8], default_api_version: &str, default_kind: &str) -> Result<Value, Error> {
    let rest = body.strip_prefix(MAGIC).ok_or_else(|| {
        Error::Malformed(
            "provided data does not appear to be a protobuf message, expected prefix [107 56 115 0]"
                .into(),
        )
    })?;
    let (mut api_version, mut kind, mut raw, mut encoding) =
        (String::new(), String::new(), &[][..], String::new());
    for (num, r) in wire::fields(rest)? {
        match (num, r) {
            (1, Raw::Bytes(tm)) => {
                for (n, r) in wire::fields(tm)? {
                    match (n, r) {
                        (1, Raw::Bytes(s)) => api_version = utf8(s),
                        (2, Raw::Bytes(s)) => kind = utf8(s),
                        _ => {}
                    }
                }
            }
            (2, Raw::Bytes(b)) => raw = b,
            (3, Raw::Bytes(s)) => encoding = utf8(s),
            _ => {}
        }
    }
    if !encoding.is_empty() {
        return Err(Error::Unsupported(format!(
            "protobuf content encoding {encoding:?} is not supported"
        )));
    }
    if api_version.is_empty() {
        api_version = default_api_version.to_owned();
    }
    if kind.is_empty() {
        kind = default_kind.to_owned();
    }
    let t = table();
    let Some(&msg) = t.kinds.get(&format!("{api_version}/{kind}")) else {
        return Err(Error::UnknownKind { api_version, kind });
    };
    let mut obj = Decoder.message(msg, raw, 0)?;
    obj.insert("apiVersion".into(), Value::String(api_version));
    obj.insert("kind".into(), Value::String(kind));
    Ok(Value::Object(obj))
}

fn utf8(b: &[u8]) -> String {
    String::from_utf8_lossy(b).into_owned()
}

struct Decoder;

impl Decoder {
    fn message(&self, idx: usize, buf: &[u8], depth: u32) -> Result<Map<String, Value>, Error> {
        if depth > MAX_DEPTH {
            return Err(Error::Malformed(
                "protobuf message nested too deeply".into(),
            ));
        }
        let msg = &table().messages[idx];
        let mut by_num: BTreeMap<u32, Vec<Raw<'_>>> = BTreeMap::new();
        for (n, r) in wire::fields(buf)? {
            by_num.entry(n).or_default().push(r);
        }
        let mut out = Map::new();
        let mut inlined = Vec::new();
        for f in &msg.fields {
            let raws = by_num.get(&f.num).map(Vec::as_slice).unwrap_or(&[]);
            let raws = if f.num == 0 { &[][..] } else { raws };
            let Some(v) = self.field(f, raws, depth, &msg.name)? else {
                continue;
            };
            if f.json.is_empty() {
                inlined.push(v);
            } else {
                out.insert(f.json.clone(), v);
            }
        }
        // Go's JSON promotes an embedded struct's fields; the outer
        // struct's own fields win a clash.
        for v in inlined {
            if let Value::Object(m) = v {
                for (k, x) in m {
                    out.entry(k).or_insert(x);
                }
            }
        }
        Ok(out)
    }

    /// A struct field, or `None` where Go's JSON leaves it out.
    fn field(
        &self,
        f: &Field,
        raws: &[Raw<'_>],
        depth: u32,
        msg: &str,
    ) -> Result<Option<Value>, Error> {
        // A nil pointer, slice or map: omitted, or null.
        let nil = || (!(f.omitempty || f.omitzero)).then_some(Value::Null);
        let ctx = || format!("{msg}.{}", f.json);
        match &f.ty {
            Ty::List(elem) => {
                if raws.is_empty() {
                    return Ok(nil());
                }
                let items = self.list(elem, raws, depth, &ctx())?;
                // Only a packed field of length 0 gets here empty: a
                // non-nil empty slice.
                Ok((!(items.is_empty() && f.omitempty)).then_some(Value::Array(items)))
            }
            Ty::Map(v) => {
                if raws.is_empty() {
                    return Ok(nil());
                }
                Ok(Some(self.map(v, raws, depth, &ctx())?))
            }
            _ if raws.is_empty() && f.pointer => Ok(nil()),
            ty => {
                let v = self.single(ty, raws, depth, &ctx())?;
                if f.pointer {
                    return Ok(Some(v));
                }
                let omit = match ty {
                    Ty::Bytes | Ty::Wrap(_) if v.is_null() => return Ok(nil()),
                    Ty::Bytes => f.omitempty && v == "",
                    Ty::Wrap(_) => f.omitempty && v.as_array().is_some_and(Vec::is_empty),
                    Ty::String
                    | Ty::Bool
                    | Ty::Int32
                    | Ty::Int64
                    | Ty::Uint32
                    | Ty::Uint64
                    | Ty::Double => (f.omitempty || f.omitzero) && scalar_is_zero(&v),
                    // Structs: omitempty never drops them; omitzero drops
                    // the zero value (by IsZero where the type has one).
                    Ty::Time | Ty::MicroTime | Ty::RawExtension | Ty::FieldsV1 => {
                        f.omitzero && v.is_null()
                    }
                    Ty::Quantity => f.omitzero && v == "0",
                    Ty::IntOrString => f.omitzero && v == 0,
                    Ty::Msg(idx) => {
                        f.omitzero && Value::Object(self.message(*idx, &[], depth + 1)?) == v
                    }
                    Ty::List(_) | Ty::Map(_) => unreachable!(),
                };
                Ok((!omit).then_some(v))
            }
        }
    }

    /// A non-repeated value from its occurrences: the last one wins, and
    /// messages merge (decoded from the concatenation, as proto merges).
    /// No occurrence: the Go zero value.
    fn single(&self, ty: &Ty, raws: &[Raw<'_>], depth: u32, ctx: &str) -> Result<Value, Error> {
        let last = raws.last().copied();
        let bytes = |r: Option<Raw<'_>>| -> Result<Option<Vec<u8>>, Error> {
            match r {
                None => Ok(None),
                Some(Raw::Bytes(b)) => Ok(Some(b.to_vec())),
                Some(r) => Err(wrong_wire(r, ctx)),
            }
        };
        let merged = || -> Result<Vec<u8>, Error> {
            let mut buf = Vec::new();
            for r in raws {
                match r {
                    Raw::Bytes(b) => buf.extend_from_slice(b),
                    r => return Err(wrong_wire(*r, ctx)),
                }
            }
            Ok(buf)
        };
        Ok(match ty {
            Ty::String
            | Ty::Bool
            | Ty::Int32
            | Ty::Int64
            | Ty::Uint32
            | Ty::Uint64
            | Ty::Double => match last {
                Some(r) => scalar(ty, r, ctx)?,
                None => zero_scalar(ty),
            },
            Ty::Bytes => match bytes(last)? {
                Some(b) => Value::String(base64(&b)),
                None => Value::Null,
            },
            Ty::Msg(idx) => Value::Object(self.message(*idx, &merged()?, depth + 1)?),
            Ty::Time | Ty::MicroTime => {
                // Time.Unmarshal resets: the last occurrence wins whole.
                match bytes(last)? {
                    None => Value::Null,
                    Some(b) if b.is_empty() => Value::Null,
                    Some(b) => {
                        let (mut secs, mut nanos) = (0i64, 0i64);
                        for (n, r) in wire::fields(&b)? {
                            match (n, r) {
                                (1, Raw::Varint(v)) => secs = v as i64,
                                (2, Raw::Varint(v)) => nanos = i64::from(v as i32),
                                (1 | 2, r) => return Err(wrong_wire(r, ctx)),
                                _ => {}
                            }
                        }
                        Value::String(if matches!(ty, Ty::Time) {
                            // Time keeps whole seconds only.
                            format_time(secs, 0, false)
                        } else {
                            // MicroTime truncates (toward zero) to µs.
                            format_time(secs, nanos - nanos % 1000, true)
                        })
                    }
                }
            }
            Ty::Quantity => {
                let mut s = None;
                for (n, r) in wire::fields(&merged()?)? {
                    match (n, r) {
                        (1, Raw::Bytes(b)) => s = Some(utf8(b)),
                        (1, r) => return Err(wrong_wire(r, ctx)),
                        _ => {}
                    }
                }
                Value::String(s.unwrap_or_else(|| "0".into()))
            }
            Ty::IntOrString => {
                let (mut kind, mut int, mut s) = (0i64, 0i64, String::new());
                for (n, r) in wire::fields(&merged()?)? {
                    match (n, r) {
                        (1, Raw::Varint(v)) => kind = v as i64,
                        (2, Raw::Varint(v)) => int = i64::from(v as i32),
                        (3, Raw::Bytes(b)) => s = utf8(b),
                        (1..=3, r) => return Err(wrong_wire(r, ctx)),
                        _ => {}
                    }
                }
                match kind {
                    0 => Value::from(int),
                    1 => Value::String(s),
                    _ => {
                        return Err(Error::Malformed(format!(
                            "json: error calling MarshalJSON for type *intstr.IntOrString: impossible IntOrString.Type ({ctx})"
                        )));
                    }
                }
            }
            Ty::RawExtension | Ty::FieldsV1 => {
                let mut raw = None;
                for (n, r) in wire::fields(&merged()?)? {
                    match (n, r) {
                        (1, Raw::Bytes(b)) => raw = Some(b.to_vec()),
                        (1, r) => return Err(wrong_wire(r, ctx)),
                        _ => {}
                    }
                }
                match raw {
                    None => Value::Null,
                    // FieldsV1 encodes no bytes as null; RawExtension's
                    // MarshalJSON returns them as is, which Go rejects.
                    Some(b) if b.is_empty() && matches!(ty, Ty::FieldsV1) => Value::Null,
                    Some(b) => serde_json::from_slice(&b).map_err(|_| {
                        Error::Unsupported(format!(
                            "{ctx}: embedded data is not JSON (protobuf-encoded embedded objects are not supported)"
                        ))
                    })?,
                }
            }
            Ty::Wrap(elem) => {
                if raws.is_empty() {
                    return Ok(Value::Null);
                }
                let buf = merged()?;
                let items: Vec<Raw<'_>> = wire::fields(&buf)?
                    .into_iter()
                    .filter(|(n, _)| *n == 1)
                    .map(|(_, r)| r)
                    .collect();
                if items.is_empty() {
                    return Ok(Value::Null);
                }
                Value::Array(self.list(elem, &items, depth, ctx)?)
            }
            Ty::List(_) | Ty::Map(_) => {
                return Err(Error::Unsupported(format!("{ctx}: nested repeated field")));
            }
        })
    }

    fn list(
        &self,
        elem: &Ty,
        raws: &[Raw<'_>],
        depth: u32,
        ctx: &str,
    ) -> Result<Vec<Value>, Error> {
        let mut out = Vec::with_capacity(raws.len());
        for &r in raws {
            match (elem, r) {
                (Ty::Bool | Ty::Int32 | Ty::Int64 | Ty::Uint32 | Ty::Uint64, Raw::Bytes(b)) => {
                    for v in wire::packed(b)? {
                        out.push(scalar(elem, Raw::Varint(v), ctx)?);
                    }
                }
                (Ty::Double, Raw::Bytes(b)) => {
                    let (chunks, rest) = b.as_chunks::<8>();
                    if !rest.is_empty() {
                        return Err(Error::Malformed("unexpected EOF".into()));
                    }
                    for c in chunks {
                        out.push(scalar(elem, Raw::Fixed64(u64::from_le_bytes(*c)), ctx)?);
                    }
                }
                _ => out.push(self.single(elem, &[r], depth, ctx)?),
            }
        }
        Ok(out)
    }

    fn map(&self, value: &Ty, raws: &[Raw<'_>], depth: u32, ctx: &str) -> Result<Value, Error> {
        let mut out = Map::new();
        for &r in raws {
            let Raw::Bytes(entry) = r else {
                return Err(wrong_wire(r, ctx));
            };
            let mut key = String::new();
            let mut vals = Vec::new();
            for (n, r) in wire::fields(entry)? {
                match (n, r) {
                    (1, Raw::Bytes(b)) => key = utf8(b),
                    (1, r) => return Err(wrong_wire(r, ctx)),
                    (2, r) => vals.push(r),
                    _ => {}
                }
            }
            // A missing value is the Go zero value (a nil []byte: null).
            let v = self.single(value, &vals, depth, ctx)?;
            out.insert(key, v);
        }
        Ok(Value::Object(out))
    }
}

fn wrong_wire(r: Raw<'_>, ctx: &str) -> Error {
    Error::Malformed(format!(
        "proto: wrong wireType = {} for field {ctx}",
        r.wire_type()
    ))
}

fn scalar(ty: &Ty, r: Raw<'_>, ctx: &str) -> Result<Value, Error> {
    Ok(match (ty, r) {
        (Ty::String, Raw::Bytes(b)) => Value::String(utf8(b)),
        (Ty::Bool, Raw::Varint(v)) => Value::Bool(v != 0),
        (Ty::Int32, Raw::Varint(v)) => Value::from(i64::from(v as i32)),
        (Ty::Int64, Raw::Varint(v)) => Value::from(v as i64),
        (Ty::Uint32, Raw::Varint(v)) => Value::from(u64::from(v as u32)),
        (Ty::Uint64, Raw::Varint(v)) => Value::from(v),
        (Ty::Double, Raw::Fixed64(v)) => float(f64::from_bits(v), ctx)?,
        (_, r) => return Err(wrong_wire(r, ctx)),
    })
}

/// Go encodes whole floats without a fraction (`1`, not `1.0`).
fn float(f: f64, ctx: &str) -> Result<Value, Error> {
    if f.fract() == 0.0 && f.abs() < 9.007_199_254_740_992e15 {
        return Ok(Value::from(f as i64));
    }
    Number::from_f64(f)
        .map(Value::Number)
        .ok_or_else(|| Error::Malformed(format!("json: unsupported value: {f} ({ctx})")))
}

fn zero_scalar(ty: &Ty) -> Value {
    match ty {
        Ty::String => Value::String(String::new()),
        Ty::Bool => Value::Bool(false),
        _ => Value::from(0),
    }
}

fn scalar_is_zero(v: &Value) -> bool {
    match v {
        Value::String(s) => s.is_empty(),
        Value::Bool(b) => !b,
        Value::Number(n) => n.as_f64() == Some(0.0),
        _ => false,
    }
}

/// RFC 3339 in UTC, as `metav1.Time` (seconds) and `metav1.MicroTime`
/// (six fractional digits) encode. `nanos` may be out of range; it is
/// normalized as `time.Unix` does.
fn format_time(secs: i64, nanos: i64, micro: bool) -> String {
    let total = i128::from(secs) * 1_000_000_000 + i128::from(nanos);
    let secs = total.div_euclid(1_000_000_000) as i64;
    let nanos = total.rem_euclid(1_000_000_000) as i64;
    let days = secs.div_euclid(86_400);
    let rem = secs.rem_euclid(86_400);
    let (y, m, d) = civil_from_days(days);
    let mut s = format!(
        "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}",
        rem / 3600,
        rem % 3600 / 60,
        rem % 60
    );
    if micro {
        s.push_str(&format!(".{:06}", nanos / 1000));
    }
    s.push('Z');
    s
}

/// Days since 1970-01-01 to (year, month, day), proleptic Gregorian
/// (Howard Hinnant's algorithm).
fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    (y, m, d)
}

/// Standard base64 with padding, as Go's JSON encodes `[]byte`.
fn base64(b: &[u8]) -> String {
    const A: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut s = String::with_capacity(b.len().div_ceil(3) * 4);
    for c in b.chunks(3) {
        let n = (u32::from(c[0]) << 16)
            | (u32::from(*c.get(1).unwrap_or(&0)) << 8)
            | u32::from(*c.get(2).unwrap_or(&0));
        for i in 0..4 {
            if i <= c.len() {
                s.push(A[(n >> (18 - 6 * i) & 63) as usize] as char);
            } else {
                s.push('=');
            }
        }
    }
    s
}

#[cfg(test)]
mod tests;
