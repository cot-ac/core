//! Lowers CIR enum ops to Cranelift IR instructions.
//!
//! Mirrors C++ enums/Lowering.cpp — 2 patterns:
//! EnumConstantOpLowering (iconst with variant tag index),
//! EnumValueOpLowering (identity — enum IS the tag integer after type conversion).
//!
//! Enums are C-like integer tags (i32 by default).

#![cfg(feature = "codegen")]

use cranelift_codegen::ir::{self as clir, types, InstBuilder};

use mlif::codegen::lowering_ctx::LoweringCtx;
use mlif::entity::OpId;
use mlif::ir::context::Context;
use mlif::ir::types::TypeKind;
use mlif::ConstructLowering;

/// Cranelift lowering for the CIR enums construct (2 ops).
pub struct EnumsLowering;

impl ConstructLowering for EnumsLowering {
    fn name(&self) -> &str {
        "enums"
    }

    fn lower_op(&self, op: OpId, lctx: &mut LoweringCtx) -> Result<bool, String> {
        let handled = match lctx.ir[op].name() {
            "cir.enum_constant" => { lower_enum_constant(op, lctx)?; true }
            "cir.enum_value" => { lower_enum_value(op, lctx)?; true }
            _ => false,
        };
        Ok(handled)
    }

    fn map_type(&self, ctx: &Context, ty: mlif::TypeId) -> Option<clir::Type> {
        match ctx.type_kind(ty) {
            TypeKind::Extension(ext) if ext.dialect == "cir" && ext.name == "enum" => {
                Some(types::I32) // Enums are i32 integer tags
            }
            _ => None,
        }
    }
}

/// Lower `cir.enum_constant` — iconst with the variant's integer index.
/// C++ equivalent: llvm.mlir.constant(variant_index).
fn lower_enum_constant(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let variant_index = lctx.int_attr(op, "variant_index")?;
    let r = lctx.ins().iconst(types::I32, variant_index);
    lctx.set_result(op, r);
    Ok(())
}

/// Lower `cir.enum_value` — identity (enum IS the integer after type conversion).
/// C++ equivalent: identity pattern (convertType makes enum → integer).
fn lower_enum_value(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let input = lctx.unary_operand(op)?;
    lctx.set_result(op, input);
    Ok(())
}
