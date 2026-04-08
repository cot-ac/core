//! Existential type: `!cir.existential<"P">`.
//!
//! An existential container for a value conforming to protocol "P".
//! At runtime: 24-byte inline buffer + VWT ptr + PWT ptr.
//! Reference: Swift SIL existential container.

use mlif::{Context, ExtensionType, TypeId};

/// Create `!cir.existential<"P">` for the named protocol.
pub fn existential_type(ctx: &mut Context, protocol: &str) -> TypeId {
    ctx.extension_type(
        ExtensionType::new("cir", "existential").with_string_params(vec![protocol.to_string()]),
    )
}

/// No upfront registration needed — types are interned on demand.
pub fn register_types(_ctx: &mut Context) {}
