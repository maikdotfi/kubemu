//! Kubernetes `Status` errors (`metav1.Status`), as the apiserver returns
//! them: the HTTP code, a machine-readable `reason`, and details naming
//! the object.

use std::fmt;

use serde_json::{Value, json};

/// A failed request. Rendered as a `Status` object.
#[derive(Clone, Debug, PartialEq)]
pub struct ApiError {
    pub code: u16,
    /// `NotFound`, `AlreadyExists`, `Conflict`, `Invalid`, ...
    pub reason: &'static str,
    pub message: String,
    pub details: Option<Box<Details>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Details {
    pub name: String,
    pub group: String,
    pub kind: String,
    pub causes: Vec<Cause>,
    pub retry_after_seconds: Option<u32>,
}

pub use kubemu_validation::Cause;

pub type ApiResult<T> = Result<T, ApiError>;

/// `group/kind/name` of the object an error is about. `kind` here is what
/// the apiserver puts in details: the lower-case plural resource.
#[derive(Clone, Copy, Debug)]
pub struct Subject<'a> {
    pub group: &'a str,
    pub resource: &'a str,
    pub name: &'a str,
}

impl Subject<'_> {
    /// `pods "x"` or `deployments.apps "x"`, as messages name objects.
    fn quoted(&self) -> String {
        if self.group.is_empty() {
            format!("{} {:?}", self.resource, self.name)
        } else {
            format!("{}.{} {:?}", self.resource, self.group, self.name)
        }
    }

    fn details(&self) -> Details {
        Details {
            name: self.name.to_owned(),
            group: self.group.to_owned(),
            kind: self.resource.to_owned(),
            ..Details::default()
        }
    }
}

impl ApiError {
    pub fn new(code: u16, reason: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            reason,
            message: message.into(),
            details: None,
        }
    }

    fn with(mut self, d: Details) -> Self {
        self.details = Some(Box::new(d));
        self
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(400, "BadRequest", message)
    }

    pub fn not_found(s: Subject<'_>) -> Self {
        Self::new(404, "NotFound", format!("{} not found", s.quoted())).with(s.details())
    }

    /// A path or resource kubemu does not serve.
    pub fn not_found_path(what: impl fmt::Display) -> Self {
        Self::new(
            404,
            "NotFound",
            format!("the server could not find the requested resource ({what})"),
        )
    }

    pub fn already_exists(s: Subject<'_>) -> Self {
        Self::new(
            409,
            "AlreadyExists",
            format!("{} already exists", s.quoted()),
        )
        .with(s.details())
    }

    pub fn conflict(s: Subject<'_>, why: &str) -> Self {
        Self::new(
            409,
            "Conflict",
            format!("Operation cannot be fulfilled on {}: {why}", s.quoted()),
        )
        .with(s.details())
    }

    /// The optimistic-concurrency conflict clients retry on.
    pub fn modified(s: Subject<'_>) -> Self {
        Self::conflict(
            s,
            "the object has been modified; please apply your changes to the latest version and try again",
        )
    }

    /// `Kind "name" is invalid: <causes>` (422).
    pub fn invalid(kind: &str, group: &str, name: &str, causes: Vec<Cause>) -> Self {
        let list: Vec<String> = causes
            .iter()
            .map(|c| format!("{}: {}", c.field, c.message))
            .collect();
        let message = match list.as_slice() {
            [one] => format!("{kind} {name:?} is invalid: {one}"),
            many => format!("{kind} {name:?} is invalid: [{}]", many.join(", ")),
        };
        Self::new(422, "Invalid", message).with(Details {
            name: name.to_owned(),
            group: group.to_owned(),
            kind: kind.to_owned(),
            causes,
            retry_after_seconds: None,
        })
    }

    pub fn forbidden(s: Subject<'_>, why: &str) -> Self {
        Self::new(
            403,
            "Forbidden",
            format!("{} is forbidden: {why}", s.quoted()),
        )
        .with(s.details())
    }

    pub fn method_not_allowed(message: impl Into<String>) -> Self {
        Self::new(405, "MethodNotAllowed", message)
    }

    pub fn gone(message: impl Into<String>) -> Self {
        Self::new(410, "Expired", message)
    }

    pub fn unsupported_media_type(message: impl Into<String>) -> Self {
        Self::new(415, "UnsupportedMediaType", message)
    }

    pub fn not_acceptable(message: impl Into<String>) -> Self {
        Self::new(406, "NotAcceptable", message)
    }

    pub fn unprocessable(message: impl Into<String>) -> Self {
        Self::new(422, "UnprocessableEntity", message)
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(500, "InternalError", message)
    }

    /// A resourceVersion newer than the store has (504, retried by
    /// clients).
    pub fn too_large_resource_version(rv: u64, current: u64) -> Self {
        let mut e = Self::new(
            504,
            "Timeout",
            format!("Too large resource version: {rv}, current: {current}"),
        );
        e.details = Some(Box::new(Details {
            causes: vec![Cause {
                reason: "ResourceVersionTooLarge",
                message: "Too large resource version".into(),
                field: String::new(),
            }],
            retry_after_seconds: Some(1),
            ..Details::default()
        }));
        e
    }

    /// The `Status` object.
    pub fn to_status(&self) -> Value {
        let mut s = json!({
            "kind": "Status",
            "apiVersion": "v1",
            "metadata": {},
            "status": "Failure",
            "message": self.message,
            "reason": self.reason,
            "code": self.code,
        });
        if let Some(d) = &self.details {
            let mut dv = serde_json::Map::new();
            let mut put = |k: &str, v: &str| {
                if !v.is_empty() {
                    dv.insert(k.into(), json!(v));
                }
            };
            put("name", &d.name);
            put("group", &d.group);
            put("kind", &d.kind);
            if !d.causes.is_empty() {
                dv.insert(
                    "causes".into(),
                    d.causes
                        .iter()
                        .map(|c| {
                            let mut m = serde_json::Map::new();
                            m.insert("reason".into(), json!(c.reason));
                            m.insert("message".into(), json!(c.message));
                            if !c.field.is_empty() {
                                m.insert("field".into(), json!(c.field));
                            }
                            Value::Object(m)
                        })
                        .collect(),
                );
            }
            if let Some(r) = d.retry_after_seconds {
                dv.insert("retryAfterSeconds".into(), json!(r));
            }
            s["details"] = Value::Object(dv);
        }
        s
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}): {}", self.code, self.reason, self.message)
    }
}

impl std::error::Error for ApiError {}

/// A `Status` reporting success, as deletes of gone objects return.
pub fn success(s: Subject<'_>, uid: Option<&str>) -> Value {
    let mut details = json!({"name": s.name, "kind": s.resource});
    if !s.group.is_empty() {
        details["group"] = json!(s.group);
    }
    if let Some(uid) = uid {
        details["uid"] = json!(uid);
    }
    json!({
        "kind": "Status",
        "apiVersion": "v1",
        "metadata": {},
        "status": "Success",
        "details": details,
    })
}
