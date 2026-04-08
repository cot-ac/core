//! Enum operations: enum_constant, enum_value.
//!
//! Both ops are Pure.
//! Reference: `core/enums/include/enums/Ops.td`

use mlif::{
    Attribute, BlockId, Context, Dialect, Location, NamedAttribute, OpDefinition, OpTrait, TypeId,
    ValueId,
};

/// Register all 2 enum ops with the CIR dialect.
pub fn register_ops(ctx: &mut Context) {
    let mut dialect = Dialect::new("cir");

    dialect.register_op(OpDefinition::new("cir.enum_constant").with_trait(OpTrait::Pure));
    dialect.register_op(OpDefinition::new("cir.enum_value").with_trait(OpTrait::Pure));

    ctx.register_dialect(dialect);
}

/// Build `cir.enum_constant "Variant" : !cir.enum<...>` — create an enum value.
pub fn build_enum_constant(
    ctx: &mut Context,
    block: BlockId,
    enum_ty: TypeId,
    variant: &str,
    loc: Location,
) -> ValueId {
    let op = ctx.create_operation(
        "cir.enum_constant",
        &[],
        &[enum_ty],
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

/// Build `cir.enum_value %input : tag_type` — extract the integer tag.
pub fn build_enum_value(
    ctx: &mut Context,
    block: BlockId,
    input: ValueId,
    result_ty: TypeId,
    loc: Location,
) -> ValueId {
    let op = ctx.create_operation(
        "cir.enum_value",
        &[input],
        &[result_ty],
        vec![],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}
