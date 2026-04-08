//! # Memory Operations
//!
//! Defines 5 CIR operations for memory access. None are Pure (all have memory
//! effects), except `addr_of` which is Pure per the tablegen definition.
//! None are terminators.
//!
//! - `cir.alloca` -- allocate a stack slot for type T, returns `!cir.ptr`
//! - `cir.store` -- store a value to a pointer (no results)
//! - `cir.load` -- load a value from a pointer, requires result type
//! - `cir.addr_of` -- convert raw pointer to typed ref, Pure
//! - `cir.deref` -- dereference a `!cir.ref<T>`, returns value of type T

use crate::types;
use mlif::{
    Attribute, BlockId, Context, Dialect, Location, NamedAttribute, OpDefinition, OpTrait, TypeId,
    ValueId,
};

// ---------------------------------------------------------------------------
// Op registration
// ---------------------------------------------------------------------------

/// Register all 5 memory operations with the context's CIR dialect.
pub fn register_ops(ctx: &mut Context) {
    let mut dialect = Dialect::new("cir");

    // 1. alloca -- MemAlloc (not Pure, not Terminator)
    dialect.register_op(OpDefinition::new("cir.alloca"));

    // 2. store -- MemWrite (not Pure, not Terminator), no results
    dialect.register_op(OpDefinition::new("cir.store"));

    // 3. load -- MemRead (not Pure, not Terminator)
    dialect.register_op(OpDefinition::new("cir.load"));

    // 4. addr_of -- Pure (zero-cost wrapper, no memory effect)
    dialect.register_op(OpDefinition::new("cir.addr_of").with_trait(OpTrait::Pure));

    // 5. deref -- MemRead (not Pure, not Terminator)
    dialect.register_op(OpDefinition::new("cir.deref"));

    ctx.register_dialect(dialect);
}

// ---------------------------------------------------------------------------
// Builder functions
// ---------------------------------------------------------------------------

/// Build `cir.alloca <elem_type> : !cir.ptr`.
///
/// Allocates a stack slot for one value of `elem_type`. Returns an opaque
/// pointer (`!cir.ptr`) to the slot.
pub fn build_alloca(
    ctx: &mut Context,
    block: BlockId,
    elem_type: TypeId,
    loc: Location,
) -> ValueId {
    let ptr_ty = types::ptr_type(ctx);
    let op = ctx.create_operation(
        "cir.alloca",
        &[],
        &[ptr_ty],
        vec![NamedAttribute::new("elem_type", Attribute::Type(elem_type))],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

/// Build `cir.store %value, %addr`.
///
/// Writes `value` to the address `addr`. No results.
pub fn build_store(
    ctx: &mut Context,
    block: BlockId,
    value: ValueId,
    addr: ValueId,
    loc: Location,
) {
    let op = ctx.create_operation("cir.store", &[value, addr], &[], vec![], vec![], loc);
    ctx.block_push_op(block, op);
}

/// Build `cir.load %addr : !cir.ptr to <result_type>`.
///
/// Reads a value of `result_type` from address `addr`.
pub fn build_load(
    ctx: &mut Context,
    block: BlockId,
    addr: ValueId,
    result_type: TypeId,
    loc: Location,
) -> ValueId {
    let op = ctx.create_operation("cir.load", &[addr], &[result_type], vec![], vec![], loc);
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

/// Build `cir.addr_of %addr : !cir.ptr to !cir.ref<T>`.
///
/// Wraps a raw pointer in a typed reference. Zero-cost: both lower to
/// `!llvm.ptr`. The reference carries the pointee type for verification.
pub fn build_addr_of(
    ctx: &mut Context,
    block: BlockId,
    addr: ValueId,
    pointee_type: TypeId,
    loc: Location,
) -> ValueId {
    let ref_ty = types::ref_type(ctx, pointee_type);
    let op = ctx.create_operation("cir.addr_of", &[addr], &[ref_ty], vec![], vec![], loc);
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

/// Build `cir.deref %ref : !cir.ref<T> to <result_type>`.
///
/// Dereferences a typed reference to get the value. Performs a memory read.
pub fn build_deref(
    ctx: &mut Context,
    block: BlockId,
    reference: ValueId,
    result_type: TypeId,
    loc: Location,
) -> ValueId {
    let op = ctx.create_operation(
        "cir.deref",
        &[reference],
        &[result_type],
        vec![],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}
