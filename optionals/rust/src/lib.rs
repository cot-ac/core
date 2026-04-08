//! # cot-optionals
//!
//! Optional construct for CIR: nullable wrapper `!cir.optional<T>`.
//! 4 ops: none, wrap_optional, is_non_null, optional_payload.
//! Reference: Zig `?T`, Swift `Optional<T>`.

pub mod lowering;
pub mod ops;
pub mod types;

/// Register optional type and ops with the MLIF context.
pub fn register(ctx: &mut mlif::Context, _sema: &mut mlif::CIRSema) {
    types::register_types(ctx);
    ops::register_ops(ctx);
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlif::*;

    #[test]
    fn register_optional_ops() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        let d = ctx.get_dialect("cir").unwrap();
        assert!(d.get_op("cir.none").is_some());
        assert!(d.get_op("cir.wrap_optional").is_some());
        assert!(d.get_op("cir.is_non_null").is_some());
        assert!(d.get_op("cir.optional_payload").is_some());
    }

    #[test]
    fn register_all_4_ops() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        assert_eq!(ctx.get_dialect("cir").unwrap().ops().len(), 4);
    }

    #[test]
    fn create_optional_type() {
        let mut ctx = Context::new();
        let i32_ty = ctx.integer_type(32);
        let opt = types::optional_type(&mut ctx, i32_ty);
        let opt2 = types::optional_type(&mut ctx, i32_ty);
        assert_eq!(opt, opt2, "same optional type should intern");

        let f64_ty = ctx.float_type(64);
        let opt_f = types::optional_type(&mut ctx, f64_ty);
        assert_ne!(opt, opt_f, "different payload produces different type");
    }

    #[test]
    fn build_none_op() {
        let mut ctx = Context::new();
        let i32_ty = ctx.integer_type(32);
        let block = ctx.create_block();

        let val = ops::build_none(&mut ctx, block, i32_ty, Location::unknown());
        let opt_ty = types::optional_type(&mut ctx, i32_ty);
        assert_eq!(ctx.value_type(val), opt_ty);

        let op_list: Vec<OpId> = ctx.block_ops(block).collect();
        assert!(ctx[op_list[0]].is_a("cir.none"));
    }

    #[test]
    fn build_wrap_and_unwrap() {
        let mut ctx = Context::new();
        let i32_ty = ctx.integer_type(32);
        let block = ctx.create_block();
        let input = ctx.block_add_argument(block, i32_ty);

        let wrapped = ops::build_wrap_optional(&mut ctx, block, input, Location::unknown());
        let opt_ty = types::optional_type(&mut ctx, i32_ty);
        assert_eq!(ctx.value_type(wrapped), opt_ty);

        let check = ops::build_is_non_null(&mut ctx, block, wrapped, Location::unknown());
        assert_eq!(ctx.value_type(check), ctx.integer_type(1));

        let payload =
            ops::build_optional_payload(&mut ctx, block, wrapped, i32_ty, Location::unknown());
        assert_eq!(ctx.value_type(payload), i32_ty);

        let op_list: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(op_list.len(), 3);
        assert!(ctx[op_list[0]].is_a("cir.wrap_optional"));
        assert!(ctx[op_list[1]].is_a("cir.is_non_null"));
        assert!(ctx[op_list[2]].is_a("cir.optional_payload"));
    }
}
