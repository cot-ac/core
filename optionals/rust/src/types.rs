//! Optional type: `!cir.optional<T>`.
//!
//! A nullable wrapper. Pointer-like payloads use null-pointer optimization
//! (the pointer IS the optional). Value payloads lower to struct<(T, i1)>.
//! Reference: Zig `?T`, Swift `Optional<T>`.

use mlif::{Context, ExtensionType, TypeId};

/// Create `!cir.optional<T>` with the given payload type.
pub fn optional_type(ctx: &mut Context, payload: TypeId) -> TypeId {
    ctx.extension_type(ExtensionType::new("cir", "optional").with_type_params(vec![payload]))
}

/// No upfront registration needed — types are interned on demand.
pub fn register_types(_ctx: &mut Context) {}
