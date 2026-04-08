//! # cot-arith
//!
//! The arithmetic construct for CIR. Provides 29 operations covering integer
//! and floating-point arithmetic, constants, comparisons, bitwise operations,
//! and sign/width casts. Operates entirely on MLIF primitive types (i1, i8,
//! i16, i32, i64, f32, f64) — no custom types are defined.
//!
//! ## Registration
//!
//! Call [`register`] to register all 29 ops with the CIR dialect and add
//! the [`ArithSemaStep`](transform::ArithSemaStep) to the sema pipeline.

pub mod lowering;
pub mod ops;
pub mod transform;

/// Register the arith construct's operations and sema step.
///
/// - Registers 29 ops under the `cir` dialect namespace.
/// - Adds `ArithSemaStep` at `StepPosition::Types` for call-site cast insertion.
pub fn register(ctx: &mut mlif::Context, sema: &mut mlif::CIRSema) {
    ops::register_ops(ctx);
    sema.add_step(Box::new(transform::ArithSemaStep));
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlif::*;

    #[test]
    fn register_arith_ops() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        let dialect = ctx.get_dialect("cir").unwrap();
        assert!(dialect.get_op("cir.add").is_some());
        assert!(dialect.get_op("cir.constant").is_some());
        assert!(dialect.get_op("cir.cmp").is_some());
        assert!(dialect.get_op("cir.truncf").is_some());
    }

    #[test]
    fn register_all_29_ops() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        let dialect = ctx.get_dialect("cir").unwrap();

        let expected_ops = [
            "cir.constant",
            "cir.add",
            "cir.sub",
            "cir.mul",
            "cir.divsi",
            "cir.divui",
            "cir.divf",
            "cir.remsi",
            "cir.remui",
            "cir.remf",
            "cir.neg",
            "cir.negf",
            "cir.bit_and",
            "cir.bit_or",
            "cir.bit_xor",
            "cir.bit_not",
            "cir.shl",
            "cir.shr",
            "cir.shr_s",
            "cir.cmp",
            "cir.cmpf",
            "cir.select",
            "cir.extsi",
            "cir.extui",
            "cir.trunci",
            "cir.sitofp",
            "cir.fptosi",
            "cir.extf",
            "cir.truncf",
        ];
        assert_eq!(dialect.ops().len(), 29);
        for name in &expected_ops {
            assert!(
                dialect.get_op(name).is_some(),
                "missing op: {}",
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

        // add is Pure + Commutative + SameOperandsAndResultType
        let add = dialect.get_op("cir.add").unwrap();
        assert!(add.has_trait(&OpTrait::Pure));
        assert!(add.has_trait(&OpTrait::Commutative));
        assert!(add.has_trait(&OpTrait::SameOperandsAndResultType));

        // sub is Pure + SameOperandsAndResultType but NOT Commutative
        let sub = dialect.get_op("cir.sub").unwrap();
        assert!(sub.has_trait(&OpTrait::Pure));
        assert!(!sub.has_trait(&OpTrait::Commutative));
        assert!(sub.has_trait(&OpTrait::SameOperandsAndResultType));

        // cmp is Pure only (result type differs from operands)
        let cmp = dialect.get_op("cir.cmp").unwrap();
        assert!(cmp.has_trait(&OpTrait::Pure));
        assert!(!cmp.has_trait(&OpTrait::SameOperandsAndResultType));

        // extsi is Pure only (cast — result type differs from input)
        let extsi = dialect.get_op("cir.extsi").unwrap();
        assert!(extsi.has_trait(&OpTrait::Pure));
        assert!(!extsi.has_trait(&OpTrait::SameOperandsAndResultType));
    }

    #[test]
    fn build_add_op() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);

        let i32_ty = ctx.integer_type(32);
        let block = ctx.create_block();
        let a = ctx.block_add_argument(block, i32_ty);
        let b = ctx.block_add_argument(block, i32_ty);

        let sum = ops::build_add(&mut ctx, block, a, b, Location::unknown());
        assert_eq!(ctx.value_type(sum), i32_ty);

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(block_ops.len(), 1);
        assert!(ctx[block_ops[0]].is_a("cir.add"));
    }

    #[test]
    fn build_constant() {
        let mut ctx = Context::new();
        let i32_ty = ctx.integer_type(32);
        let block = ctx.create_block();

        let val = ops::build_constant_int(&mut ctx, block, i32_ty, 42, Location::unknown());
        assert_eq!(ctx.value_type(val), i32_ty);

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert!(ctx[block_ops[0]].is_a("cir.constant"));
        match ctx[block_ops[0]].get_attribute("value") {
            Some(Attribute::Integer { value, .. }) => assert_eq!(*value, 42),
            _ => panic!("expected integer attribute"),
        }
    }

    #[test]
    fn build_constant_float() {
        let mut ctx = Context::new();
        let f64_ty = ctx.float_type(64);
        let block = ctx.create_block();

        let val = ops::build_constant_float(&mut ctx, block, f64_ty, 3.14, Location::unknown());
        assert_eq!(ctx.value_type(val), f64_ty);

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert!(ctx[block_ops[0]].is_a("cir.constant"));
        match ctx[block_ops[0]].get_attribute("value") {
            Some(Attribute::Float { value, .. }) => assert!((value - 3.14).abs() < 1e-10),
            _ => panic!("expected float attribute"),
        }
    }

    #[test]
    fn build_constant_bool() {
        let mut ctx = Context::new();
        let block = ctx.create_block();

        let t = ops::build_constant_bool(&mut ctx, block, true, Location::unknown());
        let f = ops::build_constant_bool(&mut ctx, block, false, Location::unknown());

        let i1_ty = ctx.integer_type(1);
        assert_eq!(ctx.value_type(t), i1_ty);
        assert_eq!(ctx.value_type(f), i1_ty);

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(block_ops.len(), 2);

        match ctx[block_ops[0]].get_attribute("value") {
            Some(Attribute::Integer { value, .. }) => assert_eq!(*value, 1),
            _ => panic!("expected integer attribute for true"),
        }
        match ctx[block_ops[1]].get_attribute("value") {
            Some(Attribute::Integer { value, .. }) => assert_eq!(*value, 0),
            _ => panic!("expected integer attribute for false"),
        }
    }

    #[test]
    fn build_comparison() {
        let mut ctx = Context::new();
        let i32_ty = ctx.integer_type(32);
        let i1_ty = ctx.integer_type(1);
        let block = ctx.create_block();
        let a = ctx.block_add_argument(block, i32_ty);
        let b = ctx.block_add_argument(block, i32_ty);

        let result =
            ops::build_cmp(&mut ctx, block, ops::IntPredicate::Eq, a, b, Location::unknown());
        assert_eq!(ctx.value_type(result), i1_ty);

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(block_ops.len(), 1);
        assert!(ctx[block_ops[0]].is_a("cir.cmp"));

        // Verify predicate attribute
        match ctx[block_ops[0]].get_attribute("predicate") {
            Some(Attribute::Integer { value, .. }) => {
                assert_eq!(*value, ops::IntPredicate::Eq as i64)
            }
            _ => panic!("expected predicate attribute"),
        }
    }

    #[test]
    fn build_float_comparison() {
        let mut ctx = Context::new();
        let f32_ty = ctx.float_type(32);
        let i1_ty = ctx.integer_type(1);
        let block = ctx.create_block();
        let a = ctx.block_add_argument(block, f32_ty);
        let b = ctx.block_add_argument(block, f32_ty);

        let result = ops::build_cmpf(
            &mut ctx,
            block,
            ops::FloatPredicate::Olt,
            a,
            b,
            Location::unknown(),
        );
        assert_eq!(ctx.value_type(result), i1_ty);

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert!(ctx[block_ops[0]].is_a("cir.cmpf"));
        match ctx[block_ops[0]].get_attribute("predicate") {
            Some(Attribute::Integer { value, .. }) => {
                assert_eq!(*value, ops::FloatPredicate::Olt as i64)
            }
            _ => panic!("expected predicate attribute"),
        }
    }

    #[test]
    fn build_select_op() {
        let mut ctx = Context::new();
        let i32_ty = ctx.integer_type(32);
        let i1_ty = ctx.integer_type(1);
        let block = ctx.create_block();
        let cond = ctx.block_add_argument(block, i1_ty);
        let a = ctx.block_add_argument(block, i32_ty);
        let b = ctx.block_add_argument(block, i32_ty);

        let result = ops::build_select(&mut ctx, block, cond, a, b, Location::unknown());
        assert_eq!(ctx.value_type(result), i32_ty);

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(block_ops.len(), 1);
        assert!(ctx[block_ops[0]].is_a("cir.select"));
        assert_eq!(ctx[block_ops[0]].num_operands(), 3);
    }

    #[test]
    fn build_integer_casts() {
        let mut ctx = Context::new();
        let i16_ty = ctx.integer_type(16);
        let i32_ty = ctx.integer_type(32);
        let i64_ty = ctx.integer_type(64);
        let block = ctx.create_block();
        let narrow = ctx.block_add_argument(block, i16_ty);
        let mid = ctx.block_add_argument(block, i32_ty);
        let wide = ctx.block_add_argument(block, i64_ty);

        // Sign-extend i16 -> i32
        let ext = ops::build_extsi(&mut ctx, block, narrow, i32_ty, Location::unknown());
        assert_eq!(ctx.value_type(ext), i32_ty);

        // Zero-extend i32 -> i64
        let zext = ops::build_extui(&mut ctx, block, mid, i64_ty, Location::unknown());
        assert_eq!(ctx.value_type(zext), i64_ty);

        // Truncate i64 -> i16
        let trunc = ops::build_trunci(&mut ctx, block, wide, i16_ty, Location::unknown());
        assert_eq!(ctx.value_type(trunc), i16_ty);

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(block_ops.len(), 3);
        assert!(ctx[block_ops[0]].is_a("cir.extsi"));
        assert!(ctx[block_ops[1]].is_a("cir.extui"));
        assert!(ctx[block_ops[2]].is_a("cir.trunci"));
    }

    #[test]
    fn build_float_casts() {
        let mut ctx = Context::new();
        let i32_ty = ctx.integer_type(32);
        let f32_ty = ctx.float_type(32);
        let f64_ty = ctx.float_type(64);
        let block = ctx.create_block();
        let int_val = ctx.block_add_argument(block, i32_ty);
        let f32_val = ctx.block_add_argument(block, f32_ty);
        let f64_val = ctx.block_add_argument(block, f64_ty);

        // sitofp: i32 -> f64
        let fp = ops::build_sitofp(&mut ctx, block, int_val, f64_ty, Location::unknown());
        assert_eq!(ctx.value_type(fp), f64_ty);

        // fptosi: f64 -> i32
        let si = ops::build_fptosi(&mut ctx, block, f64_val, i32_ty, Location::unknown());
        assert_eq!(ctx.value_type(si), i32_ty);

        // extf: f32 -> f64
        let ext = ops::build_extf(&mut ctx, block, f32_val, f64_ty, Location::unknown());
        assert_eq!(ctx.value_type(ext), f64_ty);

        // truncf: f64 -> f32
        let trunc = ops::build_truncf(&mut ctx, block, f64_val, f32_ty, Location::unknown());
        assert_eq!(ctx.value_type(trunc), f32_ty);

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(block_ops.len(), 4);
        assert!(ctx[block_ops[0]].is_a("cir.sitofp"));
        assert!(ctx[block_ops[1]].is_a("cir.fptosi"));
        assert!(ctx[block_ops[2]].is_a("cir.extf"));
        assert!(ctx[block_ops[3]].is_a("cir.truncf"));
    }

    #[test]
    fn build_binary_ops() {
        let mut ctx = Context::new();
        let i32_ty = ctx.integer_type(32);
        let block = ctx.create_block();
        let a = ctx.block_add_argument(block, i32_ty);
        let b = ctx.block_add_argument(block, i32_ty);

        // Test all binary ops produce the correct type
        let sub = ops::build_sub(&mut ctx, block, a, b, Location::unknown());
        assert_eq!(ctx.value_type(sub), i32_ty);

        let mul = ops::build_mul(&mut ctx, block, a, b, Location::unknown());
        assert_eq!(ctx.value_type(mul), i32_ty);

        let divsi = ops::build_divsi(&mut ctx, block, a, b, Location::unknown());
        assert_eq!(ctx.value_type(divsi), i32_ty);

        let divui = ops::build_divui(&mut ctx, block, a, b, Location::unknown());
        assert_eq!(ctx.value_type(divui), i32_ty);

        let remsi = ops::build_remsi(&mut ctx, block, a, b, Location::unknown());
        assert_eq!(ctx.value_type(remsi), i32_ty);

        let remui = ops::build_remui(&mut ctx, block, a, b, Location::unknown());
        assert_eq!(ctx.value_type(remui), i32_ty);

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(block_ops.len(), 6);
        assert!(ctx[block_ops[0]].is_a("cir.sub"));
        assert!(ctx[block_ops[1]].is_a("cir.mul"));
        assert!(ctx[block_ops[2]].is_a("cir.divsi"));
        assert!(ctx[block_ops[3]].is_a("cir.divui"));
        assert!(ctx[block_ops[4]].is_a("cir.remsi"));
        assert!(ctx[block_ops[5]].is_a("cir.remui"));
    }

    #[test]
    fn build_float_binary_ops() {
        let mut ctx = Context::new();
        let f64_ty = ctx.float_type(64);
        let block = ctx.create_block();
        let a = ctx.block_add_argument(block, f64_ty);
        let b = ctx.block_add_argument(block, f64_ty);

        let divf = ops::build_divf(&mut ctx, block, a, b, Location::unknown());
        assert_eq!(ctx.value_type(divf), f64_ty);

        let remf = ops::build_remf(&mut ctx, block, a, b, Location::unknown());
        assert_eq!(ctx.value_type(remf), f64_ty);

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(block_ops.len(), 2);
        assert!(ctx[block_ops[0]].is_a("cir.divf"));
        assert!(ctx[block_ops[1]].is_a("cir.remf"));
    }

    #[test]
    fn build_unary_ops() {
        let mut ctx = Context::new();
        let i32_ty = ctx.integer_type(32);
        let f64_ty = ctx.float_type(64);
        let block = ctx.create_block();
        let int_val = ctx.block_add_argument(block, i32_ty);
        let float_val = ctx.block_add_argument(block, f64_ty);

        let neg = ops::build_neg(&mut ctx, block, int_val, Location::unknown());
        assert_eq!(ctx.value_type(neg), i32_ty);

        let negf = ops::build_negf(&mut ctx, block, float_val, Location::unknown());
        assert_eq!(ctx.value_type(negf), f64_ty);

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(block_ops.len(), 2);
        assert!(ctx[block_ops[0]].is_a("cir.neg"));
        assert!(ctx[block_ops[1]].is_a("cir.negf"));
    }

    #[test]
    fn build_bitwise_ops() {
        let mut ctx = Context::new();
        let i32_ty = ctx.integer_type(32);
        let block = ctx.create_block();
        let a = ctx.block_add_argument(block, i32_ty);
        let b = ctx.block_add_argument(block, i32_ty);

        let and = ops::build_bit_and(&mut ctx, block, a, b, Location::unknown());
        assert_eq!(ctx.value_type(and), i32_ty);

        let or = ops::build_bit_or(&mut ctx, block, a, b, Location::unknown());
        assert_eq!(ctx.value_type(or), i32_ty);

        let xor = ops::build_bit_xor(&mut ctx, block, a, b, Location::unknown());
        assert_eq!(ctx.value_type(xor), i32_ty);

        let not = ops::build_bit_not(&mut ctx, block, a, Location::unknown());
        assert_eq!(ctx.value_type(not), i32_ty);

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(block_ops.len(), 4);
        assert!(ctx[block_ops[0]].is_a("cir.bit_and"));
        assert!(ctx[block_ops[1]].is_a("cir.bit_or"));
        assert!(ctx[block_ops[2]].is_a("cir.bit_xor"));
        assert!(ctx[block_ops[3]].is_a("cir.bit_not"));
    }

    #[test]
    fn build_shift_ops() {
        let mut ctx = Context::new();
        let i32_ty = ctx.integer_type(32);
        let block = ctx.create_block();
        let a = ctx.block_add_argument(block, i32_ty);
        let b = ctx.block_add_argument(block, i32_ty);

        let shl = ops::build_shl(&mut ctx, block, a, b, Location::unknown());
        assert_eq!(ctx.value_type(shl), i32_ty);

        let shr = ops::build_shr(&mut ctx, block, a, b, Location::unknown());
        assert_eq!(ctx.value_type(shr), i32_ty);

        let shr_s = ops::build_shr_s(&mut ctx, block, a, b, Location::unknown());
        assert_eq!(ctx.value_type(shr_s), i32_ty);

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(block_ops.len(), 3);
        assert!(ctx[block_ops[0]].is_a("cir.shl"));
        assert!(ctx[block_ops[1]].is_a("cir.shr"));
        assert!(ctx[block_ops[2]].is_a("cir.shr_s"));
    }
}
