//! # Struct Operations
//!
//! Defines 3 CIR operations:
//!
//! - `struct_init` — construct a struct value from field values
//! - `field_val` — extract a field value by name from a struct value (SSA)
//! - `field_ptr` — compute a pointer to a field within a struct pointer

use mlif::Context;

/// Register all 3 struct ops with the given context.
pub fn register_ops(_ctx: &mut Context) {
    todo!()
}
