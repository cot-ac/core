//! # Error Operations
//!
//! Defines 5 CIR operations:
//!
//! - `wrap_result` — wrap a success value into an error union (error_code = 0)
//! - `wrap_error` — wrap an i16 error code into an error union (no payload)
//! - `is_error` — test whether an error union is an error (returns i1)
//! - `error_payload` — extract the success payload (UB if is_error)
//! - `error_code` — extract the i16 error code (0 if success)

use mlif::Context;

/// Register all 5 error ops with the given context.
pub fn register_ops(_ctx: &mut Context) {
    todo!()
}
