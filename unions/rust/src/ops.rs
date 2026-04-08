//! # Union Operations
//!
//! Defines 3 CIR operations:
//!
//! - `union_init` — create a tagged union value with a specific variant and payload
//! - `union_tag` — extract the integer tag (variant discriminant) from a union
//! - `union_payload` — extract the payload for a specific variant (UB if wrong variant)

use mlif::Context;

/// Register all 3 union ops with the given context.
pub fn register_ops(_ctx: &mut Context) {
    todo!()
}
