//! # Slice Operations
//!
//! Defines 5 CIR operations:
//!
//! - `cir.string_constant` -- create a `!cir.slice<i8>` from a string literal (Pure)
//! - `cir.slice_ptr` -- extract the base pointer from a slice value (Pure)
//! - `cir.slice_len` -- extract the length from a slice value (Pure)
//! - `cir.slice_elem` -- load an element from a slice by index (NOT Pure -- memory read)
//! - `cir.array_to_slice` -- create a slice from a pointer and start/end indices (Pure)
//!
//! All ops are Pure except `cir.slice_elem`, which performs a memory read.

use mlif::{
    Attribute, BlockId, Context, Dialect, Location, NamedAttribute, OpDefinition, OpTrait, TypeId,
    ValueId,
};

// ---------------------------------------------------------------------------
// Op registration
// ---------------------------------------------------------------------------

/// Register all 5 slice ops with the context's CIR dialect.
pub fn register_ops(ctx: &mut Context) {
    let mut dialect = Dialect::new("cir");

    // string_constant -- Pure (no operands, string attr -> slice<i8>)
    dialect.register_op(OpDefinition::new("cir.string_constant").with_trait(OpTrait::Pure));

    // slice_ptr -- Pure (slice -> ptr)
    dialect.register_op(OpDefinition::new("cir.slice_ptr").with_trait(OpTrait::Pure));

    // slice_len -- Pure (slice -> i64)
    dialect.register_op(OpDefinition::new("cir.slice_len").with_trait(OpTrait::Pure));

    // slice_elem -- NOT Pure (memory read: slice + index -> element value)
    // Errata E6: performs a memory read, not a pure computation.
    dialect.register_op(OpDefinition::new("cir.slice_elem"));

    // array_to_slice -- Pure (base_ptr + start + end -> slice)
    dialect.register_op(OpDefinition::new("cir.array_to_slice").with_trait(OpTrait::Pure));

    ctx.register_dialect(dialect);
}

// ---------------------------------------------------------------------------
// Builder functions
// ---------------------------------------------------------------------------

/// Build `cir.string_constant "hello" : !cir.slice<i8>`.
///
/// Creates a constant slice pointing to a null-terminated global string.
/// The result type must be `!cir.slice<i8>`.
pub fn build_string_constant(
    ctx: &mut Context,
    block: BlockId,
    value: &str,
    result_ty: TypeId,
    loc: Location,
) -> ValueId {
    let op = ctx.create_operation(
        "cir.string_constant",
        &[],
        &[result_ty],
        vec![NamedAttribute::new(
            "value",
            Attribute::String(value.to_string()),
        )],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

/// Build `cir.slice_ptr %slice : slice_ty to ptr_ty`.
///
/// Extracts the data pointer from a slice value.
pub fn build_slice_ptr(
    ctx: &mut Context,
    block: BlockId,
    slice: ValueId,
    result_ty: TypeId,
    loc: Location,
) -> ValueId {
    let op = ctx.create_operation(
        "cir.slice_ptr",
        &[slice],
        &[result_ty],
        vec![],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

/// Build `cir.slice_len %slice : slice_ty`.
///
/// Extracts the length from a slice value as i64.
pub fn build_slice_len(
    ctx: &mut Context,
    block: BlockId,
    slice: ValueId,
    loc: Location,
) -> ValueId {
    let i64_ty = ctx.integer_type(64);
    let op = ctx.create_operation(
        "cir.slice_len",
        &[slice],
        &[i64_ty],
        vec![],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

/// Build `cir.slice_elem %slice, %idx : slice_ty, idx_ty to elem_ty`.
///
/// Loads the element at position `idx` from the slice. This is NOT a pure
/// operation -- it performs a memory read (Errata E6).
pub fn build_slice_elem(
    ctx: &mut Context,
    block: BlockId,
    slice: ValueId,
    idx: ValueId,
    result_ty: TypeId,
    loc: Location,
) -> ValueId {
    let op = ctx.create_operation(
        "cir.slice_elem",
        &[slice, idx],
        &[result_ty],
        vec![],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

/// Build `cir.array_to_slice %base, %start, %end : (ptr_ty, idx_ty, idx_ty) to slice_ty`.
///
/// Creates a slice from a base pointer and start/end indices. The slice
/// covers elements [start, end).
pub fn build_array_to_slice(
    ctx: &mut Context,
    block: BlockId,
    base: ValueId,
    start: ValueId,
    end: ValueId,
    result_ty: TypeId,
    loc: Location,
) -> ValueId {
    let op = ctx.create_operation(
        "cir.array_to_slice",
        &[base, start, end],
        &[result_ty],
        vec![],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}
