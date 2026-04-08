//! # Arrays Lowering
//!
//! Lowers CIR array ops to Cranelift IR:
//! - `array_init` -> `stack_slot` + sequence of `store` at element offsets
//! - `elem_val` -> `load` at index * sizeof(T) from array base
//! - `elem_ptr` -> `stack_addr` + index * sizeof(T) computation (imul + iadd)
//!
//! Phase 3 -- not yet implemented.

// Lowering will be implemented in Phase 3 when we add the Cranelift backend.
