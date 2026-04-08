//! Cranelift lowering for union ops (Phase 3).
//!
//! Layout: stack_slot with struct<(i8 tag, [max_payload_bytes x i8])>.
//! union_init → store tag + bitcast payload into byte array.
//! union_tag → load i8 from offset 0.
//! union_payload → load from payload offset, bitcast to variant type.

use mlif::Context;

pub fn lower_unions(_ctx: &mut Context) {
    todo!()
}
