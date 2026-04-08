//! Cranelift lowering for VWT ops (Phase 3).
//!
//! Data queries: GEP into VWT struct + load at known offsets.
//! Function calls: load fn ptr from VWT, call_indirect.

use mlif::Context;

pub fn lower_vwt(_ctx: &mut Context) {
    todo!()
}
