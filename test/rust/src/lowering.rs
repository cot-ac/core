//! # Test Lowering
//!
//! Lowers CIR test ops to Cranelift IR:
//! - `assert` -> `brif` on condition: if false, branch to trap block; if true, continue
//! - `test_case` -> converted to a regular function callable by the generated test runner

use mlif::Context;

/// Lower all test ops in a module to Cranelift IR.
pub fn lower_test(_ctx: &mut Context) {
    todo!()
}
