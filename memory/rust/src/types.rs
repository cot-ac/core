//! # Memory Types
//!
//! Defines two CIR types for memory operations:
//!
//! - `!cir.ptr` -- opaque pointer (like LLVM's `ptr`), no pointee type tracked
//! - `!cir.ref<T>` -- typed reference, carries the pointee type `T` for
//!   type-safe load/store without explicit casts

use mlif::{Context, ExtensionType, TypeId};

/// Create or intern the opaque pointer type `!cir.ptr`.
pub fn ptr_type(ctx: &mut Context) -> TypeId {
    ctx.extension_type(ExtensionType::new("cir", "ptr"))
}

/// Create or intern the typed reference type `!cir.ref<T>`.
pub fn ref_type(ctx: &mut Context, pointee: TypeId) -> TypeId {
    ctx.extension_type(ExtensionType::new("cir", "ref").with_type_params(vec![pointee]))
}

/// Register memory types with the context.
///
/// Types are created on-demand via [`ptr_type`] and [`ref_type`];
/// no upfront registration is needed.
pub fn register_types(_ctx: &mut Context) {
    // Types are interned lazily by the Context when ptr_type()/ref_type()
    // are called. No upfront registration step required.
}
