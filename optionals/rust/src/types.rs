//! # Optional Types
//!
//! Defines the CIR optional type:
//!
//! `!cir.optional<T>` — a value that is either a valid `T` or null/none.
//! For pointer-like T, uses null-pointer optimization (no extra storage).
//! For value types, represented as a tagged struct: `{ has_value: i1, payload: T }`.

use mlif::Context;

/// Optional type: `!cir.optional<T>`.
pub struct OptionalType {
    // TODO: payload type
}

/// Register `!cir.optional` with the type system.
pub fn register_types(_ctx: &mut Context) {
    todo!()
}
