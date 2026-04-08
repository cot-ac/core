//! Lowers CIR optional ops to Cranelift IR instructions.
//!
//! Layout: `{ T payload (offset 0), i8 has_value (offset sizeof(T)) }`
//! All optional values are stack-allocated and passed as I64 pointers.
//!
//! Mirrors C++ optionals/Lowering.cpp — 4 patterns:
//! NoneOpLowering, WrapOptionalOpLowering,
//! IsNonNullOpLowering, OptionalPayloadOpLowering

#![cfg(feature = "codegen")]

use cranelift_codegen::ir::condcodes::IntCC;
use cranelift_codegen::ir::{self as clir, types, InstBuilder};

use mlif::codegen::lowering_ctx::LoweringCtx;
use mlif::entity::OpId;
use mlif::ir::context::Context;
use mlif::ir::types::TypeKind;
use mlif::ConstructLowering;

/// Cranelift lowering for the CIR optionals construct (4 ops).
pub struct OptionalsLowering;

impl ConstructLowering for OptionalsLowering {
    fn name(&self) -> &str {
        "optionals"
    }

    fn lower_op(&self, op: OpId, lctx: &mut LoweringCtx) -> Result<bool, String> {
        let handled = match lctx.ir[op].name() {
            "cir.none" => { lower_none(op, lctx)?; true }
            "cir.wrap_optional" => { lower_wrap(op, lctx)?; true }
            "cir.is_non_null" => { lower_is_non_null(op, lctx)?; true }
            "cir.optional_payload" => { lower_payload(op, lctx)?; true }
            _ => false,
        };
        Ok(handled)
    }

    fn map_type(&self, ctx: &Context, ty: mlif::TypeId) -> Option<clir::Type> {
        match ctx.type_kind(ty) {
            TypeKind::Extension(ext) if ext.dialect == "cir" && ext.name == "optional" => {
                Some(types::I64)
            }
            _ => None,
        }
    }
}

/// Get the payload byte size for an optional type.
fn payload_size(lctx: &LoweringCtx, opt_ty: mlif::TypeId) -> u32 {
    lctx.ext_type_param(opt_ty, 0)
        .map(|p| lctx.type_size(p))
        .unwrap_or(4)
}

/// Lower `cir.none` — alloca optional struct, zero-fill entire allocation.
/// We zero the full allocation (not just has_value) to ensure no uninitialized
/// bytes leak when the pointer is passed across function boundaries.
fn lower_none(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let opt_ty = lctx.result_type(op);
    let total = lctx.type_size(opt_ty);

    let ptr = lctx.stack_alloc(total);

    // Zero-fill the entire allocation word by word.
    let mut off = 0u32;
    while off + 4 <= total {
        let z = lctx.ins().iconst(types::I32, 0);
        lctx.store_at_offset(z, ptr, off);
        off += 4;
    }
    while off < total {
        let z = lctx.ins().iconst(types::I8, 0);
        lctx.store_at_offset(z, ptr, off);
        off += 1;
    }

    lctx.set_result(op, ptr);
    Ok(())
}

/// Lower `cir.wrap_optional` — alloca, store payload + has_value = 1.
fn lower_wrap(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let input = lctx.unary_operand(op)?;
    let opt_ty = lctx.result_type(op);
    let total = lctx.type_size(opt_ty);
    let p_size = payload_size(lctx, opt_ty);

    let ptr = lctx.stack_alloc(total);
    lctx.store_at_offset(input, ptr, 0);
    let one = lctx.ins().iconst(types::I8, 1);
    lctx.store_at_offset(one, ptr, p_size);

    lctx.set_result(op, ptr);
    Ok(())
}

/// Lower `cir.is_non_null` — load has_value, icmp ne 0.
fn lower_is_non_null(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let input = lctx.unary_operand(op)?;
    let input_cir = lctx.ir[op].operands()[0];
    let opt_ty = lctx.value_type(input_cir);
    let p_size = payload_size(lctx, opt_ty);

    let has_value = lctx.load_at_offset(types::I8, input, p_size);
    let zero = lctx.ins().iconst(types::I8, 0);
    let cmp = lctx.ins().icmp(IntCC::NotEqual, has_value, zero);

    lctx.set_result(op, cmp);
    Ok(())
}

/// Lower `cir.optional_payload` — load payload from offset 0.
fn lower_payload(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let input = lctx.unary_operand(op)?;
    let cl_type = lctx.result_cranelift_type(op)?;
    let r = lctx.load_at_offset(cl_type, input, 0);
    lctx.set_result(op, r);
    Ok(())
}
