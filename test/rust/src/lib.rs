//! # cot-test
//!
//! Test construct for CIR. 2 ops: assert, test_case.
//! TestSemaStep extracts test_case regions into @__test_N functions
//! and generates @main to run them all.
//! Reference: Zig `test "name" { }`, Rust `#[test]`.

pub mod lowering;
pub mod ops;
pub mod transform;

/// Register test ops and sema step with the MLIF context.
pub fn register(ctx: &mut mlif::Context, sema: &mut mlif::CIRSema) {
    ops::register_ops(ctx);
    sema.add_step(Box::new(transform::TestSemaStep));
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlif::*;

    #[test]
    fn register_test_ops() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        let d = ctx.get_dialect("cir").unwrap();
        assert!(d.get_op("cir.assert").is_some());
        assert!(d.get_op("cir.test_case").is_some());
    }

    #[test]
    fn register_all_2_ops() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        assert_eq!(ctx.get_dialect("cir").unwrap().ops().len(), 2);
    }

    #[test]
    fn build_assert_op() {
        let mut ctx = Context::new();
        let i1_ty = ctx.integer_type(1);
        let block = ctx.create_block();
        let cond = ctx.block_add_argument(block, i1_ty);

        ops::build_assert(&mut ctx, block, cond, "x must be true", Location::unknown());

        let op_list: Vec<OpId> = ctx.block_ops(block).collect();
        assert!(ctx[op_list[0]].is_a("cir.assert"));
        match ctx[op_list[0]].get_attribute("message") {
            Some(Attribute::String(s)) => assert_eq!(s, "x must be true"),
            _ => panic!("expected message attribute"),
        }
    }

    #[test]
    fn build_test_case_op() {
        let mut ctx = Context::new();
        let block = ctx.create_block();

        let (op, body_region) =
            ops::build_test_case(&mut ctx, block, "addition works", Location::unknown());

        assert!(ctx[op].is_a("cir.test_case"));
        match ctx[op].get_attribute("name") {
            Some(Attribute::String(s)) => assert_eq!(s, "addition works"),
            _ => panic!("expected name attribute"),
        }
        assert_eq!(ctx[body_region].num_blocks(), 1);
    }

    #[test]
    fn test_sema_step_generates_main() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);

        // Build a module with two test_case ops.
        let module = Module::new(&mut ctx, Location::unknown());
        let module_block = module.body_block(&ctx);

        ops::build_test_case(&mut ctx, module_block, "test_a", Location::unknown());
        ops::build_test_case(&mut ctx, module_block, "test_b", Location::unknown());

        // Run the sema pass.
        sema.run(module.op(), &mut ctx).unwrap();

        // Check that @__test_0, @__test_1, and @main were generated.
        let sym_table = SymbolTable::build(&ctx, ctx.block_ops(module_block));
        assert!(sym_table.contains("__test_0"));
        assert!(sym_table.contains("__test_1"));
        assert!(sym_table.contains("main"));
    }
}
