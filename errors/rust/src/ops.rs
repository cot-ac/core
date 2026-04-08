//! Error operations: wrap_result, wrap_error, is_error, error_payload, error_code.
//!
//! All 5 ops are Pure (no side effects).
//! Reference: `core/errors/include/errors/Ops.td`

use mlif::{BlockId, Context, Dialect, Location, OpDefinition, OpTrait, TypeId, ValueId};

use crate::types::error_union_type;

/// Register all 5 error ops with the CIR dialect.
pub fn register_ops(ctx: &mut Context) {
    let mut dialect = Dialect::new("cir");

    dialect.register_op(OpDefinition::new("cir.wrap_result").with_trait(OpTrait::Pure));
    dialect.register_op(OpDefinition::new("cir.wrap_error").with_trait(OpTrait::Pure));
    dialect.register_op(OpDefinition::new("cir.is_error").with_trait(OpTrait::Pure));
    dialect.register_op(OpDefinition::new("cir.error_payload").with_trait(OpTrait::Pure));
    dialect.register_op(OpDefinition::new("cir.error_code").with_trait(OpTrait::Pure));

    ctx.register_dialect(dialect);
}

/// Build `cir.wrap_result %input : T to !cir.error_union<T>`.
pub fn build_wrap_result(
    ctx: &mut Context,
    block: BlockId,
    input: ValueId,
    loc: Location,
) -> ValueId {
    let payload_ty = ctx.value_type(input);
    let eu_ty = error_union_type(ctx, payload_ty);
    let op = ctx.create_operation("cir.wrap_result", &[input], &[eu_ty], vec![], vec![], loc);
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

/// Build `cir.wrap_error %code : !cir.error_union<T>`.
/// `code` must be i16. `payload_ty` specifies the T in error_union<T>.
pub fn build_wrap_error(
    ctx: &mut Context,
    block: BlockId,
    code: ValueId,
    payload_ty: TypeId,
    loc: Location,
) -> ValueId {
    let eu_ty = error_union_type(ctx, payload_ty);
    let op = ctx.create_operation("cir.wrap_error", &[code], &[eu_ty], vec![], vec![], loc);
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

/// Build `cir.is_error %input : i1` — check if error union is an error.
pub fn build_is_error(
    ctx: &mut Context,
    block: BlockId,
    input: ValueId,
    loc: Location,
) -> ValueId {
    let i1_ty = ctx.integer_type(1);
    let op = ctx.create_operation("cir.is_error", &[input], &[i1_ty], vec![], vec![], loc);
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

/// Build `cir.error_payload %input : !cir.error_union<T> to T`.
pub fn build_error_payload(
    ctx: &mut Context,
    block: BlockId,
    input: ValueId,
    result_ty: TypeId,
    loc: Location,
) -> ValueId {
    let op = ctx.create_operation(
        "cir.error_payload",
        &[input],
        &[result_ty],
        vec![],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

/// Build `cir.error_code %input : i16` — extract the error code.
pub fn build_error_code(
    ctx: &mut Context,
    block: BlockId,
    input: ValueId,
    loc: Location,
) -> ValueId {
    let i16_ty = ctx.integer_type(16);
    let op = ctx.create_operation("cir.error_code", &[input], &[i16_ty], vec![], vec![], loc);
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}
