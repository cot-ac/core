//! # Array Operations
//!
//! Defines 3 CIR operations:
//!
//! - `cir.array_init` -- construct an array value from element values (Pure)
//! - `cir.elem_val` -- extract an element value by index from an array (Pure)
//! - `cir.elem_ptr` -- compute a pointer to an element by dynamic index (Pure)
//!
//! All three ops are Pure (no side effects).

use mlif::{
    Attribute, BlockId, Context, Dialect, Location, NamedAttribute, OpDefinition, OpTrait, TypeId,
    ValueId,
};

// ---------------------------------------------------------------------------
// Op registration
// ---------------------------------------------------------------------------

/// Register all 3 array ops with the context's CIR dialect.
pub fn register_ops(ctx: &mut Context) {
    let mut dialect = Dialect::new("cir");

    // array_init -- Pure (variadic elements -> array value)
    dialect.register_op(OpDefinition::new("cir.array_init").with_trait(OpTrait::Pure));

    // elem_val -- Pure (array value + index attr -> element value)
    dialect.register_op(OpDefinition::new("cir.elem_val").with_trait(OpTrait::Pure));

    // elem_ptr -- Pure (base ptr + index value + array_type attr -> ptr to element)
    dialect.register_op(OpDefinition::new("cir.elem_ptr").with_trait(OpTrait::Pure));

    ctx.register_dialect(dialect);
}

// ---------------------------------------------------------------------------
// Builder functions
// ---------------------------------------------------------------------------

/// Build `cir.array_init(%e0, %e1, ...) : !cir.array<N x T>`.
///
/// Creates an array value from its element values. The result type is the
/// array type, and operands are the element values in order.
pub fn build_array_init(
    ctx: &mut Context,
    block: BlockId,
    elements: &[ValueId],
    result_ty: TypeId,
    loc: Location,
) -> ValueId {
    let op = ctx.create_operation(
        "cir.array_init",
        elements,
        &[result_ty],
        vec![],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

/// Build `cir.elem_val %array, <index> : array_ty to elem_ty`.
///
/// Extracts the element at `index` from an array value. The result type
/// is the element type.
pub fn build_elem_val(
    ctx: &mut Context,
    block: BlockId,
    array_val: ValueId,
    index: i64,
    result_ty: TypeId,
    loc: Location,
) -> ValueId {
    let i64_ty = ctx.integer_type(64);
    let op = ctx.create_operation(
        "cir.elem_val",
        &[array_val],
        &[result_ty],
        vec![NamedAttribute::new(
            "index",
            Attribute::Integer {
                value: index,
                ty: i64_ty,
            },
        )],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

/// Build `cir.elem_ptr %base, %idx, <array_type> : (ptr_ty, idx_ty) to ptr_ty`.
///
/// Computes the address of the element at dynamic index `idx` in an array
/// pointed to by `base`. The `array_type` attribute tells the lowering
/// pass the array layout.
pub fn build_elem_ptr(
    ctx: &mut Context,
    block: BlockId,
    base: ValueId,
    idx: ValueId,
    array_type: TypeId,
    result_ty: TypeId,
    loc: Location,
) -> ValueId {
    let op = ctx.create_operation(
        "cir.elem_ptr",
        &[base, idx],
        &[result_ty],
        vec![NamedAttribute::new(
            "array_type",
            Attribute::Type(array_type),
        )],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}
