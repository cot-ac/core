//! # Flow Lowering
//!
//! Lowers CIR flow ops to Cranelift IR:
//! - `br` -> `jump`
//! - `condbr` -> `brif`
//! - `switch` -> Cranelift `br_table`
//! - `trap` -> `trap`
//!
//! Phase 3 -- not yet implemented.

// Lowering will be implemented in Phase 3 when we add the Cranelift backend.
