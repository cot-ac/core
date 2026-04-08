//! VWT operations: vwt_size, vwt_stride, vwt_align, vwt_copy,
//! vwt_destroy, vwt_move, vwt_init_buffer.
//!
//! Data queries (size/stride/align) are Pure. Function calls
//! (copy/destroy/move/init_buffer) are not Pure (side effects).
//! Reference: `core/vwt/include/vwt/Ops.td`

use mlif::{BlockId, Context, Dialect, Location, OpDefinition, OpTrait, TypeId, ValueId};

/// Register all 7 VWT ops with the CIR dialect.
pub fn register_ops(ctx: &mut Context) {
    let mut dialect = Dialect::new("cir");

    // Data queries — Pure.
    dialect.register_op(OpDefinition::new("cir.vwt_size").with_trait(OpTrait::Pure));
    dialect.register_op(OpDefinition::new("cir.vwt_stride").with_trait(OpTrait::Pure));
    dialect.register_op(OpDefinition::new("cir.vwt_align").with_trait(OpTrait::Pure));

    // Function calls — NOT Pure (side effects via indirect call).
    dialect.register_op(OpDefinition::new("cir.vwt_copy"));
    dialect.register_op(OpDefinition::new("cir.vwt_destroy"));
    dialect.register_op(OpDefinition::new("cir.vwt_move"));
    dialect.register_op(OpDefinition::new("cir.vwt_init_buffer"));

    ctx.register_dialect(dialect);
}

/// Build `cir.vwt_size %vwt` — query byte size from VWT (index 8).
pub fn build_vwt_size(
    ctx: &mut Context,
    block: BlockId,
    vwt: ValueId,
    result_ty: TypeId,
    loc: Location,
) -> ValueId {
    let op = ctx.create_operation("cir.vwt_size", &[vwt], &[result_ty], vec![], vec![], loc);
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

/// Build `cir.vwt_stride %vwt` — query stride from VWT (index 9).
pub fn build_vwt_stride(
    ctx: &mut Context,
    block: BlockId,
    vwt: ValueId,
    result_ty: TypeId,
    loc: Location,
) -> ValueId {
    let op = ctx.create_operation("cir.vwt_stride", &[vwt], &[result_ty], vec![], vec![], loc);
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

/// Build `cir.vwt_align %vwt` — query alignment from VWT (index 10).
pub fn build_vwt_align(
    ctx: &mut Context,
    block: BlockId,
    vwt: ValueId,
    result_ty: TypeId,
    loc: Location,
) -> ValueId {
    let op = ctx.create_operation("cir.vwt_align", &[vwt], &[result_ty], vec![], vec![], loc);
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

/// Build `cir.vwt_copy %vwt, %src, %dst` — copy via initializeWithCopy (index 2).
pub fn build_vwt_copy(
    ctx: &mut Context,
    block: BlockId,
    vwt: ValueId,
    src: ValueId,
    dst: ValueId,
    loc: Location,
) {
    let op = ctx.create_operation("cir.vwt_copy", &[vwt, src, dst], &[], vec![], vec![], loc);
    ctx.block_push_op(block, op);
}

/// Build `cir.vwt_destroy %vwt, %ptr` — destroy via destroy witness (index 1).
pub fn build_vwt_destroy(
    ctx: &mut Context,
    block: BlockId,
    vwt: ValueId,
    ptr: ValueId,
    loc: Location,
) {
    let op = ctx.create_operation("cir.vwt_destroy", &[vwt, ptr], &[], vec![], vec![], loc);
    ctx.block_push_op(block, op);
}

/// Build `cir.vwt_move %vwt, %src, %dst` — move via initializeWithTake (index 4).
pub fn build_vwt_move(
    ctx: &mut Context,
    block: BlockId,
    vwt: ValueId,
    src: ValueId,
    dst: ValueId,
    loc: Location,
) {
    let op = ctx.create_operation("cir.vwt_move", &[vwt, src, dst], &[], vec![], vec![], loc);
    ctx.block_push_op(block, op);
}

/// Build `cir.vwt_init_buffer %vwt, %src, %dst` — init buffer (index 0).
pub fn build_vwt_init_buffer(
    ctx: &mut Context,
    block: BlockId,
    vwt: ValueId,
    src: ValueId,
    dst: ValueId,
    result_ty: TypeId,
    loc: Location,
) -> ValueId {
    let op = ctx.create_operation(
        "cir.vwt_init_buffer",
        &[vwt, src, dst],
        &[result_ty],
        vec![],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}
