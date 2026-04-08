//! Trait operations: witness_table, trait_call, witness_method,
//! init_existential, open_existential, deinit_existential.
//!
//! Reference: `core/traits/include/traits/Ops.td`

use mlif::{
    Attribute, BlockId, Context, Dialect, Location, NamedAttribute, OpDefinition, OpTrait, TypeId,
    ValueId,
};

/// Register all 6 trait ops with the CIR dialect.
pub fn register_ops(ctx: &mut Context) {
    let mut dialect = Dialect::new("cir");

    // witness_table is a symbol declaration (no Pure — has side effects as a declaration).
    dialect.register_op(OpDefinition::new("cir.witness_table"));
    // trait_call uses a symbol reference, not Pure.
    dialect.register_op(OpDefinition::new("cir.trait_call"));
    // witness_method loads from a witness table pointer — Pure.
    dialect.register_op(OpDefinition::new("cir.witness_method").with_trait(OpTrait::Pure));
    // init_existential writes to a container — not Pure.
    dialect.register_op(OpDefinition::new("cir.init_existential"));
    // open_existential reads from a container — Pure.
    dialect.register_op(OpDefinition::new("cir.open_existential").with_trait(OpTrait::Pure));
    // deinit_existential writes to a container — not Pure.
    dialect.register_op(OpDefinition::new("cir.deinit_existential"));

    ctx.register_dialect(dialect);
}

/// Build `cir.witness_table @name` — declare a witness table.
pub fn build_witness_table(
    ctx: &mut Context,
    block: BlockId,
    sym_name: &str,
    protocol: &str,
    conforming_type: &str,
    method_names: &[&str],
    method_impls: &[&str],
    loc: Location,
) {
    let names_attr = Attribute::Array(
        method_names
            .iter()
            .map(|n| Attribute::String(n.to_string()))
            .collect(),
    );
    let impls_attr = Attribute::Array(
        method_impls
            .iter()
            .map(|n| Attribute::SymbolRef(n.to_string()))
            .collect(),
    );
    let op = ctx.create_operation(
        "cir.witness_table",
        &[],
        &[],
        vec![
            NamedAttribute::new("sym_name", Attribute::String(sym_name.to_string())),
            NamedAttribute::new("protocol", Attribute::String(protocol.to_string())),
            NamedAttribute::new(
                "conforming_type",
                Attribute::String(conforming_type.to_string()),
            ),
            NamedAttribute::new("method_names", names_attr),
            NamedAttribute::new("method_impls", impls_attr),
        ],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
}

/// Build `cir.trait_call @Protocol::method(%args)`.
pub fn build_trait_call(
    ctx: &mut Context,
    block: BlockId,
    protocol: &str,
    method: &str,
    args: &[ValueId],
    result_types: &[TypeId],
    loc: Location,
) -> Vec<ValueId> {
    let op = ctx.create_operation(
        "cir.trait_call",
        args,
        result_types,
        vec![
            NamedAttribute::new("protocol", Attribute::SymbolRef(protocol.to_string())),
            NamedAttribute::new("method", Attribute::String(method.to_string())),
        ],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
    (0..result_types.len())
        .map(|i| ctx.op_result(op, i))
        .collect()
}

/// Build `cir.witness_method %pwt, "method"` — load method pointer from PWT.
pub fn build_witness_method(
    ctx: &mut Context,
    block: BlockId,
    pwt: ValueId,
    method: &str,
    method_index: Option<i64>,
    result_ty: TypeId,
    loc: Location,
) -> ValueId {
    let mut attrs = vec![NamedAttribute::new(
        "method",
        Attribute::String(method.to_string()),
    )];
    if let Some(idx) = method_index {
        let i64_ty = ctx.integer_type(64);
        attrs.push(NamedAttribute::new(
            "method_index",
            Attribute::Integer {
                value: idx,
                ty: i64_ty,
            },
        ));
    }
    let op = ctx.create_operation(
        "cir.witness_method",
        &[pwt],
        &[result_ty],
        attrs,
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

/// Build `cir.init_existential %container, %value, %vwt, %pwt`.
pub fn build_init_existential(
    ctx: &mut Context,
    block: BlockId,
    container: ValueId,
    value: ValueId,
    vwt: ValueId,
    pwt: ValueId,
    loc: Location,
) {
    let op = ctx.create_operation(
        "cir.init_existential",
        &[container, value, vwt, pwt],
        &[],
        vec![],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
}

/// Build `cir.open_existential %container` — extract buffer, VWT, PWT.
pub fn build_open_existential(
    ctx: &mut Context,
    block: BlockId,
    container: ValueId,
    buf_ty: TypeId,
    vwt_ty: TypeId,
    pwt_ty: TypeId,
    loc: Location,
) -> (ValueId, ValueId, ValueId) {
    let op = ctx.create_operation(
        "cir.open_existential",
        &[container],
        &[buf_ty, vwt_ty, pwt_ty],
        vec![],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
    (
        ctx.op_result(op, 0),
        ctx.op_result(op, 1),
        ctx.op_result(op, 2),
    )
}

/// Build `cir.deinit_existential %container`.
pub fn build_deinit_existential(
    ctx: &mut Context,
    block: BlockId,
    container: ValueId,
    loc: Location,
) {
    let op = ctx.create_operation(
        "cir.deinit_existential",
        &[container],
        &[],
        vec![],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
}
