//! # cot-structs
//!
//! The structs construct for CIR. Defines the named struct type and three
//! operations for initialization, field value extraction, and field pointer
//! access.

pub mod types;
pub mod ops;
pub mod lowering;

/// Register the structs construct's types and operations.
pub fn register(_ctx: &mut mlif::Context) {
    todo!()
}
