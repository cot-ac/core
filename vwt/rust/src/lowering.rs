//! # VWT Lowering
//!
//! Lowers CIR VWT ops to Cranelift IR using GEP into VWT structs + indirect calls:
//! - `vwt_size` -> load from VWT struct at size offset
//! - `vwt_stride` -> load from VWT struct at stride offset
//! - `vwt_align` -> load from VWT struct at alignment offset
//! - `vwt_copy` -> load copy function pointer from VWT, `call_indirect`
//! - `vwt_destroy` -> load destroy function pointer from VWT, `call_indirect`
//! - `vwt_move` -> load move function pointer from VWT, `call_indirect`
//! - `vwt_init_buffer` -> load initializeBufferWithCopyOfBuffer from VWT, `call_indirect`

use mlif::Context;

/// Lower all VWT ops in a module to Cranelift IR.
pub fn lower_vwt(_ctx: &mut Context) {
    todo!()
}
