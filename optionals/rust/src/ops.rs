//! # Optional Operations
//!
//! Defines 4 CIR operations:
//!
//! - `none` — create a null/none optional of a given type
//! - `wrap_optional` — wrap a non-null value into an optional
//! - `is_non_null` — test whether an optional has a value (returns i1)
//! - `optional_payload` — extract the payload value (UB if none)

use mlif::Context;

/// Register all 4 optional ops with the given context.
pub fn register_ops(_ctx: &mut Context) {
    todo!()
}
