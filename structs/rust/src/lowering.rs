//! Lowers CIR struct ops to Cranelift IR instructions.
//!
//! Mirrors C++ structs/Lowering.cpp — 3 patterns:
//! StructInitOpLowering (undef + insertvalue chain → alloca + store chain),
//! FieldValOpLowering (extractvalue → load at offset),
//! FieldPtrOpLowering (GEP → pointer arithmetic).
//!
//! Structs are stack-allocated aggregates passed as I64 pointers.

#![cfg(feature = "codegen")]

use cranelift_codegen::ir::{self as clir, types};

use mlif::codegen::lowering_ctx::LoweringCtx;
use mlif::entity::OpId;
use mlif::ir::context::Context;
use mlif::ir::types::TypeKind;
use mlif::ConstructLowering;

/// Cranelift lowering for the CIR structs construct (3 ops).
pub struct StructsLowering;

impl ConstructLowering for StructsLowering {
    fn name(&self) -> &str {
        "structs"
    }

    fn lower_op(&self, op: OpId, lctx: &mut LoweringCtx) -> Result<bool, String> {
        let handled = match lctx.ir[op].name() {
            "cir.struct_init" => { lower_struct_init(op, lctx)?; true }
            "cir.field_val" => { lower_field_val(op, lctx)?; true }
            "cir.field_ptr" => { lower_field_ptr(op, lctx)?; true }
            _ => false,
        };
        Ok(handled)
    }

    fn map_type(&self, ctx: &Context, ty: mlif::TypeId) -> Option<clir::Type> {
        match ctx.type_kind(ty) {
            TypeKind::Extension(ext) if ext.dialect == "cir" && ext.name == "struct" => {
                Some(types::I64) // Structs passed as pointers
            }
            _ => None,
        }
    }
}

/// Lower `cir.struct_init` — alloca + store each field at its offset.
/// C++ equivalent: undef + insertvalue chain.
fn lower_struct_init(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let struct_ty = lctx.result_type(op);
    let total = lctx.type_size(struct_ty);
    let field_count = lctx.field_count(struct_ty);

    let ptr = lctx.stack_alloc(total);

    let operands = lctx.all_operands(op)?;
    for i in 0..field_count {
        let offset = lctx.field_offset(struct_ty, i);
        lctx.store_at_offset(operands[i], ptr, offset);
    }

    lctx.set_result(op, ptr);
    Ok(())
}

/// Lower `cir.field_val` — load field value from struct pointer at offset.
/// C++ equivalent: llvm.extractvalue.
fn lower_field_val(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let struct_ptr = lctx.unary_operand(op)?;
    let field_idx = lctx.int_attr(op, "field_index")? as usize;

    // Get the struct type from the operand.
    let struct_cir_val = lctx.ir[op].operands()[0];
    let struct_ty = lctx.value_type(struct_cir_val);
    let offset = lctx.field_offset(struct_ty, field_idx);

    let cl_type = lctx.result_cranelift_type(op)?;
    let r = lctx.load_at_offset(cl_type, struct_ptr, offset);
    lctx.set_result(op, r);
    Ok(())
}

/// Lower `cir.field_ptr` — compute pointer to field at offset.
/// C++ equivalent: llvm.getelementptr [0, idx].
fn lower_field_ptr(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let struct_ptr = lctx.unary_operand(op)?;
    let field_idx = lctx.int_attr(op, "field_index")? as usize;

    let struct_cir_val = lctx.ir[op].operands()[0];
    let struct_ty = lctx.value_type(struct_cir_val);
    let offset = lctx.field_offset(struct_ty, field_idx);

    let r = lctx.addr_at_offset(struct_ptr, offset);
    lctx.set_result(op, r);
    Ok(())
}
