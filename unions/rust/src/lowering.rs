//! # Unions Lowering
//!
//! Lowers CIR union ops to Cranelift IR using `{ tag: i32, payload: [N x i8] }`:
//! - `union_init` -> `stack_slot` sized to tag + max payload, store tag + payload bytes
//! - `union_tag` -> `load` the i32 tag from offset 0
//! - `union_payload` -> `load` from payload offset, reinterpreted as the variant type

use mlif::Context;

/// Lower all union ops in a module to Cranelift IR.
pub fn lower_unions(_ctx: &mut Context) {
    todo!()
}
