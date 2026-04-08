//! # Traits Lowering
//!
//! Lowers CIR trait ops to Cranelift IR using indirect calls:
//! - `witness_table` -> global data section with function pointer array
//! - `trait_call` -> load witness table ptr, GEP to method slot, `call_indirect`
//! - `witness_method` -> load function pointer from witness table at method index
//! - `init_existential` -> construct `{ value_ptr, witness_table_ptr }` pair
//! - `open_existential` -> extract value_ptr from the pair
//! - `deinit_existential` -> load destroy method from witness table, `call_indirect`

use mlif::Context;

/// Lower all trait ops in a module to Cranelift IR.
pub fn lower_traits(_ctx: &mut Context) {
    todo!()
}
