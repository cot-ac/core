//! Lowers CIR tagged union ops to Cranelift IR instructions.
//!
//! Mirrors C++ unions/Lowering.cpp — 3 patterns:
//! UnionInitOpLowering (tag + store payload bytes),
//! UnionTagOpLowering (load i8 at offset 0),
//! UnionPayloadOpLowering (load from offset 1 as target type).
//!
//! Layout: `{ i8 tag (offset 0), [max_payload_bytes x i8] payload (offset 1) }`
//! Tagged unions are stack-allocated and passed as I64 pointers.

#![cfg(feature = "codegen")]

use cranelift_codegen::ir::{self as clir, types, InstBuilder};

use mlif::codegen::lowering_ctx::LoweringCtx;
use mlif::entity::OpId;
use mlif::ir::context::Context;
use mlif::ir::types::TypeKind;
use mlif::ConstructLowering;

/// Cranelift lowering for the CIR unions construct (3 ops).
pub struct UnionsLowering;

impl ConstructLowering for UnionsLowering {
    fn name(&self) -> &str {
        "unions"
    }

    fn lower_op(&self, op: OpId, lctx: &mut LoweringCtx) -> Result<bool, String> {
        let handled = match lctx.ir[op].name() {
            "cir.union_init" => { lower_union_init(op, lctx)?; true }
            "cir.union_tag" => { lower_union_tag(op, lctx)?; true }
            "cir.union_payload" => { lower_union_payload(op, lctx)?; true }
            _ => false,
        };
        Ok(handled)
    }

    fn map_type(&self, ctx: &Context, ty: mlif::TypeId) -> Option<clir::Type> {
        match ctx.type_kind(ty) {
            TypeKind::Extension(ext) if ext.dialect == "cir" && ext.name == "tagged_union" => {
                Some(types::I64)
            }
            _ => None,
        }
    }
}

/// Compute max payload byte size across all variants.
fn max_payload_size(lctx: &LoweringCtx, union_ty: mlif::TypeId) -> u32 {
    let count = lctx.ext_type_param_count(union_ty);
    let mut max = 0u32;
    for i in 0..count {
        if let Some(vty) = lctx.ext_type_param(union_ty, i) {
            let s = lctx.type_size(vty);
            if s > max { max = s; }
        }
    }
    max.max(1)
}

/// Find variant index by name in the union type.
fn variant_index(lctx: &LoweringCtx, union_ty: mlif::TypeId, variant: &str) -> Result<i64, String> {
    let count = lctx.ext_string_param_count(union_ty);
    for i in 1..count {
        if let Some(name) = lctx.ext_string_param(union_ty, i) {
            if name == variant {
                return Ok((i - 1) as i64);
            }
        }
    }
    Err(format!("unknown variant '{}'", variant))
}

/// Lower `cir.union_init "Variant" %payload?` — alloc + store tag + store payload.
/// C++ equivalent: undef + insertvalue(tag) + alloca-store-load bitcast.
fn lower_union_init(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let union_ty = lctx.result_type(op);
    let max_payload = max_payload_size(lctx, union_ty);
    let total = 1 + max_payload;

    let variant = lctx.string_attr(op, "variant")?;
    let tag_idx = variant_index(lctx, union_ty, &variant)?;

    let ptr = lctx.stack_alloc(total);

    // Store tag at offset 0.
    let tag_val = lctx.ins().iconst(types::I8, tag_idx);
    lctx.store_at_offset(tag_val, ptr, 0);

    // Store payload at offset 1 if present.
    if lctx.num_operands(op) > 0 {
        let payload = lctx.operand(op, 0)?;
        lctx.store_at_offset(payload, ptr, 1);
    }

    lctx.set_result(op, ptr);
    Ok(())
}

/// Lower `cir.union_tag %input : i8` — load discriminator tag.
/// C++ equivalent: llvm.extractvalue [0].
fn lower_union_tag(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let input = lctx.unary_operand(op)?;
    let r = lctx.load_at_offset(types::I8, input, 0);
    lctx.set_result(op, r);
    Ok(())
}

/// Lower `cir.union_payload "Variant" %input : result_type`.
/// C++ equivalent: extractvalue [1] + alloca-store-load reinterpret cast.
fn lower_union_payload(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let input = lctx.unary_operand(op)?;
    let cl_type = lctx.result_cranelift_type(op)?;
    let r = lctx.load_at_offset(cl_type, input, 1);
    lctx.set_result(op, r);
    Ok(())
}
