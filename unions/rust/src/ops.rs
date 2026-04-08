//! Union operations: union_init, union_tag, union_payload.
//!
//! All 3 ops are Pure.
//! Reference: `core/unions/include/unions/Ops.td`

use mlif::{
    Attribute, BlockId, Context, Dialect, Location, NamedAttribute, OpDefinition, OpTrait, TypeId,
    ValueId,
};

/// Register all 3 union ops with the CIR dialect.
pub fn register_ops(ctx: &mut Context) {
    let mut dialect = Dialect::new("cir");

    dialect.register_op(OpDefinition::new("cir.union_init").with_trait(OpTrait::Pure));
    dialect.register_op(OpDefinition::new("cir.union_tag").with_trait(OpTrait::Pure));
    dialect.register_op(OpDefinition::new("cir.union_payload").with_trait(OpTrait::Pure));

    ctx.register_dialect(dialect);
}

/// Build `cir.union_init "Variant" %payload : !cir.tagged_union<...>`.
/// `payload` may be absent for payload-less variants — pass `None`.
pub fn build_union_init(
    ctx: &mut Context,
    block: BlockId,
    union_ty: TypeId,
    variant: &str,
    payload: Option<ValueId>,
    loc: Location,
) -> ValueId {
    let operands: Vec<ValueId> = payload.into_iter().collect();
    let op = ctx.create_operation(
        "cir.union_init",
        &operands,
        &[union_ty],
        vec![NamedAttribute::new(
            "variant",
            Attribute::String(variant.to_string()),
        )],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

/// Build `cir.union_tag %input : i8` — extract the discriminator tag.
pub fn build_union_tag(
    ctx: &mut Context,
    block: BlockId,
    input: ValueId,
    loc: Location,
) -> ValueId {
    let i8_ty = ctx.integer_type(8);
    let op = ctx.create_operation("cir.union_tag", &[input], &[i8_ty], vec![], vec![], loc);
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

/// Build `cir.union_payload "Variant" %input : result_type`.
pub fn build_union_payload(
    ctx: &mut Context,
    block: BlockId,
    input: ValueId,
    variant: &str,
    result_ty: TypeId,
    loc: Location,
) -> ValueId {
    let op = ctx.create_operation(
        "cir.union_payload",
        &[input],
        &[result_ty],
        vec![NamedAttribute::new(
            "variant",
            Attribute::String(variant.to_string()),
        )],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}
