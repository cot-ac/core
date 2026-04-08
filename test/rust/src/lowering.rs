//! Cranelift lowering for test ops (Phase 3).
//!
//! assert → brif on condition: false → trap block, true → continue.
//! test_case → already extracted to function by TestSemaStep.

use mlif::Context;

pub fn lower_test(_ctx: &mut Context) {
    todo!()
}
