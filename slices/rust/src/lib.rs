//! # cot-slices
//!
//! The slices construct for CIR. Defines the slice type (a fat pointer of
//! base pointer + length) and five operations for string constants, pointer/
//! length extraction, element access, and array-to-slice conversion.

pub mod types;
pub mod ops;
pub mod lowering;

/// Register the slices construct's types and operations.
pub fn register(_ctx: &mut mlif::Context) {
    todo!()
}
