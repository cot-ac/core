//! # cot-generics
//!
//! Generics construct for CIR: type parameter `!cir.type_param<"T">` and
//! `cir.generic_apply` op. GenericSpecializerStep monomorphizes all
//! generic calls during CIRSema. No lowering — generics are fully
//! eliminated before that phase.
//! Reference: Rust monomorphization, Swift SIL apply.

pub mod ops;
pub mod transform;
pub mod types;

/// Register generics type, op, and sema step with the MLIF context.
pub fn register(ctx: &mut mlif::Context, sema: &mut mlif::CIRSema) {
    types::register_types(ctx);
    ops::register_ops(ctx);
    sema.add_step(Box::new(transform::GenericsSemaStep::new()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlif::*;

    #[test]
    fn register_generics_ops() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        let d = ctx.get_dialect("cir").unwrap();
        assert!(d.get_op("cir.generic_apply").is_some());
    }

    #[test]
    fn register_all_1_ops() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        assert_eq!(ctx.get_dialect("cir").unwrap().ops().len(), 1);
    }

    #[test]
    fn create_type_param() {
        let mut ctx = Context::new();
        let t = types::type_param_type(&mut ctx, "T");
        let t2 = types::type_param_type(&mut ctx, "T");
        assert_eq!(t, t2, "same type_param should intern");

        let u = types::type_param_type(&mut ctx, "U");
        assert_ne!(t, u, "different param name → different type");
    }

    #[test]
    fn build_generic_apply() {
        let mut ctx = Context::new();
        let i32_ty = ctx.integer_type(32);
        let _t_ty = types::type_param_type(&mut ctx, "T");
        let block = ctx.create_block();
        let arg = ctx.block_add_argument(block, i32_ty);

        let results = ops::build_generic_apply(
            &mut ctx,
            block,
            "identity",
            &[arg],
            &["T"],
            &[i32_ty],
            &[i32_ty],
            Location::unknown(),
        );
        assert_eq!(results.len(), 1);
        assert_eq!(ctx.value_type(results[0]), i32_ty);

        let op_list: Vec<OpId> = ctx.block_ops(block).collect();
        assert!(ctx[op_list[0]].is_a("cir.generic_apply"));
        match ctx[op_list[0]].get_attribute("callee") {
            Some(Attribute::SymbolRef(s)) => assert_eq!(s, "identity"),
            _ => panic!("expected callee symbol ref"),
        }
    }
}
