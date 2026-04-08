//! # Flow Lowering
//!
//! Lowers CIR flow ops to Cranelift IR:
//! - `br` -> `jump`
//! - `condbr` -> `brif`
//! - `switch` -> Cranelift `br_table`
//! - `trap` -> `trap`

use mlif::Context;

/// Lower all flow ops in a module to Cranelift IR.
pub fn lower_flow(_ctx: &mut Context) {
    todo!()
}
