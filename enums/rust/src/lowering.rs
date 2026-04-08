//! Cranelift lowering for enum ops (Phase 3).
//!
//! enum_constant → iconst (variant's integer index).
//! enum_value → identity (enum IS the integer).

use mlif::Context;

pub fn lower_enums(_ctx: &mut Context) {
    todo!()
}
