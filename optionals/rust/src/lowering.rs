//! Cranelift lowering for optional ops (Phase 3).
//!
//! Pointer-like payloads: null-pointer optimization (I64, 0 = none).
//! Value payloads: stack_slot (T + 1-byte has_value flag).

use mlif::Context;

pub fn lower_optionals(_ctx: &mut Context) {
    todo!()
}
