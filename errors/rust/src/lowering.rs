//! Cranelift lowering for error ops (Phase 3).
//!
//! Layout: stack_slot with struct<(T payload, i16 error_code)>.
//! error_code == 0 means success.

use mlif::Context;

pub fn lower_errors(_ctx: &mut Context) {
    todo!()
}
