//! # Memory Operations
//!
//! Defines 5 CIR operations:
//!
//! - `alloca` — allocate a stack slot for type T, returns `!cir.ptr`
//! - `store` — store a value to a pointer
//! - `load` — load a value from a pointer, requires result type annotation
//! - `addr_of` — take the address of a named global or local, returns `!cir.ptr`
//! - `deref` — dereference a `!cir.ref<T>`, returns value of type T

use mlif::Context;

/// Register all 5 memory ops with the given context.
pub fn register_ops(_ctx: &mut Context) {
    todo!()
}
