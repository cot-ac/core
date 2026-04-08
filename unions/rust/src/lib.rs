//! # cot-unions
//!
//! Tagged union construct for CIR: `!cir.tagged_union<"Name", "V0": T0, ...>`.
//! 3 ops: union_init, union_tag, union_payload.
//! Reference: Rust enum with data, Swift indirect enum.

pub mod lowering;
pub mod ops;
pub mod types;

/// Register tagged union type and ops with the MLIF context.
pub fn register(ctx: &mut mlif::Context, _sema: &mut mlif::CIRSema) {
    types::register_types(ctx);
    ops::register_ops(ctx);
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlif::*;

    #[test]
    fn register_union_ops() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        let d = ctx.get_dialect("cir").unwrap();
        assert!(d.get_op("cir.union_init").is_some());
        assert!(d.get_op("cir.union_tag").is_some());
        assert!(d.get_op("cir.union_payload").is_some());
    }

    #[test]
    fn register_all_3_ops() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        assert_eq!(ctx.get_dialect("cir").unwrap().ops().len(), 3);
    }

    #[test]
    fn create_tagged_union_type() {
        let mut ctx = Context::new();
        let f64_ty = ctx.float_type(64);
        let i32_ty = ctx.integer_type(32);
        let shape = types::tagged_union_type(
            &mut ctx,
            "Shape",
            &["Circle", "Rect"],
            &[f64_ty, i32_ty],
        );
        let shape2 = types::tagged_union_type(
            &mut ctx,
            "Shape",
            &["Circle", "Rect"],
            &[f64_ty, i32_ty],
        );
        assert_eq!(shape, shape2, "same tagged_union type should intern");
    }

    #[test]
    fn build_union_init_with_payload() {
        let mut ctx = Context::new();
        let f64_ty = ctx.float_type(64);
        let i32_ty = ctx.integer_type(32);
        let shape_ty = types::tagged_union_type(
            &mut ctx,
            "Shape",
            &["Circle", "Rect"],
            &[f64_ty, i32_ty],
        );
        let block = ctx.create_block();
        let radius = ctx.block_add_argument(block, f64_ty);

        let val = ops::build_union_init(
            &mut ctx,
            block,
            shape_ty,
            "Circle",
            Some(radius),
            Location::unknown(),
        );
        assert_eq!(ctx.value_type(val), shape_ty);

        let op_list: Vec<OpId> = ctx.block_ops(block).collect();
        assert!(ctx[op_list[0]].is_a("cir.union_init"));
        match ctx[op_list[0]].get_attribute("variant") {
            Some(Attribute::String(s)) => assert_eq!(s, "Circle"),
            _ => panic!("expected variant string attribute"),
        }
    }

    #[test]
    fn build_union_tag_and_payload() {
        let mut ctx = Context::new();
        let f64_ty = ctx.float_type(64);
        let shape_ty =
            types::tagged_union_type(&mut ctx, "Shape", &["Circle"], &[f64_ty]);
        let block = ctx.create_block();
        let input = ctx.block_add_argument(block, shape_ty);

        let tag = ops::build_union_tag(&mut ctx, block, input, Location::unknown());
        assert_eq!(ctx.value_type(tag), ctx.integer_type(8));

        let payload = ops::build_union_payload(
            &mut ctx,
            block,
            input,
            "Circle",
            f64_ty,
            Location::unknown(),
        );
        assert_eq!(ctx.value_type(payload), f64_ty);
    }
}
