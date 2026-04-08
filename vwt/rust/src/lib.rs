//! # cot-vwt
//!
//! Value Witness Table construct for CIR. 7 ops for type-abstract
//! queries (size/stride/align) and lifetime operations (copy/destroy/move).
//! No custom types — operates on VWT pointers.
//! Reference: Swift ABI ValueWitnessTable.

pub mod lowering;
pub mod ops;
pub mod transform;

/// Register VWT ops with the MLIF context.
pub fn register(ctx: &mut mlif::Context, _sema: &mut mlif::CIRSema) {
    ops::register_ops(ctx);
    // WitnessTableGenerator is a lowering concern, not a sema step.
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlif::*;

    #[test]
    fn register_vwt_ops() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        let d = ctx.get_dialect("cir").unwrap();
        assert!(d.get_op("cir.vwt_size").is_some());
        assert!(d.get_op("cir.vwt_stride").is_some());
        assert!(d.get_op("cir.vwt_align").is_some());
        assert!(d.get_op("cir.vwt_copy").is_some());
        assert!(d.get_op("cir.vwt_destroy").is_some());
        assert!(d.get_op("cir.vwt_move").is_some());
        assert!(d.get_op("cir.vwt_init_buffer").is_some());
    }

    #[test]
    fn register_all_7_ops() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        assert_eq!(ctx.get_dialect("cir").unwrap().ops().len(), 7);
    }

    #[test]
    fn build_vwt_query_ops() {
        let mut ctx = Context::new();
        let ptr_ty = ctx.extension_type(ExtensionType::new("cir", "ptr"));
        let idx_ty = ctx.index_type();
        let block = ctx.create_block();
        let vwt = ctx.block_add_argument(block, ptr_ty);

        let size = ops::build_vwt_size(&mut ctx, block, vwt, idx_ty, Location::unknown());
        assert_eq!(ctx.value_type(size), idx_ty);

        let stride = ops::build_vwt_stride(&mut ctx, block, vwt, idx_ty, Location::unknown());
        assert_eq!(ctx.value_type(stride), idx_ty);

        let align = ops::build_vwt_align(&mut ctx, block, vwt, idx_ty, Location::unknown());
        assert_eq!(ctx.value_type(align), idx_ty);

        let op_list: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(op_list.len(), 3);
        assert!(ctx[op_list[0]].is_a("cir.vwt_size"));
        assert!(ctx[op_list[1]].is_a("cir.vwt_stride"));
        assert!(ctx[op_list[2]].is_a("cir.vwt_align"));
    }

    #[test]
    fn build_vwt_action_ops() {
        let mut ctx = Context::new();
        let ptr_ty = ctx.extension_type(ExtensionType::new("cir", "ptr"));
        let block = ctx.create_block();
        let vwt = ctx.block_add_argument(block, ptr_ty);
        let src = ctx.block_add_argument(block, ptr_ty);
        let dst = ctx.block_add_argument(block, ptr_ty);

        ops::build_vwt_copy(&mut ctx, block, vwt, src, dst, Location::unknown());
        ops::build_vwt_destroy(&mut ctx, block, vwt, src, Location::unknown());
        ops::build_vwt_move(&mut ctx, block, vwt, src, dst, Location::unknown());

        let result =
            ops::build_vwt_init_buffer(&mut ctx, block, vwt, src, dst, ptr_ty, Location::unknown());
        assert_eq!(ctx.value_type(result), ptr_ty);

        let op_list: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(op_list.len(), 4);
    }

    #[test]
    fn op_traits_correct() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        let d = ctx.get_dialect("cir").unwrap();
        // Data queries are Pure.
        assert!(d.get_op("cir.vwt_size").unwrap().has_trait(&OpTrait::Pure));
        assert!(d.get_op("cir.vwt_stride").unwrap().has_trait(&OpTrait::Pure));
        assert!(d.get_op("cir.vwt_align").unwrap().has_trait(&OpTrait::Pure));
        // Function calls are NOT Pure.
        assert!(!d.get_op("cir.vwt_copy").unwrap().has_trait(&OpTrait::Pure));
        assert!(!d.get_op("cir.vwt_destroy").unwrap().has_trait(&OpTrait::Pure));
        assert!(!d.get_op("cir.vwt_move").unwrap().has_trait(&OpTrait::Pure));
        assert!(!d.get_op("cir.vwt_init_buffer").unwrap().has_trait(&OpTrait::Pure));
    }
}
