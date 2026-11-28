//! Label and field selectors, as `labels.Parse` and `fields.ParseSelector`
//! read them, and the per-resource field labels the apiserver supports.

use serde_json::Value;

use crate::status::{ApiError, ApiResult};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Op {
    Exists,
    NotExists,
    Eq(String),
    NotEq(String),
    In(Vec<String>),
    NotIn(Vec<String>),
    Gt(i64),
    Lt(i64),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LabelSelector(Vec<(String, Op)>);

impl LabelSelector {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn parse(s: &str) -> ApiResult<Self> {
        let mut p = Lexer {
            s: s.as_bytes(),
            i: 0,
        };
        let mut reqs = Vec::new();
        p.ws();
        if p.done() {
            return Ok(Self(reqs));
        }
        loop {
            reqs.push(
                p.requirement().map_err(|e| {
                    ApiError::bad_request(format!("unable to parse requirement: {e}"))
                })?,
            );
            p.ws();
            if p.done() {
                break;
            }
            if !p.eat(b",") {
                return Err(ApiError::bad_request(format!(
                    "unable to parse requirement: found '{}', expected: ','",
                    p.rest()
                )));
            }
            p.ws();
        }
        Ok(Self(reqs))
    }

    /// Whether an object with `labels` (`metadata.labels`) matches.
    pub fn matches(&self, labels: Option<&Value>) -> bool {
        let get = |k: &str| labels.and_then(|l| l.get(k)).and_then(Value::as_str);
        self.0.iter().all(|(k, op)| {
            let v = get(k);
            match op {
                Op::Exists => v.is_some(),
                Op::NotExists => v.is_none(),
                Op::Eq(x) => v == Some(x.as_str()),
                Op::NotEq(x) => v != Some(x.as_str()),
                Op::In(xs) => v.is_some_and(|v| xs.iter().any(|x| x == v)),
                Op::NotIn(xs) => !v.is_some_and(|v| xs.iter().any(|x| x == v)),
                Op::Gt(n) => v
                    .and_then(|v| v.parse::<i64>().ok())
                    .is_some_and(|v| v > *n),
                Op::Lt(n) => v
                    .and_then(|v| v.parse::<i64>().ok())
                    .is_some_and(|v| v < *n),
            }
        })
    }
}

struct Lexer<'a> {
    s: &'a [u8],
    i: usize,
}

impl Lexer<'_> {
    fn done(&self) -> bool {
        self.i >= self.s.len()
    }

    fn rest(&self) -> String {
        String::from_utf8_lossy(&self.s[self.i.min(self.s.len())..]).into_owned()
    }

    fn ws(&mut self) {
        while self.i < self.s.len() && self.s[self.i].is_ascii_whitespace() {
            self.i += 1;
        }
    }

    fn eat(&mut self, t: &[u8]) -> bool {
        if self.s[self.i..].starts_with(t) {
            self.i += t.len();
            true
        } else {
            false
        }
    }

    /// An identifier or value: anything up to whitespace or a delimiter.
    fn word(&mut self) -> String {
        let start = self.i;
        while self.i < self.s.len()
            && !self.s[self.i].is_ascii_whitespace()
            && !b",=!()<>".contains(&self.s[self.i])
        {
            self.i += 1;
        }
        String::from_utf8_lossy(&self.s[start..self.i]).into_owned()
    }

    fn values(&mut self) -> Result<Vec<String>, String> {
        self.ws();
        if !self.eat(b"(") {
            return Err("found '', expected: '('".into());
        }
        let mut out = Vec::new();
        loop {
            self.ws();
            let w = self.word();
            out.push(w);
            self.ws();
            if self.eat(b")") {
                break;
            }
            if !self.eat(b",") {
                return Err(format!("found '{}', expected: ',', or ')'", self.rest()));
            }
        }
        Ok(out)
    }

    fn requirement(&mut self) -> Result<(String, Op), String> {
        self.ws();
        let not = self.eat(b"!");
        self.ws();
        let key = self.word();
        if key.is_empty() {
            return Err(format!("found '{}', expected: identifier", self.rest()));
        }
        if let Some(e) = label_key_error(&key) {
            return Err(e);
        }
        if not {
            return Ok((key, Op::NotExists));
        }
        self.ws();
        let op =
            if self.done() || self.s[self.i] == b',' {
                Op::Exists
            } else if self.eat(b"==") || self.eat(b"=") {
                self.ws();
                Op::Eq(self.value()?)
            } else if self.eat(b"!=") {
                self.ws();
                Op::NotEq(self.value()?)
            } else if self.eat(b">") {
                self.ws();
                Op::Gt(self.word().parse().map_err(|_| {
                    "for 'Gt', 'Lt' operators, the value must be an integer".to_owned()
                })?)
            } else if self.eat(b"<") {
                self.ws();
                Op::Lt(self.word().parse().map_err(|_| {
                    "for 'Gt', 'Lt' operators, the value must be an integer".to_owned()
                })?)
            } else {
                match self.word().as_str() {
                    "in" => Op::In(self.values()?),
                    "notin" => Op::NotIn(self.values()?),
                    other => {
                        return Err(format!(
                            "found '{other}', expected: in, notin, =, ==, !=, gt, lt"
                        ));
                    }
                }
            };
        Ok((key, op))
    }

    fn value(&mut self) -> Result<String, String> {
        let v = self.word();
        if v.len() > 63 {
            return Err(format!(
                "invalid label value: {v:?}: must be no more than 63 characters"
            ));
        }
        Ok(v)
    }
}

fn label_key_error(k: &str) -> Option<String> {
    let ok = k.split('/').count() <= 2 && !k.ends_with('/') && !k.starts_with('/');
    (!ok).then(|| format!("invalid label key {k:?}"))
}

/// A field selector: `field=value`, `field==value`, `field!=value`, ANDed.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FieldSelector(Vec<(String, bool, String)>);

impl FieldSelector {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Parses and checks every field is supported for the resource.
    pub fn parse(s: &str, group: &str, resource: &str) -> ApiResult<Self> {
        let mut out = Vec::new();
        for term in split_unescaped(s, ',') {
            if term.is_empty() {
                continue;
            }
            let (field, eq, value) = if let Some((f, v)) = split_op(&term, "!=") {
                (f, false, v)
            } else if let Some((f, v)) = split_op(&term, "==") {
                (f, true, v)
            } else if let Some((f, v)) = split_op(&term, "=") {
                (f, true, v)
            } else {
                return Err(ApiError::bad_request(format!(
                    "invalid selector: '{s}'; can't understand '{term}'"
                )));
            };
            let field = field.trim().to_owned();
            if !supported_field(group, resource, &field) {
                return Err(ApiError::bad_request(format!(
                    "field label not supported: {field}"
                )));
            }
            out.push((field, eq, unescape(value.trim())));
        }
        Ok(Self(out))
    }

    pub fn matches(&self, obj: &Value) -> bool {
        self.0
            .iter()
            .all(|(f, eq, v)| (field_value(obj, f) == *v) == *eq)
    }
}

fn split_unescaped(s: &str, sep: char) -> Vec<String> {
    let mut out = vec![String::new()];
    let mut esc = false;
    for c in s.chars() {
        if esc {
            out.last_mut().expect("never empty").push('\\');
            out.last_mut().expect("never empty").push(c);
            esc = false;
        } else if c == '\\' {
            esc = true;
        } else if c == sep {
            out.push(String::new());
        } else {
            out.last_mut().expect("never empty").push(c);
        }
    }
    out
}

/// Splits at the first unescaped `op`.
fn split_op<'a>(term: &'a str, op: &str) -> Option<(&'a str, &'a str)> {
    let b = term.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'\\' {
            i += 2;
            continue;
        }
        if term[i..].starts_with(op) {
            return Some((&term[..i], &term[i + op.len()..]));
        }
        i += 1;
    }
    None
}

fn unescape(s: &str) -> String {
    let mut out = String::new();
    let mut esc = false;
    for c in s.chars() {
        if esc || c != '\\' {
            out.push(c);
            esc = false;
        } else {
            esc = true;
        }
    }
    out
}

/// Field labels each served resource supports (upstream `GetAttrs` and
/// `AddFieldLabelConversionFunc`s), beyond `metadata.name` and
/// `metadata.namespace`.
fn extra_fields(group: &str, resource: &str) -> &'static [&'static str] {
    match (group, resource) {
        ("", "pods") => &[
            "spec.nodeName",
            "spec.restartPolicy",
            "spec.schedulerName",
            "spec.serviceAccountName",
            "spec.hostNetwork",
            "status.phase",
            "status.podIP",
            "status.podIPs",
            "status.nominatedNodeName",
        ],
        ("", "events") => &[
            "involvedObject.kind",
            "involvedObject.namespace",
            "involvedObject.name",
            "involvedObject.uid",
            "involvedObject.apiVersion",
            "involvedObject.resourceVersion",
            "involvedObject.fieldPath",
            "reason",
            "reportingComponent",
            "source",
            "type",
        ],
        ("", "secrets") => &["type"],
        ("", "namespaces") => &["status.phase"],
        ("", "nodes") => &["spec.unschedulable"],
        _ => &[],
    }
}

pub fn supported_field(group: &str, resource: &str, field: &str) -> bool {
    matches!(field, "metadata.name" | "metadata.namespace")
        || extra_fields(group, resource).contains(&field)
}

/// A field label's value on `obj`, as the apiserver renders it.
fn field_value(obj: &Value, field: &str) -> String {
    let path = match field {
        "source" => "source.component",
        f => f,
    };
    let mut cur = obj;
    for seg in path.split('.') {
        match cur.get(seg) {
            Some(v) => cur = v,
            None => {
                return match field {
                    "spec.hostNetwork" | "spec.unschedulable" => "false".into(),
                    _ => String::new(),
                };
            }
        }
    }
    match cur {
        Value::String(s) => s.clone(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        // status.podIPs: the first IP, as upstream indexes it.
        Value::Array(a) => a
            .first()
            .and_then(|v| v.get("ip"))
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_owned(),
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn label_selectors() {
        let l = json!({"app": "web", "tier": "fe", "n": "5"});
        let m = |s: &str| LabelSelector::parse(s).unwrap().matches(Some(&l));
        assert!(m(""));
        assert!(m("app=web"));
        assert!(m("app==web,tier"));
        assert!(!m("app!=web"));
        assert!(m("missing!=x"));
        assert!(m("app in (web, api)"));
        assert!(!m("app notin (web)"));
        assert!(m("missing notin (web)"));
        assert!(m("!missing"));
        assert!(!m("!app"));
        assert!(m("n>4") && !m("n<4"));
        assert!(LabelSelector::parse("app in web").is_err());
        assert!(LabelSelector::parse("a=b c").is_err());
        assert!(LabelSelector::parse("=x").is_err());
    }

    #[test]
    fn field_selectors() {
        let pod = json!({"metadata": {"name": "p", "namespace": "ns"}, "spec": {"nodeName": "n1"}, "status": {"phase": "Running"}});
        let m = |s: &str| FieldSelector::parse(s, "", "pods").unwrap().matches(&pod);
        assert!(m("metadata.name=p"));
        assert!(m("spec.nodeName==n1,status.phase!=Pending"));
        assert!(m("spec.hostNetwork=false"));
        assert!(!m("metadata.namespace=other"));
        assert!(FieldSelector::parse("spec.bogus=x", "", "pods").is_err());
        assert!(FieldSelector::parse("spec.nodeName=x", "", "configmaps").is_err());
        let named = json!({"metadata": {"name": "a,b"}});
        assert!(
            FieldSelector::parse("metadata.name=a\\,b", "", "pods")
                .unwrap()
                .matches(&named)
        );
    }
}
