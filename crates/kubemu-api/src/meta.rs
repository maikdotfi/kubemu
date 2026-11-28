//! Object metadata: UIDs and timestamps drawn from [`Env`], and the
//! metadata rules every resource shares (ADR 0013), with the apiserver's
//! messages (`k8s.io/apimachinery/pkg/util/validation`).

use kubemu_env::Env;

use crate::status::Cause;

/// A random (v4) UUID from the Env's RNG.
pub fn uuid_v4(env: &Env) -> String {
    let (a, b) = (env.rng.next_u64(), env.rng.next_u64());
    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&a.to_be_bytes());
    bytes[8..].copy_from_slice(&b.to_be_bytes());
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    let h: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
    format!(
        "{}-{}-{}-{}-{}",
        &h[0..8],
        &h[8..12],
        &h[12..16],
        &h[16..20],
        &h[20..32]
    )
}

/// The Env's wall clock as RFC 3339 with second precision
/// (`metav1.Time`).
pub fn now_rfc3339(env: &Env) -> String {
    rfc3339(env.clock.wall().0 / 1_000_000_000)
}

/// Unix seconds as `YYYY-MM-DDTHH:MM:SSZ`.
pub fn rfc3339(unix: u64) -> String {
    let days = (unix / 86_400) as i64;
    let secs = unix % 86_400;
    // Howard Hinnant's civil_from_days.
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!(
        "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}Z",
        secs / 3600,
        secs / 60 % 60,
        secs % 60
    )
}

/// `generateName` suffix: five characters from the apiserver's alphabet
/// (no vowels, no confusable digits).
pub fn name_suffix(env: &Env) -> String {
    const ALPHABET: &[u8] = b"bcdfghjklmnpqrstvwxz2456789";
    (0..5)
        .map(|_| ALPHABET[env.rng.below(ALPHABET.len() as u64) as usize] as char)
        .collect()
}

/// The apiserver truncates `generateName` so the full name fits.
pub fn generate_name(prefix: &str, env: &Env) -> String {
    const MAX_PREFIX: usize = 63 - 5;
    let mut p = prefix;
    if p.len() > MAX_PREFIX {
        p = &p[..MAX_PREFIX];
    }
    format!("{p}{}", name_suffix(env))
}

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
        ("", "services") => NameRule::Dns1035Label,
        _ => NameRule::Dns1123Subdomain,
    }
}

const DNS1123_LABEL_MSG: &str = "a lowercase RFC 1123 label must consist of lower case alphanumeric characters or '-', and must start and end with an alphanumeric character (e.g. 'my-name',  or '123-abc', regex used for validation is '[a-z0-9]([-a-z0-9]*[a-z0-9])?')";
const DNS1123_SUBDOMAIN_MSG: &str = "a lowercase RFC 1123 subdomain must consist of lower case alphanumeric characters, '-' or '.', and must start and end with an alphanumeric character (e.g. 'example.com', regex used for validation is '[a-z0-9]([-a-z0-9]*[a-z0-9])?(\\.[a-z0-9]([-a-z0-9]*[a-z0-9])?)*')";
const DNS1035_LABEL_MSG: &str = "a DNS-1035 label must consist of lower case alphanumeric characters or '-', start with an alphabetic character, and end with an alphanumeric character (e.g. 'my-name',  or 'abc-123', regex used for validation is '[a-z]([-a-z0-9]*[a-z0-9])?')";

fn is_label(s: &str, first_alpha: bool) -> bool {
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
        errs.push(msg.to_owned());
    }
    errs
}

fn invalid(field: &str, value: &str, msg: &str) -> Cause {
    Cause {
        reason: "FieldValueInvalid",
        message: format!("Invalid value: {value:?}: {msg}"),
        field: field.to_owned(),
    }
}

pub fn required(field: &str, msg: &str) -> Cause {
    Cause {
        reason: "FieldValueRequired",
        message: if msg.is_empty() {
            "Required value".into()
        } else {
            format!("Required value: {msg}")
        },
        field: field.to_owned(),
    }
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
fn qualified_name_errors(key: &str) -> Vec<String> {
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
    use kubemu_env_sim::SimEnv;

    #[test]
    fn uuid_shape() {
        let env = SimEnv::new(1).env;
        let u = uuid_v4(&env);
        assert_eq!(u.len(), 36);
        assert_eq!(&u[14..15], "4");
        assert!(matches!(&u[19..20], "8" | "9" | "a" | "b"));
        assert_ne!(u, uuid_v4(&env));
    }

    #[test]
    fn timestamps() {
        assert_eq!(rfc3339(0), "1970-01-01T00:00:00Z");
        assert_eq!(rfc3339(1_735_689_600), "2025-01-01T00:00:00Z");
        assert_eq!(rfc3339(951_782_400 + 3661), "2000-02-29T01:01:01Z");
    }

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

    #[test]
    fn generated_names() {
        let env = SimEnv::new(1).env;
        let n = generate_name("web-", &env);
        assert!(n.starts_with("web-") && n.len() == 9);
        assert_eq!(generate_name(&"x".repeat(80), &env).len(), 63);
    }
}
