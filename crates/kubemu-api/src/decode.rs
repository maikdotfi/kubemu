//! Decoding a request body against its schema (ADR 0010): what the
//! apiserver's typed decode does to JSON.
//! - Values of the wrong JSON type are rejected (400), as Go's decoder
//!   rejects them.
//! - Unknown fields are dropped and reported; `fieldValidation` decides
//!   whether that is silent, a warning, or an error.
//! - `null` fields are dropped (they decode to Go zero values, which
//!   encode as absent).
//! - Strings with a format the Go type parses (`[]byte` as base64,
//!   `resource.Quantity`, `metav1.Time`) must parse. Quantities come back
//!   in the canonical form Go re-encodes them in.

use kubemu_api_types::{Schema, Ty};
use kubemu_validation::quantity;
use serde_json::{Map, Value};

use crate::status::{ApiError, ApiResult};

/// `fieldValidation`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FieldValidation {
    Ignore,
    /// The server default.
    #[default]
    Warn,
    Strict,
}

impl FieldValidation {
    pub fn parse(s: Option<&str>) -> ApiResult<Self> {
        match s {
            None | Some("") | Some("Warn") => Ok(Self::Warn),
            Some("Ignore") => Ok(Self::Ignore),
            Some("Strict") => Ok(Self::Strict),
            Some(other) => Err(ApiError::bad_request(format!(
                "fieldValidation parameter unsupported: {other:?}"
            ))),
        }
    }
}

/// A decoded body and the unknown fields dropped from it.
#[derive(Debug)]
pub struct Decoded {
    pub value: Value,
    pub unknown: Vec<String>,
}

impl Decoded {
    /// Applies `fieldValidation`: the warnings to send, or the error.
    pub fn check(&self, fv: FieldValidation, kind: &str, version: &str) -> ApiResult<Vec<String>> {
        let msgs: Vec<String> = self
            .unknown
            .iter()
            .map(|f| format!("unknown field {f:?}"))
            .collect();
        match fv {
            _ if msgs.is_empty() => Ok(Vec::new()),
            FieldValidation::Ignore => Ok(Vec::new()),
            FieldValidation::Warn => Ok(msgs),
            FieldValidation::Strict => Err(ApiError::bad_request(format!(
                "{kind} in version {version:?} cannot be handled as a {kind}: strict decoding error: {}",
                msgs.join(", ")
            ))),
        }
    }
}

/// Decodes `v` as `schema`, for a `kind` in `version` (error messages).
pub fn decode(v: Value, schema: Schema, kind: &str, version: &str) -> ApiResult<Decoded> {
    let mut d = Decoder {
        unknown: Vec::new(),
    };
    let value = d
        .value(v, schema, &mut String::new())
        .map_err(|e| {
            ApiError::bad_request(format!(
                "{kind} in version {version:?} cannot be handled as a {kind}: {e}"
            ))
        })?
        .unwrap_or_else(|| Value::Object(Map::new()));
    Ok(Decoded {
        value,
        unknown: d.unknown,
    })
}

struct Decoder {
    unknown: Vec<String>,
}

fn json_type(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

fn go_type(s: Schema) -> &'static str {
    match s.ty() {
        Ty::Object { .. } => "struct",
        Ty::Map(_) => "map",
        Ty::Array(_) => "slice",
        Ty::String if s.format() == Some("byte") => "[]uint8",
        Ty::String => "string",
        Ty::Integer if s.format() == Some("int32") => "int32",
        Ty::Integer => "int64",
        Ty::Number => "float64",
        Ty::Boolean => "bool",
        Ty::IntOrString => "intstr.IntOrString",
        Ty::Quantity => "resource.Quantity",
        Ty::Any | Ty::Ref(_) => "interface {}",
    }
}

fn mismatch(v: &Value, s: Schema, path: &str) -> String {
    format!(
        "json: cannot unmarshal {} into Go struct field {} of type {}",
        json_type(v),
        if path.is_empty() { "." } else { path },
        go_type(s)
    )
}

impl Decoder {
    /// `None`: the value decodes to nothing (a `null`).
    fn value(&mut self, v: Value, s: Schema, path: &mut String) -> Result<Option<Value>, String> {
        if v.is_null() {
            return Ok(None);
        }
        let out = match s.ty() {
            Ty::Any => v,
            Ty::Object { .. } => {
                let Value::Object(m) = v else {
                    return Err(mismatch(&v, s, path));
                };
                let preserve = s.preserve_unknown_fields();
                let mut out = Map::new();
                for (k, fv) in m {
                    match s.field(&k) {
                        Some(fs) => {
                            let len = path.len();
                            path.push('.');
                            path.push_str(&k);
                            let r = self.value(fv, fs, path);
                            path.truncate(len);
                            if let Some(fv) = r? {
                                out.insert(k, fv);
                            }
                        }
                        None if preserve => {
                            out.insert(k, fv);
                        }
                        None => {
                            let p = format!("{path}.{k}");
                            self.unknown.push(p.trim_start_matches('.').to_owned());
                        }
                    }
                }
                Value::Object(out)
            }
            Ty::Map(_) => {
                let Value::Object(m) = v else {
                    return Err(mismatch(&v, s, path));
                };
                let vs = s.values().expect("maps have values");
                let mut out = Map::new();
                for (k, mv) in m {
                    let len = path.len();
                    path.push('.');
                    path.push_str(&k);
                    let r = self.value(mv, vs, path);
                    path.truncate(len);
                    // A null map value decodes to the zero value.
                    out.insert(k, r?.unwrap_or_else(|| zero(vs)));
                }
                Value::Object(out)
            }
            Ty::Array(_) => {
                let Value::Array(a) = v else {
                    return Err(mismatch(&v, s, path));
                };
                let items = s.items().expect("arrays have items");
                let mut out = Vec::with_capacity(a.len());
                for (i, iv) in a.into_iter().enumerate() {
                    let len = path.len();
                    path.push_str(&format!("[{i}]"));
                    let r = self.value(iv, items, path);
                    path.truncate(len);
                    out.push(r?.unwrap_or_else(|| zero(items)));
                }
                Value::Array(out)
            }
            Ty::String => match &v {
                Value::String(st) => {
                    match s.format() {
                        Some("byte") if !is_base64(st) => {
                            return Err("illegal base64 data".into());
                        }
                        Some("date-time") if !is_rfc3339(st) => {
                            return Err(format!(
                                "parsing time {st:?} as \"2006-01-02T15:04:05Z07:00\": cannot parse"
                            ));
                        }
                        _ => {}
                    }
                    v
                }
                _ => return Err(mismatch(&v, s, path)),
            },
            Ty::Integer => match &v {
                Value::Number(n) if n.is_i64() || n.is_u64() => {
                    let fits = match s.format() {
                        Some("int32") => n.as_i64().is_some_and(|i| i32::try_from(i).is_ok()),
                        _ => n.is_i64(),
                    };
                    if !fits {
                        return Err(format!(
                            "json: cannot unmarshal number {n} into Go struct field {path} of type {}",
                            go_type(s)
                        ));
                    }
                    v
                }
                Value::Number(n) => {
                    return Err(format!(
                        "json: cannot unmarshal number {n} into Go struct field {path} of type {}",
                        go_type(s)
                    ));
                }
                _ => return Err(mismatch(&v, s, path)),
            },
            Ty::Number => match &v {
                Value::Number(_) => v,
                _ => return Err(mismatch(&v, s, path)),
            },
            Ty::Boolean => match &v {
                Value::Bool(_) => v,
                _ => return Err(mismatch(&v, s, path)),
            },
            Ty::IntOrString => match &v {
                Value::String(_) => v,
                Value::Number(n) if n.as_i64().is_some_and(|i| i32::try_from(i).is_ok()) => v,
                _ => return Err(mismatch(&v, s, path)),
            },
            // Parsed, and re-encoded in canonical form (`1000m` is `1`).
            Ty::Quantity => match &v {
                Value::String(_) | Value::Number(_) => {
                    Value::String(quantity::canonicalize_json(&v, false).map_err(str::to_owned)?)
                }
                _ => return Err(mismatch(&v, s, path)),
            },
            Ty::Ref(_) => unreachable!("ty() resolves refs"),
        };
        Ok(Some(out))
    }
}

/// The JSON a Go zero value of this type encodes to.
fn zero(s: Schema) -> Value {
    match s.ty() {
        Ty::String => Value::String(String::new()),
        Ty::Integer | Ty::Number | Ty::IntOrString => Value::from(0),
        Ty::Quantity => Value::String("0".into()),
        Ty::Boolean => Value::Bool(false),
        Ty::Object { .. } => Value::Object(Map::new()),
        _ => Value::Null,
    }
}

fn is_base64(s: &str) -> bool {
    let b = s.as_bytes();
    if !b.len().is_multiple_of(4) {
        return false;
    }
    let pad = b.iter().rev().take_while(|&&c| c == b'=').count();
    pad <= 2
        && b[..b.len() - pad]
            .iter()
            .all(|c| c.is_ascii_alphanumeric() || *c == b'+' || *c == b'/')
}

fn is_rfc3339(s: &str) -> bool {
    let b = s.as_bytes();
    let digits =
        |r: std::ops::Range<usize>| b.get(r).is_some_and(|x| x.iter().all(u8::is_ascii_digit));
    b.len() >= 20
        && digits(0..4)
        && b[4] == b'-'
        && digits(5..7)
        && b[7] == b'-'
        && digits(8..10)
        && (b[10] == b'T' || b[10] == b't')
        && digits(11..13)
        && b[13] == b':'
        && digits(14..16)
        && b[16] == b':'
        && digits(17..19)
}

#[cfg(test)]
mod tests {
    use super::*;
    use kubemu_api_types::resource;
    use serde_json::json;

    fn pod(v: Value) -> ApiResult<Decoded> {
        decode(v, resource("", "v1", "pods").unwrap().schema(), "Pod", "v1")
    }

    #[test]
    fn prunes_and_reports_unknown_fields() {
        let d = pod(json!({
            "metadata": {"name": "p", "bogus": 1, "labels": {"a": "b"}},
            "spec": {"containers": [{"name": "c", "image": "i", "extra": true}], "nodeName": null},
        }))
        .unwrap();
        assert_eq!(
            d.value,
            json!({"metadata": {"name": "p", "labels": {"a": "b"}}, "spec": {"containers": [{"name": "c", "image": "i"}]}})
        );
        assert_eq!(d.unknown, ["metadata.bogus", "spec.containers[0].extra"]);
        assert_eq!(
            d.check(FieldValidation::Warn, "Pod", "v1").unwrap().len(),
            2
        );
        assert!(
            d.check(FieldValidation::Ignore, "Pod", "v1")
                .unwrap()
                .is_empty()
        );
        assert_eq!(
            d.check(FieldValidation::Strict, "Pod", "v1")
                .unwrap_err()
                .code,
            400
        );
    }

    #[test]
    fn rejects_wrong_types() {
        assert!(pod(json!({"metadata": {"name": 1}})).is_err());
        assert!(pod(json!({"spec": {"containers": {}}})).is_err());
        assert!(pod(json!({"spec": {"priority": 1.5}})).is_err());
        assert!(pod(json!({"spec": {"priority": 4_294_967_296u64}})).is_err());
        assert!(
            pod(json!({"spec": {"containers": [{"resources": {"limits": {"cpu": "lots"}}}]}}))
                .is_err()
        );
        assert!(pod(json!({"spec": {"containers": [{"resources": {"limits": {"cpu": "500m", "memory": 1}}}]}})).is_ok());
        assert!(pod(json!({"metadata": {"creationTimestamp": "yesterday"}})).is_err());
        let secret = resource("", "v1", "secrets").unwrap().schema();
        assert!(
            decode(
                json!({"data": {"k": "not base64!"}}),
                secret,
                "Secret",
                "v1"
            )
            .is_err()
        );
        assert!(decode(json!({"data": {"k": "aGk="}}), secret, "Secret", "v1").is_ok());
    }

    #[test]
    fn quantities_are_canonical() {
        let d = pod(json!({"spec": {"containers": [{"resources": {"limits": {"cpu": "1000m", "memory": 1024}, "requests": {"cpu": "+5"}}}]}})).unwrap();
        assert_eq!(
            d.value["spec"]["containers"][0]["resources"],
            json!({"limits": {"cpu": "1", "memory": "1024"}, "requests": {"cpu": "+5"}})
        );
        let e = pod(json!({"spec": {"containers": [{"resources": {"limits": {"cpu": "1kk"}}}]}}))
            .unwrap_err();
        assert!(
            e.message.ends_with("unable to parse quantity's suffix"),
            "{}",
            e.message
        );
    }
}
