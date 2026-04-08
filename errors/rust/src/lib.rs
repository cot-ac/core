//! # cot-errors
//!
//! The errors construct for CIR. Defines the error union type and five
//! operations for wrapping results, wrapping errors, testing error status,
//! and extracting payloads or error codes.

pub mod types;
pub mod ops;
pub mod lowering;

/// Register the errors construct's types and operations.
pub fn register(_ctx: &mut mlif::Context) {
    todo!()
}
