//! # Struct Types
//!
//! Defines the CIR struct type:
//!
//! `!cir.struct<"Name", "field0": T0, "field1": T1, ...>` — a named aggregate
//! with ordered, named fields. Encoded as an MLIF extension type under the
//! `cir` dialect with string parameters for the struct name and field names,
//! and type parameters for the field types.
//!
//! ## Encoding
//!
//! - `string_params[0]` = struct name
//! - `string_params[1..]` = field names
//! - `type_params` = field types (same count as field names)

use mlif::{Context, ExtensionType, TypeId};

/// Create a `!cir.struct<"Name", "f1": T1, "f2": T2, ...>` type.
///
/// The struct name occupies `string_params[0]`, field names occupy
/// `string_params[1..]`, and field types are stored in `type_params`.
pub fn struct_type(
    ctx: &mut Context,
    name: &str,
    field_names: &[&str],
    field_types: &[TypeId],
) -> TypeId {
    let mut strings = vec![name.to_string()];
    strings.extend(field_names.iter().map(|s| s.to_string()));
    ctx.extension_type(
        ExtensionType::new("cir", "struct")
            .with_string_params(strings)
            .with_type_params(field_types.to_vec()),
    )
}

/// Register struct types with the context.
///
/// Extension types are registered on-demand via `Context::extension_type`,
/// so this function is a no-op but maintains the construct registration
/// pattern for forward compatibility.
pub fn register_types(_ctx: &mut Context) {
    // Extension types are interned on first use — no upfront registration needed.
}
