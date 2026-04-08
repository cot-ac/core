//! Lowers CIR array ops to Cranelift IR instructions.
//!
//! Mirrors C++ arrays/Lowering.cpp — 3 patterns:
//! ArrayInitOpLowering (undef + insertvalue chain → alloca + store chain),
//! ElemValOpLowering (extractvalue → load at dynamic offset),
//! ElemPtrOpLowering (GEP → pointer arithmetic with index * elem_size).
//!
//! Arrays are stack-allocated aggregates passed as I64 pointers.

#![cfg(feature = "codegen")]

use cranelift_codegen::ir::{self as clir, types};

use mlif::codegen::lowering_ctx::LoweringCtx;
use mlif::entity::OpId;
use mlif::ir::context::Context;
use mlif::ir::types::TypeKind;
use mlif::ConstructLowering;

/// Cranelift lowering for the CIR arrays construct (3 ops).
pub struct ArraysLowering;

impl ConstructLowering for ArraysLowering {
    fn name(&self) -> &str {
        "arrays"
    }

    fn lower_op(&self, op: OpId, lctx: &mut LoweringCtx) -> Result<bool, String> {
        let handled = match lctx.ir[op].name() {
            "cir.array_init" => { lower_array_init(op, lctx)?; true }
            "cir.elem_val" => { lower_elem_val(op, lctx)?; true }
            "cir.elem_ptr" => { lower_elem_ptr(op, lctx)?; true }
            _ => false,
        };
        Ok(handled)
    }

    fn map_type(&self, ctx: &Context, ty: mlif::TypeId) -> Option<clir::Type> {
        match ctx.type_kind(ty) {
            TypeKind::Extension(ext) if ext.dialect == "cir" && ext.name == "array" => {
                Some(types::I64) // Arrays passed as pointers
            }
            _ => None,
        }
    }
}

/// Lower `cir.array_init` — alloca + store each element at stride offset.
/// C++ equivalent: undef + insertvalue chain.
fn lower_array_init(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let array_ty = lctx.result_type(op);
    let total = lctx.type_size(array_ty);
    let elem_ty = lctx.ext_type_param(array_ty, 0)
        .ok_or("cir.array_init: missing element type")?;
    let elem_size = lctx.type_size(elem_ty);

    let ptr = lctx.stack_alloc(total);

    let operands = lctx.all_operands(op)?;
    for (i, val) in operands.iter().enumerate() {
        let offset = (i as u32) * elem_size;
        lctx.store_at_offset(*val, ptr, offset);
    }

    lctx.set_result(op, ptr);
    Ok(())
}

/// Lower `cir.elem_val` — load element at dynamic index from array pointer.
/// C++ equivalent: llvm.extractvalue.
fn lower_elem_val(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let array_ptr = lctx.operand(op, 0)?;
    let index = lctx.operand(op, 1)?;

    // Get the array type from operand 0.
    let array_cir_val = lctx.ir[op].operands()[0];
    let array_ty = lctx.value_type(array_cir_val);

    let byte_offset = lctx.elem_byte_offset(array_ty, index);
    let cl_type = lctx.result_cranelift_type(op)?;
    let r = lctx.load_dynamic(cl_type, array_ptr, byte_offset);
    lctx.set_result(op, r);
    Ok(())
}

/// Lower `cir.elem_ptr` — compute pointer to element at dynamic index.
/// C++ equivalent: llvm.getelementptr [0, idx].
fn lower_elem_ptr(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let array_ptr = lctx.operand(op, 0)?;
    let index = lctx.operand(op, 1)?;

    let array_cir_val = lctx.ir[op].operands()[0];
    let array_ty = lctx.value_type(array_cir_val);

    let byte_offset = lctx.elem_byte_offset(array_ty, index);
    let r = lctx.ptr_add(array_ptr, byte_offset);
    lctx.set_result(op, r);
    Ok(())
}
