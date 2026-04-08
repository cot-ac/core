//! # cot-test
//!
//! The test construct for CIR. Provides two operations for assertions and
//! test case declarations, plus a TestRunnerGenerator transform pass that
//! collects test cases and emits a main function to run them. No custom types.

pub mod ops;
pub mod lowering;
pub mod transform;

/// Register the test construct's operations.
pub fn register(_ctx: &mut mlif::Context) {
    todo!()
}
