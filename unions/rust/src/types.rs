//! Tagged union type: `!cir.tagged_union<"Name", "V0": T0, "V1": T1, ...>`.
//!
//! Discriminated union with i8 tag and variant payloads.
//! Lowers to struct<(i8, [max_payload_bytes x i8])>.
//! Reference: Rust enum with data, Swift indirect enum.

use mlif::{Context, ExtensionType, TypeId};

/// Create `!cir.tagged_union<"Name", "V0": T0, "V1": T1, ...>`.
///
/// Parameters:
/// - `name`: union name (e.g., "Shape")
/// - `variant_names`: variant names (e.g., ["Circle", "Rect"])
/// - `variant_types`: payload types per variant (e.g., [f64_ty, struct_ty])
pub fn tagged_union_type(
    ctx: &mut Context,
    name: &str,
    variant_names: &[&str],
    variant_types: &[TypeId],
) -> TypeId {
    let mut string_params = vec![name.to_string()];
    string_params.extend(variant_names.iter().map(|s| s.to_string()));
    ctx.extension_type(
        ExtensionType::new("cir", "tagged_union")
            .with_string_params(string_params)
            .with_type_params(variant_types.to_vec()),
    )
}

/// No upfront registration needed — types are interned on demand.
pub fn register_types(_ctx: &mut Context) {}
