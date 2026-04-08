//! # cot-enums
//!
//! The enums construct for CIR. Defines the named enum type (backed by an
//! integer tag type) and two operations for creating enum constants and
//! extracting the underlying integer value.

pub mod types;
pub mod ops;
pub mod lowering;

/// Register the enums construct's types and operations.
pub fn register(_ctx: &mut mlif::Context) {
    todo!()
}
