//! # Value Witness Table Operations
//!
//! Defines 7 CIR operations for type-abstract value manipulation:
//!
//! - `vwt_size` — query the byte size of a type through its VWT
//! - `vwt_stride` — query the stride (size + padding to alignment) of a type
//! - `vwt_align` — query the alignment requirement of a type
//! - `vwt_copy` — copy a value from source to destination through its VWT
//! - `vwt_destroy` — destroy a value (run destructors) through its VWT
//! - `vwt_move` — move a value from source to destination through its VWT
//! - `vwt_init_buffer` — initialize an inline buffer or allocate for a type

use mlif::Context;

/// Register all 7 VWT ops with the given context.
pub fn register_ops(_ctx: &mut Context) {
    todo!()
}
