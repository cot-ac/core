//! # Flow Operations
//!
//! Defines 4 CIR terminator operations for control flow. All four are
//! Terminators: they must appear exactly once at the end of a basic block.
//!
//! - `cir.br` -- unconditional branch to a target block
//! - `cir.condbr` -- conditional branch on an i1 value to one of two blocks
//! - `cir.switch` -- multi-way branch on an integer value with a default target
//! - `cir.trap` -- unreachable / abort (no successors)
//!
//! Successor blocks are modeled as integer attributes (BlockId indices) since
//! MLIF does not have first-class successor support.

use mlif::{
    Attribute, BlockId, Context, Dialect, EntityRef, Location, NamedAttribute, OpDefinition,
    OpTrait, ValueId,
};

// ---------------------------------------------------------------------------
// Op registration
// ---------------------------------------------------------------------------

/// Register all 4 flow operations with the context's CIR dialect.
///
/// All four ops have the `Terminator` trait. `br`, `condbr`, and `switch`
/// are also `Pure` (per the C++ tablegen definitions). `trap` is not Pure.
pub fn register_ops(ctx: &mut Context) {
    let mut dialect = Dialect::new("cir");

    // 1. br -- Terminator + Pure
    dialect.register_op(
        OpDefinition::new("cir.br")
            .with_trait(OpTrait::Terminator)
            .with_trait(OpTrait::Pure),
    );

    // 2. condbr -- Terminator + Pure
    dialect.register_op(
        OpDefinition::new("cir.condbr")
            .with_trait(OpTrait::Terminator)
            .with_trait(OpTrait::Pure),
    );

    // 3. switch -- Terminator + Pure
    dialect.register_op(
        OpDefinition::new("cir.switch")
            .with_trait(OpTrait::Terminator)
            .with_trait(OpTrait::Pure),
    );

    // 4. trap -- Terminator only (not Pure, it aborts)
    dialect.register_op(OpDefinition::new("cir.trap").with_trait(OpTrait::Terminator));

    ctx.register_dialect(dialect);
}

// ---------------------------------------------------------------------------
// Helper: encode a BlockId as an integer attribute
// ---------------------------------------------------------------------------

/// Encode a `BlockId` as an `Attribute::Integer` using the index type.
fn block_attr(ctx: &mut Context, block: BlockId) -> Attribute {
    let index_ty = ctx.index_type();
    Attribute::Integer {
        value: block.index() as i64,
        ty: index_ty,
    }
}

// ---------------------------------------------------------------------------
// Builder functions
// ---------------------------------------------------------------------------

/// Build `cir.br ^dest`.
///
/// Unconditional branch to `dest`. No operands, no results.
pub fn build_br(ctx: &mut Context, block: BlockId, dest: BlockId, loc: Location) {
    let dest_attr = block_attr(ctx, dest);
    let op = ctx.create_operation(
        "cir.br",
        &[],
        &[],
        vec![NamedAttribute::new("dest", dest_attr)],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
}

/// Build `cir.condbr %condition, ^true_dest, ^false_dest`.
///
/// Conditional branch on an i1 `condition`. Branches to `true_dest` if
/// true, `false_dest` otherwise. No results.
pub fn build_condbr(
    ctx: &mut Context,
    block: BlockId,
    condition: ValueId,
    true_dest: BlockId,
    false_dest: BlockId,
    loc: Location,
) {
    let true_attr = block_attr(ctx, true_dest);
    let false_attr = block_attr(ctx, false_dest);
    let op = ctx.create_operation(
        "cir.condbr",
        &[condition],
        &[],
        vec![
            NamedAttribute::new("true_dest", true_attr),
            NamedAttribute::new("false_dest", false_attr),
        ],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
}

/// Build `cir.switch %value : <ty>, ^default_dest [case_val: ^case_dest, ...]`.
///
/// Multi-way branch on an integer `value`. If `value` matches a case, branch
/// to the corresponding destination; otherwise branch to `default_dest`.
///
/// `cases` is a slice of `(case_value, case_dest_block)` pairs.
pub fn build_switch(
    ctx: &mut Context,
    block: BlockId,
    value: ValueId,
    default_dest: BlockId,
    cases: &[(i64, BlockId)],
    loc: Location,
) {
    let default_attr = block_attr(ctx, default_dest);

    // Build case_values as an Array of integer attributes.
    let index_ty = ctx.index_type();
    let case_values: Vec<Attribute> = cases
        .iter()
        .map(|&(val, _)| Attribute::Integer {
            value: val,
            ty: index_ty,
        })
        .collect();

    // Build case_dests as an Array of integer (BlockId) attributes.
    let case_dests: Vec<Attribute> = cases
        .iter()
        .map(|&(_, dest)| Attribute::Integer {
            value: dest.index() as i64,
            ty: index_ty,
        })
        .collect();

    let op = ctx.create_operation(
        "cir.switch",
        &[value],
        &[],
        vec![
            NamedAttribute::new("default_dest", default_attr),
            NamedAttribute::new("case_values", Attribute::Array(case_values)),
            NamedAttribute::new("case_dests", Attribute::Array(case_dests)),
        ],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
}

/// Build `cir.trap`.
///
/// Emits a trap instruction followed by unreachable. Used for unreachable
/// code paths (e.g., after exhaustive switch). No operands, no results.
pub fn build_trap(ctx: &mut Context, block: BlockId, loc: Location) {
    let op = ctx.create_operation("cir.trap", &[], &[], vec![], vec![], loc);
    ctx.block_push_op(block, op);
}
