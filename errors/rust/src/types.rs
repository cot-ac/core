//! Error union type: `!cir.error_union<T>`.
//!
//! A tagged union of success payload `T` or i16 error code.
//! Lowers to struct<(T, i16)> where error_code == 0 means success.
//! Reference: Zig error unions, Rust `Result<T, E>`.

use mlif::{Context, ExtensionType, TypeId};

/// Create `!cir.error_union<T>` with the given payload type.
pub fn error_union_type(ctx: &mut Context, payload: TypeId) -> TypeId {
    ctx.extension_type(ExtensionType::new("cir", "error_union").with_type_params(vec![payload]))
}

/// No upfront registration needed — types are interned on demand.
pub fn register_types(_ctx: &mut Context) {}
