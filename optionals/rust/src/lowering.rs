//! # Optionals Lowering
//!
//! Lowers CIR optional ops to Cranelift IR with two strategies:
//!
//! **Null-pointer optimization** (for pointer-like payloads):
//! - `none` -> `iconst 0` (null pointer)
//! - `wrap_optional` -> identity (pointer is already non-null)
//! - `is_non_null` -> `icmp ne, val, 0`
//! - `optional_payload` -> identity (pointer value)
//!
//! **Tagged struct** (for value-type payloads):
//! - `none` -> stack_slot with `{ i1 0, undef }`
//! - `wrap_optional` -> stack_slot with `{ i1 1, payload }`
//! - `is_non_null` -> load the i1 tag
//! - `optional_payload` -> load from payload offset

use mlif::Context;

/// Lower all optional ops in a module to Cranelift IR.
pub fn lower_optionals(_ctx: &mut Context) {
    todo!()
}
