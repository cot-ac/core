//! # Array Types
//!
//! Defines the CIR array type:
//!
//! `!cir.array<N x T>` — a fixed-size array of N elements of type T. The size
//! N is a compile-time constant. Total byte size = N * sizeof(T).

use mlif::Context;

/// Fixed-size array type: `!cir.array<N x T>`.
pub struct ArrayType {
    // TODO: element count, element type
}

/// Register `!cir.array` with the type system.
pub fn register_types(_ctx: &mut Context) {
    todo!()
}
