//! Defaulting and validation, ported from upstream only where users observe
//! them, each rule checked against the Go oracle (ADR 0013). Plus
//! `resource.Quantity`, which both need.
//!
//! Everything here is a pure function of JSON objects: core code, run under
//! DST.

pub mod defaults;
mod image;
pub mod quantity;

pub use defaults::default;
