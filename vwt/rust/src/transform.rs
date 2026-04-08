//! WitnessTableGenerator — stub for Phase 3.
//!
//! Generates VWT globals for each concrete type in the module.
//! Each VWT contains size, stride, alignment, and function pointers
//! for copy, destroy, move, initializeBufferWithCopyOfBuffer.
//!
//! Full implementation deferred to Phase 3 (Cranelift lowering).

use mlif::Context;

pub fn witness_table_generator(_ctx: &mut Context) {
    todo!()
}
