//! # cot-flow
//!
//! The flow construct for CIR. Provides four terminator operations for
//! control flow: unconditional branch, conditional branch, multi-way switch,
//! and trap (unreachable/abort). No custom types -- all operate on MLIF
//! primitives and block references.
//!
//! ## Operations
//!
//! - `cir.br` -- unconditional branch (Terminator + Pure)
//! - `cir.condbr` -- conditional branch on i1 (Terminator + Pure)
//! - `cir.switch` -- multi-way branch on integer (Terminator + Pure)
//! - `cir.trap` -- abort / unreachable (Terminator only)
//!
//! ## Registration
//!
//! Call [`register`] to register all 4 ops with the CIR dialect.

pub mod lowering;
pub mod ops;

/// Register the flow construct's operations.
///
/// - Registers 4 ops under the `cir` dialect namespace.
/// - All ops have the `Terminator` trait.
pub fn register(ctx: &mut mlif::Context, _sema: &mut mlif::CIRSema) {
    ops::register_ops(ctx);
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlif::*;

    #[test]
    fn register_flow_ops() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        let dialect = ctx.get_dialect("cir").unwrap();
        assert!(dialect.get_op("cir.br").is_some());
        assert!(dialect.get_op("cir.condbr").is_some());
        assert!(dialect.get_op("cir.switch").is_some());
        assert!(dialect.get_op("cir.trap").is_some());
    }

    #[test]
    fn register_all_4_ops() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        let dialect = ctx.get_dialect("cir").unwrap();

        let expected_ops = ["cir.br", "cir.condbr", "cir.switch", "cir.trap"];
        assert_eq!(dialect.ops().len(), 4);
        for name in &expected_ops {
            assert!(
                dialect.get_op(name).is_some(),
                "missing op: {}",
                name
            );
        }
    }

    #[test]
    fn all_ops_are_terminators() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        let dialect = ctx.get_dialect("cir").unwrap();

        for name in &["cir.br", "cir.condbr", "cir.switch", "cir.trap"] {
            let op = dialect.get_op(name).unwrap();
            assert!(
                op.has_trait(&OpTrait::Terminator),
                "{} should be a Terminator",
                name
            );
            assert!(
                op.is_terminator(),
                "{} should return true for is_terminator()",
                name
            );
        }
    }

    #[test]
    fn op_traits_correct() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        let dialect = ctx.get_dialect("cir").unwrap();

        // br is Terminator + Pure
        let br = dialect.get_op("cir.br").unwrap();
        assert!(br.has_trait(&OpTrait::Terminator));
        assert!(br.has_trait(&OpTrait::Pure));

        // condbr is Terminator + Pure
        let condbr = dialect.get_op("cir.condbr").unwrap();
        assert!(condbr.has_trait(&OpTrait::Terminator));
        assert!(condbr.has_trait(&OpTrait::Pure));

        // switch is Terminator + Pure
        let switch = dialect.get_op("cir.switch").unwrap();
        assert!(switch.has_trait(&OpTrait::Terminator));
        assert!(switch.has_trait(&OpTrait::Pure));

        // trap is Terminator only (NOT Pure)
        let trap = dialect.get_op("cir.trap").unwrap();
        assert!(trap.has_trait(&OpTrait::Terminator));
        assert!(!trap.has_trait(&OpTrait::Pure));
    }

    #[test]
    fn build_br() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);

        let block = ctx.create_block();
        let dest = ctx.create_block();

        ops::build_br(&mut ctx, block, dest, Location::unknown());

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(block_ops.len(), 1);
        assert!(ctx[block_ops[0]].is_a("cir.br"));

        // Check dest attribute
        match ctx[block_ops[0]].get_attribute("dest") {
            Some(Attribute::Integer { value, .. }) => {
                assert_eq!(*value, dest.index() as i64);
            }
            _ => panic!("expected Integer attribute for dest"),
        }
    }

    #[test]
    fn build_condbr() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);

        let i1_ty = ctx.integer_type(1);
        let block = ctx.create_block();
        let cond = ctx.block_add_argument(block, i1_ty);
        let true_block = ctx.create_block();
        let false_block = ctx.create_block();

        ops::build_condbr(
            &mut ctx,
            block,
            cond,
            true_block,
            false_block,
            Location::unknown(),
        );

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(block_ops.len(), 1);
        assert!(ctx[block_ops[0]].is_a("cir.condbr"));
        assert_eq!(ctx[block_ops[0]].num_operands(), 1);

        // Check true_dest attribute
        match ctx[block_ops[0]].get_attribute("true_dest") {
            Some(Attribute::Integer { value, .. }) => {
                assert_eq!(*value, true_block.index() as i64);
            }
            _ => panic!("expected Integer attribute for true_dest"),
        }

        // Check false_dest attribute
        match ctx[block_ops[0]].get_attribute("false_dest") {
            Some(Attribute::Integer { value, .. }) => {
                assert_eq!(*value, false_block.index() as i64);
            }
            _ => panic!("expected Integer attribute for false_dest"),
        }
    }

    #[test]
    fn build_switch() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);

        let i32_ty = ctx.integer_type(32);
        let block = ctx.create_block();
        let value = ctx.block_add_argument(block, i32_ty);
        let default_block = ctx.create_block();
        let case1_block = ctx.create_block();
        let case2_block = ctx.create_block();

        ops::build_switch(
            &mut ctx,
            block,
            value,
            default_block,
            &[(0, case1_block), (1, case2_block)],
            Location::unknown(),
        );

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(block_ops.len(), 1);
        assert!(ctx[block_ops[0]].is_a("cir.switch"));
        assert_eq!(ctx[block_ops[0]].num_operands(), 1);

        // Check default_dest
        match ctx[block_ops[0]].get_attribute("default_dest") {
            Some(Attribute::Integer { value, .. }) => {
                assert_eq!(*value, default_block.index() as i64);
            }
            _ => panic!("expected Integer attribute for default_dest"),
        }

        // Check case_values array
        match ctx[block_ops[0]].get_attribute("case_values") {
            Some(Attribute::Array(values)) => {
                assert_eq!(values.len(), 2);
            }
            _ => panic!("expected Array attribute for case_values"),
        }

        // Check case_dests array
        match ctx[block_ops[0]].get_attribute("case_dests") {
            Some(Attribute::Array(dests)) => {
                assert_eq!(dests.len(), 2);
            }
            _ => panic!("expected Array attribute for case_dests"),
        }
    }

    #[test]
    fn build_trap() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);

        let block = ctx.create_block();
        ops::build_trap(&mut ctx, block, Location::unknown());

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(block_ops.len(), 1);
        assert!(ctx[block_ops[0]].is_a("cir.trap"));
        assert_eq!(ctx[block_ops[0]].num_operands(), 0);
        assert_eq!(ctx[block_ops[0]].num_results(), 0);
    }
}
