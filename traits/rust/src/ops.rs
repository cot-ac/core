//! # Trait Operations
//!
//! Defines 6 CIR operations:
//!
//! - `witness_table` — declare/define a witness table for a type conforming to a protocol
//! - `trait_call` — dispatch a method call through an existential's witness table
//! - `witness_method` — extract a specific method function pointer from a witness table
//! - `init_existential` — wrap a concrete value + witness table into an existential container
//! - `open_existential` — unwrap an existential to access the concrete value pointer
//! - `deinit_existential` — destroy an existential container (calls destroy witness if needed)

use mlif::Context;

/// Register all 6 trait ops with the given context.
pub fn register_ops(_ctx: &mut Context) {
    todo!()
}
