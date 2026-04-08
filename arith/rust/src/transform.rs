//! # Arith Transforms
//!
//! Contains the SemanticAnalysis pass, which inserts implicit casts at function
//! call boundaries. When argument types don't exactly match parameter types,
//! this pass inserts the appropriate `extsi`, `trunci`, `sitofp`, `fptosi`,
//! `extf`, or `truncf` op to make the IR well-typed.

use mlif::Context;

/// Run the SemanticAnalysis pass: insert implicit casts at call sites.
pub fn semantic_analysis(_ctx: &mut Context) {
    todo!()
}
