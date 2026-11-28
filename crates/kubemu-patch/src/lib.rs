//! Strategic merge patch, JSON patch, merge patch and server-side apply.
//!
//! The reference for every patch type is what the apiserver's PATCH handler
//! does in the target minor (ADR 0007), not the RFC text. Each one is
//! differentially fuzzed against the Go oracle (`testing/oracle-go`).
//!
//! Done: JSON merge patch, JSON patch, strategic merge patch. Next: SSA (M3).

mod json_patch;
mod merge;
mod smp;

pub use json_patch::{JsonPatchError, MAX_OPERATIONS as MAX_JSON_PATCH_OPERATIONS, json_patch};
pub use merge::{MergePatchError, merge_patch};
pub use smp::{SmpError, strategic_merge_patch};
