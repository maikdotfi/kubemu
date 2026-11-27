//! Strategic merge patch, JSON patch, merge patch and server-side apply.
//!
//! The reference for every patch type is what the apiserver's PATCH handler
//! does in the target minor (ADR 0007), not the RFC text. Each one is
//! differentially fuzzed against the Go oracle (`testing/oracle-go`).
//!
//! Done: JSON merge patch. Next: SMP and JSON patch (M1), SSA (M3).

mod merge;

pub use merge::{MergePatchError, merge_patch};
