//! # Enums Lowering
//!
//! Lowers CIR enum ops to Cranelift IR as plain integer constants:
//! - `enum_constant` -> `iconst` with the variant's integer value
//! - `enum_value` -> identity (the enum is already an integer at this level)

use mlif::Context;

/// Lower all enum ops in a module to Cranelift IR.
pub fn lower_enums(_ctx: &mut Context) {
    todo!()
}
