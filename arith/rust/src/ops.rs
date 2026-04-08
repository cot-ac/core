//! # Arithmetic Operations
//!
//! Defines 29 CIR operations for arithmetic, constants, comparisons, casts,
//! and bitwise logic. Each op is a struct implementing `mlif::Op` with its
//! operands, result type, and verification constraints.
//!
//! ## Constants
//! - `constant_int` — integer literal
//! - `constant_float` — floating-point literal
//! - `constant_bool` — boolean literal (i1)
//!
//! ## Binary arithmetic
//! - `add`, `sub`, `mul`, `div`, `rem` — integer and float variants
//! - `neg` — unary negation
//!
//! ## Comparison
//! - `cmp` — integer comparison (eq, ne, slt, sle, sgt, sge, ult, ule, ugt, uge)
//! - `cmpf` — float comparison (oeq, one, olt, ole, ogt, oge, uno, ord)
//! - `select` — ternary select on i1 condition
//!
//! ## Bitwise
//! - `bit_and`, `bit_or`, `bit_xor`, `bit_not`, `shl`, `shr`
//!
//! ## Integer casts
//! - `extsi` — sign-extend
//! - `extui` — zero-extend
//! - `trunci` — truncate
//!
//! ## Float casts
//! - `sitofp` — signed int to float
//! - `fptosi` — float to signed int
//! - `extf` — widen float (f32 -> f64)
//! - `truncf` — narrow float (f64 -> f32)

use mlif::Context;

/// Integer comparison predicates.
pub enum IntPredicate {
    Eq, Ne, Slt, Sle, Sgt, Sge, Ult, Ule, Ugt, Uge,
}

/// Float comparison predicates.
pub enum FloatPredicate {
    Oeq, One, Olt, Ole, Ogt, Oge, Uno, Ord,
}

/// Register all 29 arith ops with the given context.
pub fn register_ops(_ctx: &mut Context) {
    todo!()
}
