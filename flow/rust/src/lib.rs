//! # cot-flow
//!
//! The flow construct for CIR. Provides four terminator operations for
//! control flow: unconditional branch, conditional branch, multi-way switch,
//! and trap (unreachable/abort). No custom types — all operate on MLIF
//! primitives and block references.

pub mod ops;
pub mod lowering;

/// Register the flow construct's operations.
pub fn register(_ctx: &mut mlif::Context) {
    todo!()
}
