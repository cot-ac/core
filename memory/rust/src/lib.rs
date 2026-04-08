//! # cot-memory
//!
//! The memory construct for CIR. Defines two types — opaque pointers and typed
//! references — and five operations for stack allocation, store, load,
//! address-of, and dereference.

pub mod types;
pub mod ops;
pub mod lowering;

/// Register the memory construct's types and operations.
pub fn register(_ctx: &mut mlif::Context) {
    todo!()
}
