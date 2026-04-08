//! # Tagged Union Types
//!
//! Defines the CIR tagged union type:
//!
//! `!cir.tagged_union<"Name", "V0": T0, "V1": T1, ...>` — a discriminated
//! union with named variants, each carrying a payload of a specific type.
//! Represented at runtime as `{ tag: i32, payload: [max_payload_size x i8] }`
//! where the tag indicates the active variant.

use mlif::Context;

/// Tagged union type: `!cir.tagged_union<"Name", "V0": T0, "V1": T1, ...>`.
pub struct TaggedUnionType {
    // TODO: name, variant names, variant payload types
}

/// Register `!cir.tagged_union` with the type system.
pub fn register_types(_ctx: &mut Context) {
    todo!()
}
