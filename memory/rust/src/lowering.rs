//! # Memory Lowering
//!
//! Lowers CIR memory ops to Cranelift IR:
//! - `alloca` -> `stack_slot` allocation
//! - `store` -> Cranelift `store` instruction
//! - `load` -> Cranelift `load` instruction
//! - `addr_of` -> `stack_addr` for the corresponding slot
//! - `deref` -> `load` with type derived from `!cir.ref<T>`

use mlif::Context;

/// Lower all memory ops in a module to Cranelift IR.
pub fn lower_memory(_ctx: &mut Context) {
    todo!()
}
