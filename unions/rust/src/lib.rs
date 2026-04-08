//! # cot-unions
//!
//! The unions construct for CIR. Defines the tagged union type and three
//! operations for variant initialization, tag extraction, and payload access.

pub mod types;
pub mod ops;
pub mod lowering;

/// Register the unions construct's types and operations.
pub fn register(_ctx: &mut mlif::Context) {
    todo!()
}
