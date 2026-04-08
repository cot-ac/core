//! Enum type: `!cir.enum<"Name", TagType, "V0", "V1", ...>`.
//!
//! A named enumeration with an integer tag type and named variants.
//! Lowers directly to the tag type (the enum IS the integer).
//! Reference: Rust C-like enum, C/C++ enum.

use mlif::{Context, ExtensionType, TypeId};

/// Create `!cir.enum<"Name", TagType, "V0", "V1", ...>`.
///
/// Parameters:
/// - `name`: enum name (e.g., "Color")
/// - `tag_ty`: underlying integer type (e.g., i32)
/// - `variants`: variant names (e.g., ["Red", "Green", "Blue"])
pub fn enum_type(ctx: &mut Context, name: &str, tag_ty: TypeId, variants: &[&str]) -> TypeId {
    let mut string_params = vec![name.to_string()];
    string_params.extend(variants.iter().map(|s| s.to_string()));
    ctx.extension_type(
        ExtensionType::new("cir", "enum")
            .with_string_params(string_params)
            .with_type_params(vec![tag_ty]),
    )
}

/// No upfront registration needed — types are interned on demand.
pub fn register_types(_ctx: &mut Context) {}
