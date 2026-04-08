//! # Arithmetic Operations
//!
//! Defines 29 CIR operations for arithmetic, constants, comparisons, casts,
//! and bitwise logic. Provides op registration and builder functions that
//! construct well-typed IR nodes in the MLIF context.
//!
//! ## Constants
//! - `cir.constant` — integer, float, or boolean literal
//!
//! ## Binary arithmetic
//! - `cir.add`, `cir.sub`, `cir.mul` — polymorphic (int + float)
//! - `cir.divsi`, `cir.divui`, `cir.divf` — division (signed/unsigned/float)
//! - `cir.remsi`, `cir.remui`, `cir.remf` — remainder
//!
//! ## Unary
//! - `cir.neg` — integer negation
//! - `cir.negf` — float negation
//!
//! ## Bitwise
//! - `cir.bit_and`, `cir.bit_or`, `cir.bit_xor`, `cir.bit_not`
//! - `cir.shl`, `cir.shr`, `cir.shr_s`
//!
//! ## Comparison
//! - `cir.cmp` — integer comparison (10 predicates)
//! - `cir.cmpf` — float comparison (14 predicates, NaN-aware)
//! - `cir.select` — ternary select on i1 condition
//!
//! ## Integer casts
//! - `cir.extsi`, `cir.extui`, `cir.trunci`
//!
//! ## Float casts
//! - `cir.sitofp`, `cir.fptosi`, `cir.extf`, `cir.truncf`

use mlif::{
    Attribute, BlockId, Context, Dialect, Location, NamedAttribute, OpDefinition, OpTrait, TypeId,
    ValueId,
};

// ---------------------------------------------------------------------------
// Predicate enums
// ---------------------------------------------------------------------------

/// Integer comparison predicates. Values match the CIR tablegen enum.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i64)]
pub enum IntPredicate {
    Eq = 0,
    Ne = 1,
    Slt = 2,
    Sle = 3,
    Sgt = 4,
    Sge = 5,
    Ult = 6,
    Ule = 7,
    Ugt = 8,
    Uge = 9,
}

/// Float comparison predicates. Ordered (O) and unordered (U) NaN variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i64)]
pub enum FloatPredicate {
    Oeq = 0,
    Ogt = 1,
    Oge = 2,
    Olt = 3,
    Ole = 4,
    One = 5,
    Ord = 6,
    Ueq = 7,
    Ugt = 8,
    Uge = 9,
    Ult = 10,
    Ule = 11,
    Une = 12,
    Uno = 13,
}

// ---------------------------------------------------------------------------
// Op registration
// ---------------------------------------------------------------------------

/// Register all 29 arith operations with the context's CIR dialect.
pub fn register_ops(ctx: &mut Context) {
    let mut dialect = Dialect::new("cir");

    // 1. constant — Pure (ConstantLike is a C++ MLIR trait, not in our Rust OpTrait)
    dialect.register_op(OpDefinition::new("cir.constant").with_trait(OpTrait::Pure));

    // 2. add — Pure, Commutative, SameOperandsAndResultType
    dialect.register_op(
        OpDefinition::new("cir.add")
            .with_trait(OpTrait::Pure)
            .with_trait(OpTrait::Commutative)
            .with_trait(OpTrait::SameOperandsAndResultType),
    );

    // 3. sub — Pure, SameOperandsAndResultType
    dialect.register_op(
        OpDefinition::new("cir.sub")
            .with_trait(OpTrait::Pure)
            .with_trait(OpTrait::SameOperandsAndResultType),
    );

    // 4. mul — Pure, Commutative, SameOperandsAndResultType
    dialect.register_op(
        OpDefinition::new("cir.mul")
            .with_trait(OpTrait::Pure)
            .with_trait(OpTrait::Commutative)
            .with_trait(OpTrait::SameOperandsAndResultType),
    );

    // 5. divsi — Pure, SameOperandsAndResultType
    dialect.register_op(
        OpDefinition::new("cir.divsi")
            .with_trait(OpTrait::Pure)
            .with_trait(OpTrait::SameOperandsAndResultType),
    );

    // 6. divui — Pure, SameOperandsAndResultType
    dialect.register_op(
        OpDefinition::new("cir.divui")
            .with_trait(OpTrait::Pure)
            .with_trait(OpTrait::SameOperandsAndResultType),
    );

    // 7. divf — Pure, SameOperandsAndResultType
    dialect.register_op(
        OpDefinition::new("cir.divf")
            .with_trait(OpTrait::Pure)
            .with_trait(OpTrait::SameOperandsAndResultType),
    );

    // 8. remsi — Pure, SameOperandsAndResultType
    dialect.register_op(
        OpDefinition::new("cir.remsi")
            .with_trait(OpTrait::Pure)
            .with_trait(OpTrait::SameOperandsAndResultType),
    );

    // 9. remui — Pure, SameOperandsAndResultType
    dialect.register_op(
        OpDefinition::new("cir.remui")
            .with_trait(OpTrait::Pure)
            .with_trait(OpTrait::SameOperandsAndResultType),
    );

    // 10. remf — Pure, SameOperandsAndResultType
    dialect.register_op(
        OpDefinition::new("cir.remf")
            .with_trait(OpTrait::Pure)
            .with_trait(OpTrait::SameOperandsAndResultType),
    );

    // 11. neg — Pure (integer negation, unary — no SameOperandsAndResultType
    //     because tablegen uses CIR_IntUnaryOp which has Pure + SameOperandsAndResultType,
    //     but the spec says just Pure. Follow the tablegen: CIR_IntUnaryOp has
    //     [Pure, SameOperandsAndResultType])
    dialect.register_op(
        OpDefinition::new("cir.neg")
            .with_trait(OpTrait::Pure)
            .with_trait(OpTrait::SameOperandsAndResultType),
    );

    // 12. negf — Pure, SameOperandsAndResultType
    dialect.register_op(
        OpDefinition::new("cir.negf")
            .with_trait(OpTrait::Pure)
            .with_trait(OpTrait::SameOperandsAndResultType),
    );

    // 13. bit_and — Pure, Commutative, SameOperandsAndResultType
    dialect.register_op(
        OpDefinition::new("cir.bit_and")
            .with_trait(OpTrait::Pure)
            .with_trait(OpTrait::Commutative)
            .with_trait(OpTrait::SameOperandsAndResultType),
    );

    // 14. bit_or — Pure, Commutative, SameOperandsAndResultType
    dialect.register_op(
        OpDefinition::new("cir.bit_or")
            .with_trait(OpTrait::Pure)
            .with_trait(OpTrait::Commutative)
            .with_trait(OpTrait::SameOperandsAndResultType),
    );

    // 15. bit_xor — Pure, Commutative, SameOperandsAndResultType
    dialect.register_op(
        OpDefinition::new("cir.bit_xor")
            .with_trait(OpTrait::Pure)
            .with_trait(OpTrait::Commutative)
            .with_trait(OpTrait::SameOperandsAndResultType),
    );

    // 16. bit_not — Pure, SameOperandsAndResultType
    dialect.register_op(
        OpDefinition::new("cir.bit_not")
            .with_trait(OpTrait::Pure)
            .with_trait(OpTrait::SameOperandsAndResultType),
    );

    // 17. shl — Pure, SameOperandsAndResultType
    dialect.register_op(
        OpDefinition::new("cir.shl")
            .with_trait(OpTrait::Pure)
            .with_trait(OpTrait::SameOperandsAndResultType),
    );

    // 18. shr — Pure, SameOperandsAndResultType (logical / unsigned)
    dialect.register_op(
        OpDefinition::new("cir.shr")
            .with_trait(OpTrait::Pure)
            .with_trait(OpTrait::SameOperandsAndResultType),
    );

    // 19. shr_s — Pure, SameOperandsAndResultType (arithmetic / signed)
    dialect.register_op(
        OpDefinition::new("cir.shr_s")
            .with_trait(OpTrait::Pure)
            .with_trait(OpTrait::SameOperandsAndResultType),
    );

    // 20. cmp — Pure (result is i1, not same as operands)
    dialect.register_op(OpDefinition::new("cir.cmp").with_trait(OpTrait::Pure));

    // 21. cmpf — Pure (result is i1, not same as operands)
    dialect.register_op(OpDefinition::new("cir.cmpf").with_trait(OpTrait::Pure));

    // 22. select — Pure
    dialect.register_op(OpDefinition::new("cir.select").with_trait(OpTrait::Pure));

    // 23. extsi — Pure
    dialect.register_op(OpDefinition::new("cir.extsi").with_trait(OpTrait::Pure));

    // 24. extui — Pure
    dialect.register_op(OpDefinition::new("cir.extui").with_trait(OpTrait::Pure));

    // 25. trunci — Pure
    dialect.register_op(OpDefinition::new("cir.trunci").with_trait(OpTrait::Pure));

    // 26. sitofp — Pure
    dialect.register_op(OpDefinition::new("cir.sitofp").with_trait(OpTrait::Pure));

    // 27. fptosi — Pure
    dialect.register_op(OpDefinition::new("cir.fptosi").with_trait(OpTrait::Pure));

    // 28. extf — Pure
    dialect.register_op(OpDefinition::new("cir.extf").with_trait(OpTrait::Pure));

    // 29. truncf — Pure
    dialect.register_op(OpDefinition::new("cir.truncf").with_trait(OpTrait::Pure));

    ctx.register_dialect(dialect);
}

// ---------------------------------------------------------------------------
// Builder functions — Constants
// ---------------------------------------------------------------------------

/// Build `cir.constant <value> : <ty>` for an integer value.
pub fn build_constant_int(
    ctx: &mut Context,
    block: BlockId,
    ty: TypeId,
    value: i64,
    loc: Location,
) -> ValueId {
    let op = ctx.create_operation(
        "cir.constant",
        &[],
        &[ty],
        vec![NamedAttribute::new(
            "value",
            Attribute::Integer { value, ty },
        )],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

/// Build `cir.constant <value> : <ty>` for a floating-point value.
pub fn build_constant_float(
    ctx: &mut Context,
    block: BlockId,
    ty: TypeId,
    value: f64,
    loc: Location,
) -> ValueId {
    let op = ctx.create_operation(
        "cir.constant",
        &[],
        &[ty],
        vec![NamedAttribute::new(
            "value",
            Attribute::Float { value, ty },
        )],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

/// Build a boolean constant (`cir.constant 0/1 : i1`).
pub fn build_constant_bool(
    ctx: &mut Context,
    block: BlockId,
    value: bool,
    loc: Location,
) -> ValueId {
    let i1_ty = ctx.integer_type(1);
    build_constant_int(ctx, block, i1_ty, if value { 1 } else { 0 }, loc)
}

// ---------------------------------------------------------------------------
// Builder functions — Binary arithmetic
// ---------------------------------------------------------------------------

/// Helper: build a binary op that returns the same type as its LHS operand.
fn build_binary_same_type(
    ctx: &mut Context,
    block: BlockId,
    op_name: &str,
    lhs: ValueId,
    rhs: ValueId,
    loc: Location,
) -> ValueId {
    let ty = ctx.value_type(lhs);
    let op = ctx.create_operation(op_name, &[lhs, rhs], &[ty], vec![], vec![], loc);
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

/// Build `cir.add %lhs, %rhs : ty`.
pub fn build_add(
    ctx: &mut Context,
    block: BlockId,
    lhs: ValueId,
    rhs: ValueId,
    loc: Location,
) -> ValueId {
    build_binary_same_type(ctx, block, "cir.add", lhs, rhs, loc)
}

/// Build `cir.sub %lhs, %rhs : ty`.
pub fn build_sub(
    ctx: &mut Context,
    block: BlockId,
    lhs: ValueId,
    rhs: ValueId,
    loc: Location,
) -> ValueId {
    build_binary_same_type(ctx, block, "cir.sub", lhs, rhs, loc)
}

/// Build `cir.mul %lhs, %rhs : ty`.
pub fn build_mul(
    ctx: &mut Context,
    block: BlockId,
    lhs: ValueId,
    rhs: ValueId,
    loc: Location,
) -> ValueId {
    build_binary_same_type(ctx, block, "cir.mul", lhs, rhs, loc)
}

/// Build `cir.divsi %lhs, %rhs : ty` (signed integer division).
pub fn build_divsi(
    ctx: &mut Context,
    block: BlockId,
    lhs: ValueId,
    rhs: ValueId,
    loc: Location,
) -> ValueId {
    build_binary_same_type(ctx, block, "cir.divsi", lhs, rhs, loc)
}

/// Build `cir.divui %lhs, %rhs : ty` (unsigned integer division).
pub fn build_divui(
    ctx: &mut Context,
    block: BlockId,
    lhs: ValueId,
    rhs: ValueId,
    loc: Location,
) -> ValueId {
    build_binary_same_type(ctx, block, "cir.divui", lhs, rhs, loc)
}

/// Build `cir.divf %lhs, %rhs : ty` (float division).
pub fn build_divf(
    ctx: &mut Context,
    block: BlockId,
    lhs: ValueId,
    rhs: ValueId,
    loc: Location,
) -> ValueId {
    build_binary_same_type(ctx, block, "cir.divf", lhs, rhs, loc)
}

/// Build `cir.remsi %lhs, %rhs : ty` (signed integer remainder).
pub fn build_remsi(
    ctx: &mut Context,
    block: BlockId,
    lhs: ValueId,
    rhs: ValueId,
    loc: Location,
) -> ValueId {
    build_binary_same_type(ctx, block, "cir.remsi", lhs, rhs, loc)
}

/// Build `cir.remui %lhs, %rhs : ty` (unsigned integer remainder).
pub fn build_remui(
    ctx: &mut Context,
    block: BlockId,
    lhs: ValueId,
    rhs: ValueId,
    loc: Location,
) -> ValueId {
    build_binary_same_type(ctx, block, "cir.remui", lhs, rhs, loc)
}

/// Build `cir.remf %lhs, %rhs : ty` (float remainder).
pub fn build_remf(
    ctx: &mut Context,
    block: BlockId,
    lhs: ValueId,
    rhs: ValueId,
    loc: Location,
) -> ValueId {
    build_binary_same_type(ctx, block, "cir.remf", lhs, rhs, loc)
}

// ---------------------------------------------------------------------------
// Builder functions — Unary
// ---------------------------------------------------------------------------

/// Helper: build a unary op that returns the same type as its operand.
fn build_unary_same_type(
    ctx: &mut Context,
    block: BlockId,
    op_name: &str,
    operand: ValueId,
    loc: Location,
) -> ValueId {
    let ty = ctx.value_type(operand);
    let op = ctx.create_operation(op_name, &[operand], &[ty], vec![], vec![], loc);
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

/// Build `cir.neg %operand : ty` (integer negation).
pub fn build_neg(
    ctx: &mut Context,
    block: BlockId,
    operand: ValueId,
    loc: Location,
) -> ValueId {
    build_unary_same_type(ctx, block, "cir.neg", operand, loc)
}

/// Build `cir.negf %operand : ty` (float negation).
pub fn build_negf(
    ctx: &mut Context,
    block: BlockId,
    operand: ValueId,
    loc: Location,
) -> ValueId {
    build_unary_same_type(ctx, block, "cir.negf", operand, loc)
}

// ---------------------------------------------------------------------------
// Builder functions — Bitwise
// ---------------------------------------------------------------------------

/// Build `cir.bit_and %lhs, %rhs : ty`.
pub fn build_bit_and(
    ctx: &mut Context,
    block: BlockId,
    lhs: ValueId,
    rhs: ValueId,
    loc: Location,
) -> ValueId {
    build_binary_same_type(ctx, block, "cir.bit_and", lhs, rhs, loc)
}

/// Build `cir.bit_or %lhs, %rhs : ty`.
pub fn build_bit_or(
    ctx: &mut Context,
    block: BlockId,
    lhs: ValueId,
    rhs: ValueId,
    loc: Location,
) -> ValueId {
    build_binary_same_type(ctx, block, "cir.bit_or", lhs, rhs, loc)
}

/// Build `cir.bit_xor %lhs, %rhs : ty`.
pub fn build_bit_xor(
    ctx: &mut Context,
    block: BlockId,
    lhs: ValueId,
    rhs: ValueId,
    loc: Location,
) -> ValueId {
    build_binary_same_type(ctx, block, "cir.bit_xor", lhs, rhs, loc)
}

/// Build `cir.bit_not %operand : ty`.
pub fn build_bit_not(
    ctx: &mut Context,
    block: BlockId,
    operand: ValueId,
    loc: Location,
) -> ValueId {
    build_unary_same_type(ctx, block, "cir.bit_not", operand, loc)
}

// ---------------------------------------------------------------------------
// Builder functions — Shifts
// ---------------------------------------------------------------------------

/// Build `cir.shl %lhs, %rhs : ty` (shift left).
pub fn build_shl(
    ctx: &mut Context,
    block: BlockId,
    lhs: ValueId,
    rhs: ValueId,
    loc: Location,
) -> ValueId {
    build_binary_same_type(ctx, block, "cir.shl", lhs, rhs, loc)
}

/// Build `cir.shr %lhs, %rhs : ty` (logical / unsigned shift right).
pub fn build_shr(
    ctx: &mut Context,
    block: BlockId,
    lhs: ValueId,
    rhs: ValueId,
    loc: Location,
) -> ValueId {
    build_binary_same_type(ctx, block, "cir.shr", lhs, rhs, loc)
}

/// Build `cir.shr_s %lhs, %rhs : ty` (arithmetic / signed shift right).
pub fn build_shr_s(
    ctx: &mut Context,
    block: BlockId,
    lhs: ValueId,
    rhs: ValueId,
    loc: Location,
) -> ValueId {
    build_binary_same_type(ctx, block, "cir.shr_s", lhs, rhs, loc)
}

// ---------------------------------------------------------------------------
// Builder functions — Comparison
// ---------------------------------------------------------------------------

/// Build `cir.cmp <predicate>, %lhs, %rhs : ty` (integer comparison).
/// Returns an i1 result.
pub fn build_cmp(
    ctx: &mut Context,
    block: BlockId,
    predicate: IntPredicate,
    lhs: ValueId,
    rhs: ValueId,
    loc: Location,
) -> ValueId {
    let i1_ty = ctx.integer_type(1);
    let op = ctx.create_operation(
        "cir.cmp",
        &[lhs, rhs],
        &[i1_ty],
        vec![NamedAttribute::new(
            "predicate",
            Attribute::Integer {
                value: predicate as i64,
                ty: i1_ty,
            },
        )],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

/// Build `cir.cmpf <predicate>, %lhs, %rhs : ty` (float comparison).
/// Returns an i1 result.
pub fn build_cmpf(
    ctx: &mut Context,
    block: BlockId,
    predicate: FloatPredicate,
    lhs: ValueId,
    rhs: ValueId,
    loc: Location,
) -> ValueId {
    let i1_ty = ctx.integer_type(1);
    let op = ctx.create_operation(
        "cir.cmpf",
        &[lhs, rhs],
        &[i1_ty],
        vec![NamedAttribute::new(
            "predicate",
            Attribute::Integer {
                value: predicate as i64,
                ty: i1_ty,
            },
        )],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

/// Build `cir.select %cond, %true_val, %false_val : ty`.
/// Result type matches `true_val`/`false_val`.
pub fn build_select(
    ctx: &mut Context,
    block: BlockId,
    cond: ValueId,
    true_val: ValueId,
    false_val: ValueId,
    loc: Location,
) -> ValueId {
    let ty = ctx.value_type(true_val);
    let op = ctx.create_operation(
        "cir.select",
        &[cond, true_val, false_val],
        &[ty],
        vec![],
        vec![],
        loc,
    );
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

// ---------------------------------------------------------------------------
// Builder functions — Integer casts
// ---------------------------------------------------------------------------

/// Helper: build a unary cast op with explicit result type.
fn build_cast(
    ctx: &mut Context,
    block: BlockId,
    op_name: &str,
    input: ValueId,
    result_ty: TypeId,
    loc: Location,
) -> ValueId {
    let op = ctx.create_operation(op_name, &[input], &[result_ty], vec![], vec![], loc);
    ctx.block_push_op(block, op);
    ctx.op_result(op, 0)
}

/// Build `cir.extsi %input : narrow_int -> wide_int` (sign-extend).
pub fn build_extsi(
    ctx: &mut Context,
    block: BlockId,
    input: ValueId,
    result_ty: TypeId,
    loc: Location,
) -> ValueId {
    build_cast(ctx, block, "cir.extsi", input, result_ty, loc)
}

/// Build `cir.extui %input : narrow_int -> wide_int` (zero-extend).
pub fn build_extui(
    ctx: &mut Context,
    block: BlockId,
    input: ValueId,
    result_ty: TypeId,
    loc: Location,
) -> ValueId {
    build_cast(ctx, block, "cir.extui", input, result_ty, loc)
}

/// Build `cir.trunci %input : wide_int -> narrow_int` (truncate).
pub fn build_trunci(
    ctx: &mut Context,
    block: BlockId,
    input: ValueId,
    result_ty: TypeId,
    loc: Location,
) -> ValueId {
    build_cast(ctx, block, "cir.trunci", input, result_ty, loc)
}

// ---------------------------------------------------------------------------
// Builder functions — Float casts
// ---------------------------------------------------------------------------

/// Build `cir.sitofp %input : int -> float` (signed int to float).
pub fn build_sitofp(
    ctx: &mut Context,
    block: BlockId,
    input: ValueId,
    result_ty: TypeId,
    loc: Location,
) -> ValueId {
    build_cast(ctx, block, "cir.sitofp", input, result_ty, loc)
}

/// Build `cir.fptosi %input : float -> int` (float to signed int).
pub fn build_fptosi(
    ctx: &mut Context,
    block: BlockId,
    input: ValueId,
    result_ty: TypeId,
    loc: Location,
) -> ValueId {
    build_cast(ctx, block, "cir.fptosi", input, result_ty, loc)
}

/// Build `cir.extf %input : f32 -> f64` (widen float).
pub fn build_extf(
    ctx: &mut Context,
    block: BlockId,
    input: ValueId,
    result_ty: TypeId,
    loc: Location,
) -> ValueId {
    build_cast(ctx, block, "cir.extf", input, result_ty, loc)
}

/// Build `cir.truncf %input : f64 -> f32` (narrow float).
pub fn build_truncf(
    ctx: &mut Context,
    block: BlockId,
    input: ValueId,
    result_ty: TypeId,
    loc: Location,
) -> ValueId {
    build_cast(ctx, block, "cir.truncf", input, result_ty, loc)
}
