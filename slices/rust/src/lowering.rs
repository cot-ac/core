//! # Slices Lowering
//!
//! Lowers CIR slice ops to Cranelift IR using a two-value (ptr, len) struct:
//! - `string_constant` -> global data section + `global_value` for ptr, `iconst` for len
//! - `slice_ptr` -> extract first element of the pair
//! - `slice_len` -> extract second element of the pair
//! - `slice_elem` -> ptr + index * sizeof(T), then `load`
//! - `array_to_slice` -> `stack_addr` for ptr, `iconst N` for len

use mlif::Context;

/// Lower all slice ops in a module to Cranelift IR.
pub fn lower_slices(_ctx: &mut Context) {
    todo!()
}
