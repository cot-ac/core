//! # Slice Operations
//!
//! Defines 5 CIR operations:
//!
//! - `string_constant` — create a `!cir.slice<i8>` from a string literal
//! - `slice_ptr` — extract the base pointer from a slice value
//! - `slice_len` — extract the length from a slice value
//! - `slice_elem` — index into a slice to get an element value
//! - `array_to_slice` — convert a `!cir.array<N x T>` pointer to a `!cir.slice<T>`

use mlif::Context;

/// Register all 5 slice ops with the given context.
pub fn register_ops(_ctx: &mut Context) {
    todo!()
}
