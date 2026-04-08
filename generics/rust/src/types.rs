//! Type parameter: `!cir.type_param<"T">`.
//!
//! A placeholder type in generic function signatures and bodies.
//! Eliminated by GenericSpecializerStep before lowering.
//! Falls back to ptr if it reaches lowering unresolved.
//! Reference: Swift SIL archetype, Rust type parameter.

use mlif::{Context, ExtensionType, TypeId};

/// Create `!cir.type_param<"T">` with the given parameter name.
pub fn type_param_type(ctx: &mut Context, name: &str) -> TypeId {
    ctx.extension_type(
        ExtensionType::new("cir", "type_param").with_string_params(vec![name.to_string()]),
    )
}

/// No upfront registration needed — types are interned on demand.
pub fn register_types(_ctx: &mut Context) {}
