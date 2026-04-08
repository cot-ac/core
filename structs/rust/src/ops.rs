//! # Struct Operations
//!
//! Defines 3 CIR operations:
//!
//! - `cir.struct_init` — construct a struct value from field values (Pure)
//! - `cir.field_val` — extract a field value by index from a struct (Pure)
//! - `cir.field_ptr` — compute a pointer to a field within a struct pointer (Pure)
//!
//! All three ops are Pure (no side effects).

use mlif::{
    Attribute, BlockId, Context, Dialect, Location, NamedAttribute, OpDefinition, OpTrait, TypeId,
    ValueId,
};

// ---------------------------------------------------------------------------
// Op registration
// ---------------------------------------------------------------------------

/// Register all 3 struct ops with the context's CIR dialect.
pub fn register_ops(ctx: &mut Context) {
    let mut dialect = Dialect::new("cir");

    // struct_init — Pure (variadic fields -> struct value)
    dialect.register_op(OpDefinition::new("cir.struct_init").with_trait(OpTrait::Pure));

    // field_val — Pure (struct value + index attr -> field value)
    dialect.register_op(OpDefinition::new("cir.field_val").with_trait(OpTrait::Pure));

    // field_ptr — Pure (base ptr + index attr + struct_type attr -> ptr to field)
    dialect.register_op(OpDefinition::new("cir.field_ptr").with_trait(OpTrait::Pure));

    ctx.register_dialect(dialect);
}

// ---------------------------------------------------------------------------
// Builder functions
// ---------------------------------------------------------------------------

/// Build `cir.struct_init(%f0, %f1, ...) : !cir.struct<...>`.
///
/// Creates a struct value from its constituent field values. The result type
/// is the struct type, and operands are the field values in order.
pub fn build_struct_init(
    ctx: &mut Context,
    block: BlockId,
    fields: &[ValueId],
    result_ty: TypeId,
    loc: Location,
) -> ValueId {
    let op = ctx.create_operation("cir.struct_init", fields, &[result_ty], vec![], vec![], loc);
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

/// Build `cir.field_val %struct, <index> : struct_ty to field_ty`.
///
/// Extracts the field at `index` from a struct value. The result type is
/// the field's type.
pub fn build_field_val(
    ctx: &mut Context,
    block: BlockId,
    struct_val: ValueId,
    index: i64,
    result_ty: TypeId,
    loc: Location,
) -> ValueId {
    let i64_ty = ctx.integer_type(64);
    let op = ctx.create_operation(
        "cir.field_val",
        &[struct_val],
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

/// Build `cir.field_ptr %base, <index>, <struct_type> : ptr_ty to ptr_ty`.
///
/// Computes the address of field `index` in a struct pointed to by `base`.
/// The `struct_type` attribute tells the lowering pass the struct layout.
pub fn build_field_ptr(
    ctx: &mut Context,
    block: BlockId,
    base: ValueId,
    index: i64,
    struct_type: TypeId,
    result_ty: TypeId,
    loc: Location,
) -> ValueId {
    let i64_ty = ctx.integer_type(64);
    let op = ctx.create_operation(
        "cir.field_ptr",
        &[base],
        &[result_ty],
        vec![
            NamedAttribute::new(
                "index",
                Attribute::Integer {
                    value: index,
                    ty: i64_ty,
                },
            ),
            NamedAttribute::new("struct_type", Attribute::Type(struct_type)),
        ],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}
