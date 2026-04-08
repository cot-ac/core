//! # Existential Types
//!
//! Defines the CIR existential type:
//!
//! `!cir.existential<"P">` — an existential container for a value that
//! conforms to protocol/trait "P". At runtime, represented as a pair of
//! `(value_ptr, witness_table_ptr)`. The witness table contains function
//! pointers for each method required by the protocol.

use mlif::Context;

/// Existential type: `!cir.existential<"P">`.
pub struct ExistentialType {
    // TODO: protocol name
}

/// Register `!cir.existential` with the type system.
pub fn register_types(_ctx: &mut Context) {
    todo!()
}
