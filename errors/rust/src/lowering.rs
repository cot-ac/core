//! Lowers CIR error union ops to Cranelift IR instructions.
//!
//! Layout: `{ T payload (offset 0), i16 error_code (offset sizeof(T)) }`
//! error_code == 0 means success. All error unions are stack-allocated, passed as I64 pointers.
//!
//! Mirrors C++ errors/Lowering.cpp — 5 patterns:
//! WrapResultOpLowering, WrapErrorOpLowering, IsErrorOpLowering,
//! ErrorPayloadOpLowering, ErrorCodeOpLowering

#![cfg(feature = "codegen")]

use cranelift_codegen::ir::condcodes::IntCC;
use cranelift_codegen::ir::{self as clir, types, InstBuilder};

use mlif::codegen::lowering_ctx::LoweringCtx;
use mlif::entity::OpId;
use mlif::ir::context::Context;
use mlif::ir::types::TypeKind;
use mlif::ConstructLowering;

/// Cranelift lowering for the CIR errors construct (5 ops).
pub struct ErrorsLowering;

impl ConstructLowering for ErrorsLowering {
    fn name(&self) -> &str {
        "errors"
    }

    fn lower_op(&self, op: OpId, lctx: &mut LoweringCtx) -> Result<bool, String> {
        let handled = match lctx.ir[op].name() {
            "cir.wrap_result" => { lower_wrap_result(op, lctx)?; true }
            "cir.wrap_error" => { lower_wrap_error(op, lctx)?; true }
            "cir.is_error" => { lower_is_error(op, lctx)?; true }
            "cir.error_payload" => { lower_error_payload(op, lctx)?; true }
            "cir.error_code" => { lower_error_code(op, lctx)?; true }
            _ => false,
        };
        Ok(handled)
    }

    fn map_type(&self, ctx: &Context, ty: mlif::TypeId) -> Option<clir::Type> {
        match ctx.type_kind(ty) {
            TypeKind::Extension(ext) if ext.dialect == "cir" && ext.name == "error_union" => {
                Some(types::I64)
            }
            _ => None,
        }
    }
}

/// Get payload byte size for an error_union type.
fn payload_size(lctx: &LoweringCtx, eu_ty: mlif::TypeId) -> u32 {
    lctx.ext_type_param(eu_ty, 0)
        .map(|p| lctx.type_size(p))
        .unwrap_or(4)
}

/// Lower `cir.wrap_result` — alloca, store payload + error_code = 0.
fn lower_wrap_result(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let input = lctx.unary_operand(op)?;
    let eu_ty = lctx.result_type(op);
    let total = lctx.type_size(eu_ty);
    let p_size = payload_size(lctx, eu_ty);

    let ptr = lctx.stack_alloc(total);
    lctx.store_at_offset(input, ptr, 0);
    let zero = lctx.ins().iconst(types::I16, 0);
    lctx.store_at_offset(zero, ptr, p_size);

    lctx.set_result(op, ptr);
    Ok(())
}

/// Lower `cir.wrap_error` — alloca, store error_code (payload undefined).
fn lower_wrap_error(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let code_val = lctx.unary_operand(op)?;
    let eu_ty = lctx.result_type(op);
    let total = lctx.type_size(eu_ty);
    let p_size = payload_size(lctx, eu_ty);

    let ptr = lctx.stack_alloc(total);
    lctx.store_at_offset(code_val, ptr, p_size);

    lctx.set_result(op, ptr);
    Ok(())
}

/// Lower `cir.is_error` — load error_code, icmp ne 0.
fn lower_is_error(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let input = lctx.unary_operand(op)?;
    let input_cir = lctx.ir[op].operands()[0];
    let eu_ty = lctx.value_type(input_cir);
    let p_size = payload_size(lctx, eu_ty);

    let code = lctx.load_at_offset(types::I16, input, p_size);
    let zero = lctx.ins().iconst(types::I16, 0);
    let cmp = lctx.ins().icmp(IntCC::NotEqual, code, zero);

    lctx.set_result(op, cmp);
    Ok(())
}

/// Lower `cir.error_payload` — load payload from offset 0.
fn lower_error_payload(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let input = lctx.unary_operand(op)?;
    let cl_type = lctx.result_cranelift_type(op)?;
    let r = lctx.load_at_offset(cl_type, input, 0);
    lctx.set_result(op, r);
    Ok(())
}

/// Lower `cir.error_code` — load i16 from offset payload_size.
fn lower_error_code(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let input = lctx.unary_operand(op)?;
    let input_cir = lctx.ir[op].operands()[0];
    let eu_ty = lctx.value_type(input_cir);
    let p_size = payload_size(lctx, eu_ty);

    let code = lctx.load_at_offset(types::I16, input, p_size);
    lctx.set_result(op, code);
    Ok(())
}
