//! # cot-optionals
//!
//! The optionals construct for CIR. Defines the optional type and four
//! operations for creating none values, wrapping values, null checking,
//! and payload extraction.

pub mod types;
pub mod ops;
pub mod lowering;

/// Register the optionals construct's types and operations.
pub fn register(_ctx: &mut mlif::Context) {
    todo!()
}
