//! # Memory Types
//!
//! Defines two CIR types for memory operations:
//!
//! - `!cir.ptr` — opaque pointer (like LLVM's `ptr`), no pointee type tracked
//! - `!cir.ref<T>` — typed reference, carries the pointee type `T` for
//!   type-safe load/store without explicit casts

use mlif::Context;

/// Opaque pointer type: `!cir.ptr`.
pub struct PtrType;

/// Typed reference type: `!cir.ref<T>`.
pub struct RefType {
    // TODO: pointee type field
}

/// Register `!cir.ptr` and `!cir.ref<T>` with the type system.
pub fn register_types(_ctx: &mut Context) {
    todo!()
}
