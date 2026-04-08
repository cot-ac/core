//! # Structs Lowering
//!
//! Lowers CIR struct ops to Cranelift IR:
//! - `struct_init` -> `stack_slot` + sequence of `store` at field offsets
//! - `field_val` -> `load` at field offset from struct base
//! - `field_ptr` -> `stack_addr` + offset computation (iadd_imm)
//!
//! Field offsets are computed from the target data layout, respecting alignment.
//!
//! Phase 3 -- not yet implemented.

// Lowering will be implemented in Phase 3 when we add the Cranelift backend.
