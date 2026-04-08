//! # cot-arrays
//!
//! The arrays construct for CIR. Defines the fixed-size array type and three
//! operations for initialization, element value extraction, and element pointer
//! access.

pub mod types;
pub mod ops;
pub mod lowering;

/// Register the arrays construct's types and operations.
pub fn register(_ctx: &mut mlif::Context) {
    todo!()
}
