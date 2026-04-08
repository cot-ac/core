//! # cot-traits
//!
//! Traits construct for CIR: existential type `!cir.existential<"P">` and
//! 6 ops for witness tables, static/dynamic dispatch, and existential containers.
//! Reference: Swift SIL witness tables + existential ops.

pub mod lowering;
pub mod ops;
pub mod transform;
pub mod types;

/// Register existential type and trait ops with the MLIF context.
pub fn register(ctx: &mut mlif::Context, _sema: &mut mlif::CIRSema) {
    types::register_types(ctx);
    ops::register_ops(ctx);
    // WitnessThunkGenerator is a lowering concern, not a sema step.
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlif::*;

    #[test]
    fn register_trait_ops() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        let d = ctx.get_dialect("cir").unwrap();
        assert!(d.get_op("cir.witness_table").is_some());
        assert!(d.get_op("cir.trait_call").is_some());
        assert!(d.get_op("cir.witness_method").is_some());
        assert!(d.get_op("cir.init_existential").is_some());
        assert!(d.get_op("cir.open_existential").is_some());
        assert!(d.get_op("cir.deinit_existential").is_some());
    }

    #[test]
    fn register_all_6_ops() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        assert_eq!(ctx.get_dialect("cir").unwrap().ops().len(), 6);
    }

    #[test]
    fn create_existential_type() {
        let mut ctx = Context::new();
        let summable = types::existential_type(&mut ctx, "Summable");
        let summable2 = types::existential_type(&mut ctx, "Summable");
        assert_eq!(summable, summable2);

        let drawable = types::existential_type(&mut ctx, "Drawable");
        assert_ne!(summable, drawable);
    }

    #[test]
    fn build_witness_table_op() {
        let mut ctx = Context::new();
        let block = ctx.create_block();

        ops::build_witness_table(
            &mut ctx,
            block,
            "Summable_Point",
            "Summable",
            "Point",
            &["sum"],
            &["Point_sum"],
            Location::unknown(),
        );

        let op_list: Vec<OpId> = ctx.block_ops(block).collect();
        assert!(ctx[op_list[0]].is_a("cir.witness_table"));
        match ctx[op_list[0]].get_attribute("sym_name") {
            Some(Attribute::String(s)) => assert_eq!(s, "Summable_Point"),
            _ => panic!("expected sym_name"),
        }
        match ctx[op_list[0]].get_attribute("protocol") {
            Some(Attribute::String(s)) => assert_eq!(s, "Summable"),
            _ => panic!("expected protocol"),
        }
    }

    #[test]
    fn build_open_existential_op() {
        let mut ctx = Context::new();
        let ptr_ty = ctx.extension_type(ExtensionType::new("cir", "ptr"));
        let exist_ty = types::existential_type(&mut ctx, "Summable");
        let block = ctx.create_block();
        let container = ctx.block_add_argument(block, exist_ty);

        let (buf, vwt, pwt) = ops::build_open_existential(
            &mut ctx,
            block,
            container,
            ptr_ty,
            ptr_ty,
            ptr_ty,
            Location::unknown(),
        );
        assert_eq!(ctx.value_type(buf), ptr_ty);
        assert_eq!(ctx.value_type(vwt), ptr_ty);
        assert_eq!(ctx.value_type(pwt), ptr_ty);
    }
}
