//! # Flow Operations
//!
//! Defines 4 CIR terminator operations:
//!
//! - `br` — unconditional branch to a target block with arguments
//! - `condbr` — conditional branch on an i1 value to one of two blocks
//! - `switch` — multi-way branch on an integer value with a default target
//! - `trap` — unreachable / abort (no successors)
//!
//! All four ops are terminators: they must appear exactly once at the end of
//! a basic block.

use mlif::Context;

/// Register all 4 flow ops with the given context.
pub fn register_ops(_ctx: &mut Context) {
    todo!()
}
