//! # Errors Lowering
//!
//! Lowers CIR error ops to Cranelift IR using a struct layout `{ payload: T, error_code: i16 }`:
//! - `wrap_result` -> store payload at offset 0, store `iconst.i16 0` at error offset
//! - `wrap_error` -> store error code at error offset, payload is undef
//! - `is_error` -> load i16 error code, `icmp ne, code, 0`
//! - `error_payload` -> load from payload offset
//! - `error_code` -> load from error code offset

use mlif::Context;

/// Lower all error ops in a module to Cranelift IR.
pub fn lower_errors(_ctx: &mut Context) {
    todo!()
}
