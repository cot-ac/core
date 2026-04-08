//! # cot-vwt
//!
//! The value witness table (VWT) construct for CIR. Provides seven operations
//! for querying type metadata (size, stride, alignment) and performing
//! type-abstract lifetime operations (copy, destroy, move, init_buffer).
//! No custom types — operates through the VWT infrastructure. Includes a
//! WitnessTableGenerator transform pass.

pub mod ops;
pub mod lowering;
pub mod transform;

/// Register the VWT construct's operations.
pub fn register(_ctx: &mut mlif::Context) {
    todo!()
}
