//! # Trait Transforms
//!
//! Contains the WitnessThunkGenerator pass. This pass generates thunk
//! functions for each protocol method that bridge between the generic
//! witness-table calling convention and the concrete type's method signature.
//! Thunks handle the `self` parameter cast from opaque pointer to concrete
//! type pointer.

use mlif::Context;

/// Run the WitnessThunkGenerator pass: create thunks for witness table entries.
pub fn witness_thunk_generator(_ctx: &mut Context) {
    todo!()
}
