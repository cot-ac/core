//! # Slice Types
//!
//! Defines the CIR slice type:
//!
//! `!cir.slice<T>` -- a fat pointer consisting of a base pointer and a length.
//! Represented at runtime as a two-field struct: `{ ptr: *T, len: i64 }`.
//!
//! ## Encoding
//!
//! - `type_params[0]` = element type T

use mlif::{Context, ExtensionType, TypeId};

/// Create a `!cir.slice<T>` type.
///
/// The element type T is stored in `type_params[0]`.
pub fn slice_type(ctx: &mut Context, elem_ty: TypeId) -> TypeId {
    ctx.extension_type(ExtensionType::new("cir", "slice").with_type_params(vec![elem_ty]))
}

/// Register slice types with the context.
///
/// Extension types are registered on-demand via `Context::extension_type`,
/// so this function is a no-op but maintains the construct registration
/// pattern for forward compatibility.
pub fn register_types(_ctx: &mut Context) {
    // Extension types are interned on first use -- no upfront registration needed.
}
