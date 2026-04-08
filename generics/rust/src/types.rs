//! # Generic Types
//!
//! Defines the CIR type parameter placeholder:
//!
//! `!cir.type_param<"T">` — a placeholder type that stands for a concrete type
//! to be filled in during monomorphization. Only valid in generic function
//! signatures and bodies; must be eliminated by the GenericSpecializer pass
//! before lowering.

use mlif::Context;

/// Type parameter placeholder: `!cir.type_param<"T">`.
pub struct TypeParamType {
    // TODO: parameter name
}

/// Register `!cir.type_param` with the type system.
pub fn register_types(_ctx: &mut Context) {
    todo!()
}
