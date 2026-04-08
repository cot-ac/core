//! Optional operations: none, wrap_optional, is_non_null, optional_payload.
//!
//! All 4 ops are Pure (no side effects).
//! Reference: `core/optionals/include/optionals/Ops.td`

use mlif::{BlockId, Context, Dialect, Location, OpDefinition, OpTrait, TypeId, ValueId};

use crate::types::optional_type;

/// Register all 4 optional ops with the CIR dialect.
pub fn register_ops(ctx: &mut Context) {
    let mut dialect = Dialect::new("cir");

    dialect.register_op(OpDefinition::new("cir.none").with_trait(OpTrait::Pure));
    dialect.register_op(OpDefinition::new("cir.wrap_optional").with_trait(OpTrait::Pure));
    dialect.register_op(OpDefinition::new("cir.is_non_null").with_trait(OpTrait::Pure));
    dialect.register_op(OpDefinition::new("cir.optional_payload").with_trait(OpTrait::Pure));

    ctx.register_dialect(dialect);
}

/// Build `cir.none : !cir.optional<T>` — create a null optional.
pub fn build_none(
    ctx: &mut Context,
    block: BlockId,
    payload_ty: TypeId,
    loc: Location,
) -> ValueId {
    let opt_ty = optional_type(ctx, payload_ty);
    let op = ctx.create_operation("cir.none", &[], &[opt_ty], vec![], vec![], loc);
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

/// Build `cir.wrap_optional %input : T to !cir.optional<T>`.
pub fn build_wrap_optional(
    ctx: &mut Context,
    block: BlockId,
    input: ValueId,
    loc: Location,
) -> ValueId {
    let payload_ty = ctx.value_type(input);
    let opt_ty = optional_type(ctx, payload_ty);
    let op = ctx.create_operation("cir.wrap_optional", &[input], &[opt_ty], vec![], vec![], loc);
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

/// Build `cir.is_non_null %input : i1` — check if optional has a value.
pub fn build_is_non_null(
    ctx: &mut Context,
    block: BlockId,
    input: ValueId,
    loc: Location,
) -> ValueId {
    let i1_ty = ctx.integer_type(1);
    let op = ctx.create_operation("cir.is_non_null", &[input], &[i1_ty], vec![], vec![], loc);
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

/// Build `cir.optional_payload %input : !cir.optional<T> to T`.
pub fn build_optional_payload(
    ctx: &mut Context,
    block: BlockId,
    input: ValueId,
    result_ty: TypeId,
    loc: Location,
) -> ValueId {
    let op = ctx.create_operation(
        "cir.optional_payload",
        &[input],
        &[result_ty],
        vec![],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}
