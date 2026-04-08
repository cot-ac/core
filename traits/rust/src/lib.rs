//! # cot-traits
//!
//! The traits construct for CIR. Defines the existential type for protocol/
//! trait-typed values, six operations for witness table management and trait
//! dispatch, and a WitnessThunkGenerator transform pass.

pub mod types;
pub mod ops;
pub mod lowering;
pub mod transform;

/// Register the traits construct's types and operations.
pub fn register(_ctx: &mut mlif::Context) {
    todo!()
}
