//! Generic operations: generic_apply.
//!
//! One op that calls a generic function with type substitutions.
//! Resolved by GenericSpecializerStep into func.call.
//! Reference: `core/generics/include/generics/Ops.td`

use mlif::{
    Attribute, BlockId, Context, Dialect, Location, NamedAttribute, OpDefinition, TypeId, ValueId,
};

/// Register the generic_apply op with the CIR dialect.
pub fn register_ops(ctx: &mut Context) {
    let mut dialect = Dialect::new("cir");
    // generic_apply is NOT Pure — it has a callee symbol reference.
    dialect.register_op(OpDefinition::new("cir.generic_apply"));
    ctx.register_dialect(dialect);
}

/// Build `cir.generic_apply @callee(%args) subs ["T" = i32] : (...) -> (...)`.
///
/// - `callee`: name of the generic function
/// - `args`: argument values
/// - `sub_keys`: type parameter names (e.g., ["T", "U"])
/// - `sub_types`: concrete types to substitute (e.g., [i32_ty, f64_ty])
/// - `result_types`: result types after substitution
pub fn build_generic_apply(
    ctx: &mut Context,
    block: BlockId,
    callee: &str,
    args: &[ValueId],
    sub_keys: &[&str],
    sub_types: &[TypeId],
    result_types: &[TypeId],
    loc: Location,
) -> Vec<ValueId> {
    let sub_keys_attr = Attribute::Array(
        sub_keys
            .iter()
            .map(|k| Attribute::String(k.to_string()))
            .collect(),
    );
    let sub_types_attr = Attribute::Array(sub_types.iter().map(|&t| Attribute::Type(t)).collect());

    let op = ctx.create_operation(
        "cir.generic_apply",
        args,
        result_types,
        vec![
            NamedAttribute::new("callee", Attribute::SymbolRef(callee.to_string())),
            NamedAttribute::new("sub_keys", sub_keys_attr),
            NamedAttribute::new("sub_types", sub_types_attr),
        ],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
    (0..result_types.len())
        .map(|i| ctx.op_result(op, i))
        .collect()
}
