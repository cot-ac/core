//! # Array Operations
//!
//! Defines 3 CIR operations:
//!
//! - `array_init` — construct an array value from element values
//! - `elem_val` — extract an element value by index from an array value (SSA)
//! - `elem_ptr` — compute a pointer to an element within an array pointer

use mlif::Context;

/// Register all 3 array ops with the given context.
pub fn register_ops(_ctx: &mut Context) {
    todo!()
}
