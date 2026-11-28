//! Field errors as the apiserver reports them in an `Invalid` status
//! (`k8s.io/apimachinery/pkg/util/validation/field`), messages included.

use std::fmt::Display;

/// One cause of an `Invalid` error: a field and what is wrong with it.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cause {
    /// `FieldValueInvalid`, `FieldValueRequired`, ...
    pub reason: &'static str,
    pub message: String,
    pub field: String,
}

fn cause(reason: &'static str, field: &str, message: String) -> Cause {
    Cause {
        reason,
        message,
        field: field.to_owned(),
    }
}

fn with_detail(body: String, detail: &str) -> String {
    if detail.is_empty() {
        body
    } else {
        format!("{body}: {detail}")
    }
}

/// `field.Required`.
pub fn required(field: &str, detail: &str) -> Cause {
    cause(
        "FieldValueRequired",
        field,
        with_detail("Required value".into(), detail),
    )
}

/// `field.Forbidden`.
pub fn forbidden(field: &str, detail: &str) -> Cause {
    cause(
        "FieldValueForbidden",
        field,
        with_detail("Forbidden".into(), detail),
    )
}

/// `field.Invalid` with a string value (quoted, as `%q`).
pub fn invalid(field: &str, value: &str, detail: &str) -> Cause {
    cause(
        "FieldValueInvalid",
        field,
        with_detail(format!("Invalid value: {value:?}"), detail),
    )
}

/// `field.Invalid` with a number (printed bare).
pub fn invalid_num(field: &str, value: impl Display, detail: &str) -> Cause {
    cause(
        "FieldValueInvalid",
        field,
        with_detail(format!("Invalid value: {value}"), detail),
    )
}

/// `field.NotSupported`: `supported` as upstream lists them.
pub fn not_supported(field: &str, value: &str, supported: &[&str]) -> Cause {
    let list: Vec<String> = supported.iter().map(|s| format!("{s:?}")).collect();
    cause(
        "FieldValueNotSupported",
        field,
        format!(
            "Unsupported value: {value:?}: supported values: {}",
            list.join(", ")
        ),
    )
}

/// `field.Duplicate` with a string value.
pub fn duplicate(field: &str, value: &str) -> Cause {
    cause(
        "FieldValueDuplicate",
        field,
        format!("Duplicate value: {value:?}"),
    )
}

/// `field.NotFound` with a string value.
pub fn not_found(field: &str, value: &str) -> Cause {
    cause("FieldValueNotFound", field, format!("Not found: {value:?}"))
}
