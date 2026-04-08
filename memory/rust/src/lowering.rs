//! # Memory Lowering
//!
//! Lowers CIR memory ops to Cranelift IR:
//! - `alloca` -> `stack_slot` allocation
//! - `store` -> Cranelift `store` instruction
//! - `load` -> Cranelift `load` instruction
//! - `addr_of` -> `stack_addr` for the corresponding slot
//! - `deref` -> `load` with type derived from `!cir.ref<T>`
//!
//! Phase 3 -- not yet implemented.

// Lowering will be implemented in Phase 3 when we add the Cranelift backend.
