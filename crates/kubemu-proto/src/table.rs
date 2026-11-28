//! The wire table (`wiretable.json`), generated from the Go types by
//! `mise run wiretable` (`testing/oracle-go/wiretable.go` documents the
//! format). Parsed once, on the first protobuf request.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use serde_json::Value;

const SOURCE: &str = include_str!("../wiretable.json");

#[derive(Debug)]
pub enum Ty {
    String,
    Bool,
    Int32,
    Int64,
    Uint32,
    Uint64,
    Double,
    Bytes,
    Time,
    MicroTime,
    Quantity,
    IntOrString,
    RawExtension,
    FieldsV1,
    Msg(usize),
    List(Box<Ty>),
    Wrap(Box<Ty>),
    Map(Box<Ty>),
}

#[derive(Debug)]
pub struct Field {
    /// 0: a JSON field with no protobuf tag (only its zero value shows).
    pub num: u32,
    /// Empty: an embedded struct, inlined into its parent.
    pub json: String,
    pub ty: Ty,
    pub pointer: bool,
    pub omitempty: bool,
    pub omitzero: bool,
}

#[derive(Debug)]
pub struct Message {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Table {
    pub kubernetes: String,
    pub messages: Vec<Message>,
    /// `<apiVersion>/<Kind>` to message.
    pub kinds: BTreeMap<String, usize>,
}

pub fn table() -> &'static Table {
    static TABLE: OnceLock<Table> = OnceLock::new();
    TABLE.get_or_init(|| parse(SOURCE).unwrap_or_else(|e| panic!("wiretable.json: {e}")))
}

fn parse(src: &str) -> Result<Table, String> {
    let v: Value = serde_json::from_str(src).map_err(|e| e.to_string())?;
    let msgs = v["messages"].as_object().ok_or("no messages")?;
    let index: BTreeMap<&str, usize> = msgs
        .keys()
        .enumerate()
        .map(|(i, k)| (k.as_str(), i))
        .collect();
    let mut messages = Vec::with_capacity(msgs.len());
    for (name, fields) in msgs {
        let fields = fields
            .as_array()
            .ok_or_else(|| format!("{name}: fields"))?
            .iter()
            .map(|f| field(f, &index).map_err(|e| format!("{name}: {e}")))
            .collect::<Result<_, _>>()?;
        messages.push(Message {
            name: name.clone(),
            fields,
        });
    }
    let kinds = v["kinds"]
        .as_object()
        .ok_or("no kinds")?
        .iter()
        .map(|(k, m)| {
            let m = m.as_str().ok_or_else(|| format!("kind {k}"))?;
            let i = *index
                .get(m)
                .ok_or_else(|| format!("kind {k}: no message {m}"))?;
            Ok((k.clone(), i))
        })
        .collect::<Result<_, String>>()?;
    Ok(Table {
        kubernetes: v["kubernetes"].as_str().unwrap_or("").to_owned(),
        messages,
        kinds,
    })
}

fn field(f: &Value, index: &BTreeMap<&str, usize>) -> Result<Field, String> {
    let flag = |k: &str| f[k].as_bool().unwrap_or(false);
    Ok(Field {
        num: f["n"]
            .as_u64()
            .and_then(|n| u32::try_from(n).ok())
            .ok_or("bad n")?,
        json: f["j"].as_str().ok_or("bad j")?.to_owned(),
        ty: ty(&f["t"], index)?,
        pointer: flag("p"),
        omitempty: flag("o"),
        omitzero: flag("z"),
    })
}

fn ty(t: &Value, index: &BTreeMap<&str, usize>) -> Result<Ty, String> {
    if let Some(s) = t.as_str() {
        return Ok(match s {
            "string" => Ty::String,
            "bool" => Ty::Bool,
            "int32" => Ty::Int32,
            "int64" => Ty::Int64,
            "uint32" => Ty::Uint32,
            "uint64" => Ty::Uint64,
            "double" => Ty::Double,
            "bytes" => Ty::Bytes,
            "time" => Ty::Time,
            "microtime" => Ty::MicroTime,
            "quantity" => Ty::Quantity,
            "intorstring" => Ty::IntOrString,
            "rawext" => Ty::RawExtension,
            "fieldsv1" => Ty::FieldsV1,
            // "duration" (metav1.Duration) is in no k8s.io/api type yet.
            other => return Err(format!("unsupported type {other:?}")),
        });
    }
    let (k, v) = t
        .as_object()
        .and_then(|o| o.iter().next())
        .ok_or_else(|| format!("bad type {t}"))?;
    Ok(match k.as_str() {
        "msg" => Ty::Msg(
            *v.as_str()
                .and_then(|m| index.get(m))
                .ok_or_else(|| format!("no message {v}"))?,
        ),
        "list" => Ty::List(Box::new(ty(v, index)?)),
        "wrap" => Ty::Wrap(Box::new(ty(v, index)?)),
        "map" => {
            if v[0] != "string" {
                return Err(format!("map key {}", v[0]));
            }
            Ty::Map(Box::new(ty(&v[1], index)?))
        }
        other => return Err(format!("bad type {other:?}")),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vendored_table_parses() {
        let t = table();
        assert!(t.kubernetes.starts_with("v1."));
        let pod = t.kinds["v1/Pod"];
        assert_eq!(t.messages[pod].name, "core/v1.Pod");
        assert!(t.kinds.contains_key("apps/v1/DeleteOptions"));
    }
}
