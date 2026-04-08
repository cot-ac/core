//! # Slice Types
//!
//! Defines the CIR slice type:
//!
//! `!cir.slice<T>` — a fat pointer consisting of a base pointer and a length.
//! Represented at runtime as a two-field struct: `{ ptr: *T, len: i64 }`.

use mlif::Context;

/// Slice type: `!cir.slice<T>`.
pub struct SliceType {
    // TODO: element type
}

/// Register `!cir.slice` with the type system.
pub fn register_types(_ctx: &mut Context) {
    todo!()
}
