//! # Test Operations
//!
//! Defines 2 CIR operations:
//!
//! - `assert` — assert that an i1 condition is true; traps with a message if false
//! - `test_case` — declare a named test case (a region of IR to be run by the test runner)

use mlif::Context;

/// Register all 2 test ops with the given context.
pub fn register_ops(_ctx: &mut Context) {
    todo!()
}
