//! # Struct Types
//!
//! Defines the CIR struct type:
//!
//! `!cir.struct<"Name", "field": T, ...>` — a named aggregate with ordered,
//! named fields. The struct carries its layout (field offsets, alignment,
//! total size) once resolved against a target data layout.

use mlif::Context;

/// Named struct type: `!cir.struct<"Name", "field0": T0, "field1": T1, ...>`.
pub struct StructType {
    // TODO: name, field names, field types
}

/// Register `!cir.struct` with the type system.
pub fn register_types(_ctx: &mut Context) {
    todo!()
}
