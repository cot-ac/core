//! # Slices Lowering
//!
//! Lowers CIR slice ops to Cranelift IR using a two-value (ptr, len) struct:
//! - `string_constant` -> global data section + `global_value` for ptr, `iconst` for len
//! - `slice_ptr` -> extract first element of the pair
//! - `slice_len` -> extract second element of the pair
//! - `slice_elem` -> ptr + index * sizeof(T), then `load`
//! - `array_to_slice` -> `stack_addr` for ptr, `iconst N` for len
//!
//! Phase 3 -- not yet implemented.

// Lowering will be implemented in Phase 3 when we add the Cranelift backend.
