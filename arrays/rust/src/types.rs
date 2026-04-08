//! # Array Types
//!
//! Defines the CIR array type:
//!
//! `!cir.array<N x T>` -- a fixed-size array of N elements of type T. The size
//! N is a compile-time constant encoded as an integer parameter.
//!
//! ## Encoding
//!
//! - `int_params[0]` = array size N
//! - `type_params[0]` = element type T

use mlif::{Context, ExtensionType, TypeId};

/// Create a `!cir.array<N x T>` type.
///
/// The size N is stored in `int_params[0]` and the element type T
/// in `type_params[0]`.
pub fn array_type(ctx: &mut Context, size: i64, elem_ty: TypeId) -> TypeId {
    ctx.extension_type(
        ExtensionType::new("cir", "array")
            .with_int_params(vec![size])
            .with_type_params(vec![elem_ty]),
    )
}

/// Register array types with the context.
///
/// Extension types are registered on-demand via `Context::extension_type`,
/// so this function is a no-op but maintains the construct registration
/// pattern for forward compatibility.
pub fn register_types(_ctx: &mut Context) {
    // Extension types are interned on first use -- no upfront registration needed.
}
