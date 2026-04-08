//! # cot-generics
//!
//! The generics construct for CIR. Defines the type parameter placeholder type,
//! one operation for generic application, and the GenericSpecializer transform
//! pass that monomorphizes generic code. Generics are fully eliminated before
//! lowering — there is no lowering.rs.

pub mod types;
pub mod ops;
pub mod transform;

/// Register the generics construct's types and operations.
pub fn register(_ctx: &mut mlif::Context) {
    todo!()
}
