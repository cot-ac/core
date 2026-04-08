//! # Arrays Lowering
//!
//! Lowers CIR array ops to Cranelift IR:
//! - `array_init` -> `stack_slot` + sequence of `store` at element offsets
//! - `elem_val` -> `load` at index * sizeof(T) from array base
//! - `elem_ptr` -> `stack_addr` + index * sizeof(T) computation (imul + iadd)

use mlif::Context;

/// Lower all array ops in a module to Cranelift IR.
pub fn lower_arrays(_ctx: &mut Context) {
    todo!()
}
