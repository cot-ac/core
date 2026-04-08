//! Cranelift lowering for trait ops (Phase 3).
//!
//! witness_table → global data with function pointer array.
//! trait_call → resolved by GenericSpecializerStep to direct func.call.
//! witness_method → load fn ptr from PWT at method_index.
//! init_existential → store value + VWT + PWT into container.
//! open_existential → extract buffer, VWT, PWT pointers.
//! deinit_existential → null out metadata slots.

use mlif::Context;

pub fn lower_traits(_ctx: &mut Context) {
    todo!()
}
