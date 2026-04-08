//! Test operations: assert, test_case.
//!
//! assert is not Pure (traps on failure — side effect).
//! test_case is not Pure (region op — NoTerminator).
//! Reference: `core/test/include/test/Ops.td`

use mlif::{
    Attribute, BlockId, Context, Dialect, Location, NamedAttribute, OpDefinition, RegionId,
    ValueId,
};

/// Register all 2 test ops with the CIR dialect.
pub fn register_ops(ctx: &mut Context) {
    let mut dialect = Dialect::new("cir");

    // assert traps on failure — not Pure.
    dialect.register_op(OpDefinition::new("cir.assert"));
    // test_case is a region op (NoTerminator) — not Pure.
    dialect.register_op(OpDefinition::new("cir.test_case"));

    ctx.register_dialect(dialect);
}

/// Build `cir.assert %condition, "message"`.
pub fn build_assert(
    ctx: &mut Context,
    block: BlockId,
    condition: ValueId,
    message: &str,
    loc: Location,
) {
    let op = ctx.create_operation(
        "cir.assert",
        &[condition],
        &[],
        vec![NamedAttribute::new(
            "message",
            Attribute::String(message.to_string()),
        )],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
}

/// Build `cir.test_case "name" { body }`. Returns the op and its body region.
///
/// The caller should populate the body region's entry block with test ops.
pub fn build_test_case(
    ctx: &mut Context,
    block: BlockId,
    name: &str,
    loc: Location,
) -> (mlif::OpId, RegionId) {
    let body_block = ctx.create_block();
    let body_region = ctx.create_region();
    ctx.region_push_block(body_region, body_block);

    let op = ctx.create_operation(
        "cir.test_case",
        &[],
        &[],
        vec![NamedAttribute::new(
            "name",
            Attribute::String(name.to_string()),
        )],
        vec![body_region],
        loc,
    );
    ctx.block_push_op(block, op);
    (op, body_region)
}
