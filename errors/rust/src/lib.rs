//! # cot-errors
//!
//! Error union construct for CIR: `!cir.error_union<T>`.
//! 5 ops: wrap_result, wrap_error, is_error, error_payload, error_code.
//! Reference: Zig error unions, Rust `Result<T, E>`.

pub mod lowering;
pub mod ops;
pub mod types;

/// Register error union type and ops with the MLIF context.
pub fn register(ctx: &mut mlif::Context, _sema: &mut mlif::CIRSema) {
    types::register_types(ctx);
    ops::register_ops(ctx);
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlif::*;

    #[test]
    fn register_error_ops() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        let d = ctx.get_dialect("cir").unwrap();
        assert!(d.get_op("cir.wrap_result").is_some());
        assert!(d.get_op("cir.wrap_error").is_some());
        assert!(d.get_op("cir.is_error").is_some());
        assert!(d.get_op("cir.error_payload").is_some());
        assert!(d.get_op("cir.error_code").is_some());
    }

    #[test]
    fn register_all_5_ops() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        assert_eq!(ctx.get_dialect("cir").unwrap().ops().len(), 5);
    }

    #[test]
    fn create_error_union_type() {
        let mut ctx = Context::new();
        let i32_ty = ctx.integer_type(32);
        let eu = types::error_union_type(&mut ctx, i32_ty);
        let eu2 = types::error_union_type(&mut ctx, i32_ty);
        assert_eq!(eu, eu2, "same error_union type should intern");
    }

    #[test]
    fn build_wrap_result_and_check() {
        let mut ctx = Context::new();
        let i32_ty = ctx.integer_type(32);
        let block = ctx.create_block();
        let input = ctx.block_add_argument(block, i32_ty);

        let wrapped = ops::build_wrap_result(&mut ctx, block, input, Location::unknown());
        let eu_ty = types::error_union_type(&mut ctx, i32_ty);
        assert_eq!(ctx.value_type(wrapped), eu_ty);

        let check = ops::build_is_error(&mut ctx, block, wrapped, Location::unknown());
        assert_eq!(ctx.value_type(check), ctx.integer_type(1));

        let payload =
            ops::build_error_payload(&mut ctx, block, wrapped, i32_ty, Location::unknown());
        assert_eq!(ctx.value_type(payload), i32_ty);

        let code = ops::build_error_code(&mut ctx, block, wrapped, Location::unknown());
        assert_eq!(ctx.value_type(code), ctx.integer_type(16));

        let op_list: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(op_list.len(), 4);
        assert!(ctx[op_list[0]].is_a("cir.wrap_result"));
        assert!(ctx[op_list[1]].is_a("cir.is_error"));
        assert!(ctx[op_list[2]].is_a("cir.error_payload"));
        assert!(ctx[op_list[3]].is_a("cir.error_code"));
    }

    #[test]
    fn build_wrap_error() {
        let mut ctx = Context::new();
        let i32_ty = ctx.integer_type(32);
        let i16_ty = ctx.integer_type(16);
        let block = ctx.create_block();
        let code = ctx.block_add_argument(block, i16_ty);

        let wrapped =
            ops::build_wrap_error(&mut ctx, block, code, i32_ty, Location::unknown());
        let eu_ty = types::error_union_type(&mut ctx, i32_ty);
        assert_eq!(ctx.value_type(wrapped), eu_ty);
    }
}
