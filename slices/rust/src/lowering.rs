//! Lowers CIR slice ops to Cranelift IR instructions.
//!
//! Mirrors C++ slices/Lowering.cpp — 5 patterns:
//! StringConstantOpLowering (global data + {ptr, len} struct),
//! SlicePtrOpLowering (extractvalue [0] → load ptr at offset 0),
//! SliceLenOpLowering (extractvalue [1] → load len at offset 8),
//! SliceElemOpLowering (GEP + load → ptr + index * elem_size + load),
//! ArrayToSliceOpLowering (GEP + len → pointer + element count).
//!
//! Slice layout: `{ ptr: I64 (offset 0), len: I64 (offset 8) }`.
//! All slices are stack-allocated and passed as I64 pointers.

#![cfg(feature = "codegen")]

use cranelift_codegen::ir::{self as clir, types, InstBuilder};

use mlif::codegen::lowering_ctx::LoweringCtx;
use mlif::entity::OpId;
use mlif::ir::context::Context;
use mlif::ir::types::TypeKind;
use mlif::ConstructLowering;

/// Global counter for generating unique data section names.
static DATA_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Cranelift lowering for the CIR slices construct (5 ops).
pub struct SlicesLowering;

impl ConstructLowering for SlicesLowering {
    fn name(&self) -> &str {
        "slices"
    }

    fn lower_op(&self, op: OpId, lctx: &mut LoweringCtx) -> Result<bool, String> {
        let handled = match lctx.ir[op].name() {
            "cir.string_constant" => { lower_string_constant(op, lctx)?; true }
            "cir.slice_ptr" => { lower_slice_ptr(op, lctx)?; true }
            "cir.slice_len" => { lower_slice_len(op, lctx)?; true }
            "cir.slice_elem" => { lower_slice_elem(op, lctx)?; true }
            "cir.array_to_slice" => { lower_array_to_slice(op, lctx)?; true }
            _ => false,
        };
        Ok(handled)
    }

    fn map_type(&self, ctx: &Context, ty: mlif::TypeId) -> Option<clir::Type> {
        match ctx.type_kind(ty) {
            TypeKind::Extension(ext) if ext.dialect == "cir" && ext.name == "slice" => {
                Some(types::I64) // Slices passed as pointers to {ptr, len}
            }
            _ => None,
        }
    }
}

/// Lower `cir.string_constant` — store string bytes in stack slot, build {ptr, len} slice.
/// Uses stack storage instead of global data to avoid AArch64 text-relocation issues.
/// C++ equivalent: GlobalOp + AddressOfOp + InsertValueOp chain.
fn lower_string_constant(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let text = lctx.string_attr(op, "value")?;
    let bytes = text.as_bytes();
    let len = bytes.len() as i64;

    // Allocate stack slot for string data and write bytes.
    let data_ptr = lctx.stack_alloc(bytes.len() as u32);
    for (i, &byte) in bytes.iter().enumerate() {
        let bv = lctx.ins().iconst(types::I8, byte as i64);
        lctx.store_at_offset(bv, data_ptr, i as u32);
    }

    // Build slice struct: alloca {ptr, len}, store both.
    let len_val = lctx.ins().iconst(types::I64, len);
    let slice_ptr = lctx.stack_alloc(16); // {I64, I64}
    lctx.store_at_offset(data_ptr, slice_ptr, 0);
    lctx.store_at_offset(len_val, slice_ptr, 8);

    lctx.set_result(op, slice_ptr);
    Ok(())
}

/// Lower `cir.slice_ptr` — load ptr from slice at offset 0.
/// C++ equivalent: llvm.extractvalue [0].
fn lower_slice_ptr(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let slice = lctx.unary_operand(op)?;
    let r = lctx.load_at_offset(types::I64, slice, 0);
    lctx.set_result(op, r);
    Ok(())
}

/// Lower `cir.slice_len` — load len from slice at offset 8.
/// C++ equivalent: llvm.extractvalue [1].
fn lower_slice_len(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let slice = lctx.unary_operand(op)?;
    let r = lctx.load_at_offset(types::I64, slice, 8);
    lctx.set_result(op, r);
    Ok(())
}

/// Lower `cir.slice_elem` — extract ptr, compute offset, load element.
/// C++ equivalent: extractvalue [0] + GEP + load.
fn lower_slice_elem(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let slice = lctx.operand(op, 0)?;
    let index = lctx.operand(op, 1)?;

    // Get slice's element type from operand type.
    let slice_cir_val = lctx.ir[op].operands()[0];
    let slice_ty = lctx.value_type(slice_cir_val);
    let elem_ty = lctx.ext_type_param(slice_ty, 0)
        .ok_or("cir.slice_elem: missing element type")?;
    let elem_size = lctx.type_size(elem_ty) as i64;

    // Load the data pointer from the slice.
    let data_ptr = lctx.load_at_offset(types::I64, slice, 0);

    // Compute byte offset: index * elem_size.
    let size_val = lctx.ins().iconst(types::I64, elem_size);
    let byte_off = lctx.ins().imul(index, size_val);
    let elem_addr = lctx.ptr_add(data_ptr, byte_off);

    // Load the element.
    let cl_type = lctx.result_cranelift_type(op)?;
    let r = lctx.ins().load(cl_type, cranelift_codegen::ir::MemFlags::new(), elem_addr, 0);
    lctx.set_result(op, r);
    Ok(())
}

/// Lower `cir.array_to_slice` — compute ptr to array start + length.
/// Operands: (base_ptr, start_index, end_index).
/// C++ equivalent: GEP for base + length computation → {ptr, len} struct.
fn lower_array_to_slice(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let array_ptr = lctx.operand(op, 0)?;
    let start_val = lctx.operand(op, 1)?;
    let end_val = lctx.operand(op, 2)?;

    // Get element type from the result slice type.
    let slice_ty = lctx.result_type(op);
    let elem_ty = lctx.ext_type_param(slice_ty, 0)
        .ok_or("cir.array_to_slice: missing element type")?;
    let elem_size = lctx.type_size(elem_ty) as i64;

    // Compute pointer to start element: base + start * elem_size.
    let es = lctx.ins().iconst(types::I64, elem_size);
    let start_off = lctx.ins().imul(start_val, es);
    let data_ptr = lctx.ptr_add(array_ptr, start_off);

    // Compute length: end - start.
    let len_val = lctx.ins().isub(end_val, start_val);

    // Build slice struct: {ptr, len}.
    let slice_ptr = lctx.stack_alloc(16);
    lctx.store_at_offset(data_ptr, slice_ptr, 0);
    lctx.store_at_offset(len_val, slice_ptr, 8);

    lctx.set_result(op, slice_ptr);
    Ok(())
}
