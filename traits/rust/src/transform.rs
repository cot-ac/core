//! WitnessThunkGenerator — stub for Phase 3.
//!
//! Generates thunk functions that bridge between witness-table calling
//! convention and concrete method signatures. The thunk casts the opaque
//! self pointer back to the concrete type.
//!
//! This is a lowering-level concern, not a sema step. Full implementation
//! deferred to Phase 3 (Cranelift lowering).

use mlif::Context;

pub fn witness_thunk_generator(_ctx: &mut Context) {
    todo!()
}
