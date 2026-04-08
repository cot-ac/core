//! # Error Union Types
//!
//! Defines the CIR error union type:
//!
//! `!cir.error_union<T>` — a tagged union of a success payload `T` or an i16
//! error code. Represented at runtime as a struct:
//! `{ payload: T, error_code: i16 }` where error_code == 0 means success.

use mlif::Context;

/// Error union type: `!cir.error_union<T>`.
pub struct ErrorUnionType {
    // TODO: payload type
}

/// Register `!cir.error_union` with the type system.
pub fn register_types(_ctx: &mut Context) {
    todo!()
}
