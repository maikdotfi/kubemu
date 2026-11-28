//! The metadata rules every resource shares (ADR 0013), with the
//! apiserver's messages (`k8s.io/apimachinery/pkg/util/validation`).

use crate::field::{Cause, invalid};

/// Which rule a resource's names follow.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NameRule {
    /// `[a-z0-9]([-a-z0-9]*[a-z0-9])?(\.[a-z0-9]([-a-z0-9]*[a-z0-9])?)*`, max 253.
    Dns1123Subdomain,
    /// `[a-z0-9]([-a-z0-9]*[a-z0-9])?`, max 63.
    Dns1123Label,
    /// `[a-z]([-a-z0-9]*[a-z0-9])?`, max 63.
    Dns1035Label,
}

/// The name rule of a served resource (upstream `ValidateXName` funcs).
pub fn name_rule(group: &str, resource: &str) -> NameRule {
    match (group, resource) {
        ("", "namespaces") => NameRule::Dns1123Label,
        // RelaxedServiceNameValidation (GA in 1.37): labels, not DNS-1035.
        ("", "services") => NameRule::Dns1123Label,
        _ => NameRule::Dns1123Subdomain,
    }
}

const DNS1123_LABEL_MSG: &str = "a lowercase RFC 1123 label must consist of lower case alphanumeric characters or '-', and must start and end with an alphanumeric character (e.g. 'my-name',  or '123-abc', regex used for validation is '[a-z0-9]([-a-z0-9]*[a-z0-9])?')";
const DNS1123_SUBDOMAIN_MSG: &str = "a lowercase RFC 1123 subdomain must consist of lower case alphanumeric characters, '-' or '.', and must start and end with an alphanumeric character (e.g. 'example.com', regex used for validation is '[a-z0-9]([-a-z0-9]*[a-z0-9])?(\\.[a-z0-9]([-a-z0-9]*[a-z0-9])?)*')";
const DNS1035_LABEL_MSG: &str = "a DNS-1035 label must consist of lower case alphanumeric characters or '-', start with an alphabetic character, and end with an alphanumeric character (e.g. 'my-name',  or 'abc-123', regex used for validation is '[a-z]([-a-z0-9]*[a-z0-9])?')";

pub(crate) fn is_label(s: &str, first_alpha: bool) -> bool {
    let b = s.as_bytes();
    let ok = |c: u8| c.is_ascii_lowercase() || c.is_ascii_digit();
    !b.is_empty()
        && if first_alpha {
            b[0].is_ascii_lowercase()
        } else {
            ok(b[0])
        }
        && ok(b[b.len() - 1])
        && b.iter().all(|&c| ok(c) || c == b'-')
}

fn is_subdomain(s: &str) -> bool {
    s.split('.').all(|l| is_label(l, false))
}

/// Problems with a name under `rule`, as upstream reports them.
pub fn name_errors(name: &str, rule: NameRule) -> Vec<String> {
    let mut errs = Vec::new();
    let (max, ok, msg) = match rule {
        NameRule::Dns1123Subdomain => (253, is_subdomain(name), DNS1123_SUBDOMAIN_MSG),
        NameRule::Dns1123Label => (63, is_label(name, false), DNS1123_LABEL_MSG),
        NameRule::Dns1035Label => (63, is_label(name, true), DNS1035_LABEL_MSG),
    };
    if name.len() > max {
        errs.push(format!("must be no more than {max} characters"));
    }
    if !ok {
        // `content.IsDNS1123Label`: a valid subdomain can only be failing
        // on its dots.
        if rule == NameRule::Dns1123Label && is_subdomain(name) {
            errs.push("must not contain dots".to_owned());
        } else {
            errs.push(msg.to_owned());
        }
    }
    errs
}

pub fn validate_name(name: &str, rule: NameRule, causes: &mut Vec<Cause>) {
    for e in name_errors(name, rule) {
        causes.push(invalid("metadata.name", name, &e));
    }
}

pub fn validate_namespace_name(ns: &str, causes: &mut Vec<Cause>) {
    for e in name_errors(ns, NameRule::Dns1123Label) {
        causes.push(invalid("metadata.namespace", ns, &e));
    }
}

const QUALIFIED_NAME_MSG: &str = "name part must consist of alphanumeric characters, '-', '_' or '.', and must start and end with an alphanumeric character (e.g. 'MyName',  or 'my.name',  or '123-abc', regex used for validation is '([A-Za-z0-9][-A-Za-z0-9_.]*)?[A-Za-z0-9]')";
const LABEL_VALUE_MSG: &str = "a valid label must be an empty string or consist of alphanumeric characters, '-', '_' or '.', and must start and end with an alphanumeric character (e.g. 'MyValue',  or 'my_value',  or '12345', regex used for validation is '(([A-Za-z0-9][-A-Za-z0-9_.]*)?[A-Za-z0-9])?')";

fn is_name_part(s: &str) -> bool {
    let b = s.as_bytes();
    let alnum = |c: u8| c.is_ascii_alphanumeric();
    !b.is_empty()
        && alnum(b[0])
        && alnum(b[b.len() - 1])
        && b.iter()
            .all(|&c| alnum(c) || matches!(c, b'-' | b'_' | b'.'))
}

/// `IsQualifiedName`: `[prefix/]name`.
pub(crate) fn qualified_name_errors(key: &str) -> Vec<String> {
    let mut errs = Vec::new();
    let (prefix, name) = match key.split_once('/') {
        Some((p, n)) => (Some(p), n),
        None => (None, key),
    };
    if key.matches('/').count() > 1 {
        return vec!["a qualified name must consist of alphanumeric characters, '-', '_' or '.', and must start and end with an alphanumeric character (e.g. 'MyName',  or 'my.name',  or '123-abc', regex used for validation is '([A-Za-z0-9][-A-Za-z0-9_.]*)?[A-Za-z0-9]') with an optional DNS subdomain prefix and '/' (e.g. 'example.com/MyName')".into()];
    }
    if let Some(p) = prefix {
        if p.is_empty() {
            errs.push("prefix part must be non-empty".into());
        } else {
            for e in name_errors(p, NameRule::Dns1123Subdomain) {
                errs.push(format!("prefix part {e}"));
            }
        }
    }
    if name.is_empty() {
        errs.push("name part must be non-empty".into());
    } else {
        if name.len() > 63 {
            errs.push("name part must be no more than 63 characters".into());
        }
        if !is_name_part(name) {
            errs.push(QUALIFIED_NAME_MSG.into());
        }
    }
    errs
}

pub fn validate_labels(
    labels: &serde_json::Map<String, serde_json::Value>,
    field: &str,
    causes: &mut Vec<Cause>,
) {
    for (k, v) in labels {
        for e in qualified_name_errors(k) {
            causes.push(invalid(field, k, &e));
        }
        let v = v.as_str().unwrap_or_default();
        if v.len() > 63 {
            causes.push(invalid(field, v, "must be no more than 63 characters"));
        }
        if !v.is_empty() && !is_name_part(v) {
            causes.push(invalid(field, v, LABEL_VALUE_MSG));
        }
    }
}

pub fn validate_annotations(
    ann: &serde_json::Map<String, serde_json::Value>,
    causes: &mut Vec<Cause>,
) {
    let mut total = 0;
    for (k, v) in ann {
        for e in qualified_name_errors(&k.to_lowercase()) {
            causes.push(invalid("metadata.annotations", k, &e));
        }
        total += k.len() + v.as_str().map_or(0, str::len);
    }
    if total > 256 * 1024 {
        causes.push(Cause {
            reason: "FieldValueTooLong",
            message: format!("Too long: may not be more than {} bytes", 256 * 1024),
            field: "metadata.annotations".into(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names() {
        use NameRule::*;
        assert!(name_errors("a.b-c", Dns1123Subdomain).is_empty());
        assert!(!name_errors("A", Dns1123Subdomain).is_empty());
        assert!(!name_errors("a.b", Dns1123Label).is_empty());
        assert!(!name_errors("1abc", Dns1035Label).is_empty());
        assert!(name_errors("1abc", Dns1123Label).is_empty());
        assert_eq!(name_errors(&"a".repeat(64), Dns1123Label).len(), 1);
        assert!(qualified_name_errors("example.com/My_Name").is_empty());
        assert!(!qualified_name_errors("/x").is_empty());
        assert!(!qualified_name_errors("a/b/c").is_empty());
    }
}
