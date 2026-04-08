//! # Enum Types
//!
//! Defines the CIR enum type:
//!
//! `!cir.enum<"Name", TagType, "V0", "V1", ...>` — a named enumeration with
//! an explicit integer tag type and a list of named variants. Each variant
//! maps to a sequential integer starting from 0 (or explicitly assigned).

use mlif::Context;

/// Enum type: `!cir.enum<"Name", TagType, "V0", "V1", ...>`.
pub struct EnumType {
    // TODO: name, tag type, variant names
}

/// Register `!cir.enum` with the type system.
pub fn register_types(_ctx: &mut Context) {
    todo!()
}
