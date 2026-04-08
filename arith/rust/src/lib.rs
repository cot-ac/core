//! # cot-arith
//!
//! The arithmetic construct for CIR. Provides 29 operations covering integer
//! and floating-point arithmetic, constants, comparisons, bitwise operations,
//! and sign/width casts. Operates entirely on MLIF primitive types (i1, i8,
//! i16, i32, i64, f32, f64) — no custom types are defined.

pub mod ops;
pub mod lowering;
pub mod transform;

/// Register the arith construct's operations with the MLIF dialect system.
pub fn register(_ctx: &mut mlif::Context) {
    todo!()
}
