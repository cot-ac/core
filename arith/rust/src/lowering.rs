//! Lowers CIR arith ops to Cranelift IR instructions.
//!
//! Implements `ConstructLowering` for the arith construct, handling all 29 ops:
//! constants, binary arithmetic (int + float), unary ops, bitwise, shifts,
//! comparisons, select, and integer/float casts.
//!
//! Gated behind the `codegen` feature.

#![cfg(feature = "codegen")]

use cranelift_codegen::ir::condcodes::{FloatCC, IntCC};
use cranelift_codegen::ir::InstBuilder;

use mlif::codegen::lowering_ctx::LoweringCtx;
use mlif::entity::OpId;
use mlif::ir::attributes::Attribute;
use mlif::ir::types::TypeKind;
use mlif::ConstructLowering;

/// Cranelift lowering for the CIR arith construct (29 ops).
pub struct ArithLowering;

impl ConstructLowering for ArithLowering {
    fn name(&self) -> &str {
        "arith"
    }

    fn lower_op(&self, op: OpId, lctx: &mut LoweringCtx) -> Result<bool, String> {
        let handled = match lctx.ir[op].name() {
            // Constants
            "cir.constant" => { lower_constant(op, lctx)?; true }

            // Binary arithmetic (polymorphic: int + float)
            "cir.add" => { lower_poly_bin(op, lctx, PolyBin::Add)?; true }
            "cir.sub" => { lower_poly_bin(op, lctx, PolyBin::Sub)?; true }
            "cir.mul" => { lower_poly_bin(op, lctx, PolyBin::Mul)?; true }

            // Integer-only binary
            "cir.divsi" => { lower_int_bin(op, lctx, IntBin::Sdiv)?; true }
            "cir.divui" => { lower_int_bin(op, lctx, IntBin::Udiv)?; true }
            "cir.remsi" => { lower_int_bin(op, lctx, IntBin::Srem)?; true }
            "cir.remui" => { lower_int_bin(op, lctx, IntBin::Urem)?; true }

            // Float-only binary
            "cir.divf" => { lower_float_bin(op, lctx, FloatBin::Fdiv)?; true }
            "cir.remf" => { lower_remf(op, lctx)?; true }

            // Unary
            "cir.neg" => { lower_unary(op, lctx, UnaryOp::Ineg)?; true }
            "cir.negf" => { lower_unary(op, lctx, UnaryOp::Fneg)?; true }

            // Bitwise
            "cir.bit_and" => { lower_int_bin(op, lctx, IntBin::Band)?; true }
            "cir.bit_or" => { lower_int_bin(op, lctx, IntBin::Bor)?; true }
            "cir.bit_xor" => { lower_int_bin(op, lctx, IntBin::Bxor)?; true }
            "cir.bit_not" => { lower_unary(op, lctx, UnaryOp::Bnot)?; true }

            // Shifts
            "cir.shl" => { lower_int_bin(op, lctx, IntBin::Ishl)?; true }
            "cir.shr" => { lower_int_bin(op, lctx, IntBin::Ushr)?; true }
            "cir.shr_s" => { lower_int_bin(op, lctx, IntBin::Sshr)?; true }

            // Comparison
            "cir.cmp" => { lower_cmp(op, lctx)?; true }
            "cir.cmpf" => { lower_cmpf(op, lctx)?; true }
            "cir.select" => { lower_select(op, lctx)?; true }

            // Integer casts
            "cir.extsi" => { lower_cast(op, lctx, Cast::Sextend)?; true }
            "cir.extui" => { lower_cast(op, lctx, Cast::Uextend)?; true }
            "cir.trunci" => { lower_cast(op, lctx, Cast::Ireduce)?; true }

            // Float casts
            "cir.sitofp" => { lower_cast(op, lctx, Cast::FcvtFromSint)?; true }
            "cir.fptosi" => { lower_cast(op, lctx, Cast::FcvtToSintSat)?; true }
            "cir.extf" => { lower_cast(op, lctx, Cast::Fpromote)?; true }
            "cir.truncf" => { lower_cast(op, lctx, Cast::Fdemote)?; true }

            _ => false,
        };
        Ok(handled)
    }
}

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

fn lower_constant(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    match lctx.ir[op].get_attribute("value") {
        Some(Attribute::Integer { value, .. }) => {
            let cl_type = lctx.result_cranelift_type(op)?;
            let r = lctx.ins().iconst(cl_type, *value);
            lctx.set_result(op, r);
        }
        Some(Attribute::Float { value, .. }) => {
            let rt = lctx.result_type(op);
            match lctx.ir.type_kind(rt) {
                TypeKind::Float { width: 32 } => {
                    let r = lctx.ins().f32const(*value as f32);
                    lctx.set_result(op, r);
                }
                TypeKind::Float { width: 64 } => {
                    let r = lctx.ins().f64const(*value);
                    lctx.set_result(op, r);
                }
                _ => return Err("cir.constant: unsupported float width".into()),
            }
        }
        _ => return Err("cir.constant: missing or unsupported 'value' attribute".into()),
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Binary arithmetic — polymorphic (int or float based on result type)
// ---------------------------------------------------------------------------

enum PolyBin { Add, Sub, Mul }

fn lower_poly_bin(op: OpId, lctx: &mut LoweringCtx, bin_op: PolyBin) -> Result<(), String> {
    let (lhs, rhs) = lctx.binary_operands(op)?;
    let is_float = lctx.result_is_float(op);

    let r = match (bin_op, is_float) {
        (PolyBin::Add, false) => lctx.ins().iadd(lhs, rhs),
        (PolyBin::Add, true)  => lctx.ins().fadd(lhs, rhs),
        (PolyBin::Sub, false) => lctx.ins().isub(lhs, rhs),
        (PolyBin::Sub, true)  => lctx.ins().fsub(lhs, rhs),
        (PolyBin::Mul, false) => lctx.ins().imul(lhs, rhs),
        (PolyBin::Mul, true)  => lctx.ins().fmul(lhs, rhs),
    };

    lctx.set_result(op, r);
    Ok(())
}

// ---------------------------------------------------------------------------
// Binary integer ops
// ---------------------------------------------------------------------------

enum IntBin { Sdiv, Udiv, Srem, Urem, Band, Bor, Bxor, Ishl, Ushr, Sshr }

fn lower_int_bin(op: OpId, lctx: &mut LoweringCtx, bin_op: IntBin) -> Result<(), String> {
    let (lhs, rhs) = lctx.binary_operands(op)?;

    let r = match bin_op {
        IntBin::Sdiv => lctx.ins().sdiv(lhs, rhs),
        IntBin::Udiv => lctx.ins().udiv(lhs, rhs),
        IntBin::Srem => lctx.ins().srem(lhs, rhs),
        IntBin::Urem => lctx.ins().urem(lhs, rhs),
        IntBin::Band => lctx.ins().band(lhs, rhs),
        IntBin::Bor  => lctx.ins().bor(lhs, rhs),
        IntBin::Bxor => lctx.ins().bxor(lhs, rhs),
        IntBin::Ishl => lctx.ins().ishl(lhs, rhs),
        IntBin::Ushr => lctx.ins().ushr(lhs, rhs),
        IntBin::Sshr => lctx.ins().sshr(lhs, rhs),
    };

    lctx.set_result(op, r);
    Ok(())
}

// ---------------------------------------------------------------------------
// Binary float ops
// ---------------------------------------------------------------------------

enum FloatBin { Fdiv }

fn lower_float_bin(op: OpId, lctx: &mut LoweringCtx, bin_op: FloatBin) -> Result<(), String> {
    let (lhs, rhs) = lctx.binary_operands(op)?;

    let r = match bin_op {
        FloatBin::Fdiv => lctx.ins().fdiv(lhs, rhs),
    };

    lctx.set_result(op, r);
    Ok(())
}

/// Lower cir.remf: a - trunc(a / b) * b (no native Cranelift frem).
fn lower_remf(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let (lhs, rhs) = lctx.binary_operands(op)?;

    let div = lctx.ins().fdiv(lhs, rhs);
    let trunc = lctx.ins().trunc(div);
    let mul = lctx.ins().fmul(trunc, rhs);
    let r = lctx.ins().fsub(lhs, mul);

    lctx.set_result(op, r);
    Ok(())
}

// ---------------------------------------------------------------------------
// Unary ops
// ---------------------------------------------------------------------------

enum UnaryOp { Ineg, Fneg, Bnot }

fn lower_unary(op: OpId, lctx: &mut LoweringCtx, unary_op: UnaryOp) -> Result<(), String> {
    let input = lctx.unary_operand(op)?;

    let r = match unary_op {
        UnaryOp::Ineg => lctx.ins().ineg(input),
        UnaryOp::Fneg => lctx.ins().fneg(input),
        UnaryOp::Bnot => lctx.ins().bnot(input),
    };

    lctx.set_result(op, r);
    Ok(())
}

// ---------------------------------------------------------------------------
// Comparison
// ---------------------------------------------------------------------------

fn map_int_predicate(pred: i64) -> Result<IntCC, String> {
    match pred {
        0 => Ok(IntCC::Equal),
        1 => Ok(IntCC::NotEqual),
        2 => Ok(IntCC::SignedLessThan),
        3 => Ok(IntCC::SignedLessThanOrEqual),
        4 => Ok(IntCC::SignedGreaterThan),
        5 => Ok(IntCC::SignedGreaterThanOrEqual),
        6 => Ok(IntCC::UnsignedLessThan),
        7 => Ok(IntCC::UnsignedLessThanOrEqual),
        8 => Ok(IntCC::UnsignedGreaterThan),
        9 => Ok(IntCC::UnsignedGreaterThanOrEqual),
        _ => Err(format!("cir.cmp: unknown predicate {}", pred)),
    }
}

fn map_float_predicate(pred: i64) -> Result<FloatCC, String> {
    match pred {
        0 => Ok(FloatCC::Equal),
        1 => Ok(FloatCC::GreaterThan),
        2 => Ok(FloatCC::GreaterThanOrEqual),
        3 => Ok(FloatCC::LessThan),
        4 => Ok(FloatCC::LessThanOrEqual),
        5 => Ok(FloatCC::OrderedNotEqual),
        6 => Ok(FloatCC::Ordered),
        7 => Ok(FloatCC::UnorderedOrEqual),
        8 => Ok(FloatCC::UnorderedOrGreaterThan),
        9 => Ok(FloatCC::UnorderedOrGreaterThanOrEqual),
        10 => Ok(FloatCC::UnorderedOrLessThan),
        11 => Ok(FloatCC::UnorderedOrLessThanOrEqual),
        12 => Ok(FloatCC::NotEqual),
        13 => Ok(FloatCC::Unordered),
        _ => Err(format!("cir.cmpf: unknown predicate {}", pred)),
    }
}

fn lower_cmp(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let pred = lctx.int_attr(op, "predicate")?;
    let cc = map_int_predicate(pred)?;
    let (lhs, rhs) = lctx.binary_operands(op)?;
    let r = lctx.ins().icmp(cc, lhs, rhs);
    lctx.set_result(op, r);
    Ok(())
}

fn lower_cmpf(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let pred = lctx.int_attr(op, "predicate")?;
    let cc = map_float_predicate(pred)?;
    let (lhs, rhs) = lctx.binary_operands(op)?;
    let r = lctx.ins().fcmp(cc, lhs, rhs);
    lctx.set_result(op, r);
    Ok(())
}

fn lower_select(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let cond = lctx.operand(op, 0)?;
    let true_val = lctx.operand(op, 1)?;
    let false_val = lctx.operand(op, 2)?;
    let r = lctx.ins().select(cond, true_val, false_val);
    lctx.set_result(op, r);
    Ok(())
}

// ---------------------------------------------------------------------------
// Casts
// ---------------------------------------------------------------------------

enum Cast { Sextend, Uextend, Ireduce, FcvtFromSint, FcvtToSintSat, Fpromote, Fdemote }

fn lower_cast(op: OpId, lctx: &mut LoweringCtx, cast_op: Cast) -> Result<(), String> {
    let input = lctx.unary_operand(op)?;
    let result_ty = lctx.result_cranelift_type(op)?;

    let r = match cast_op {
        Cast::Sextend       => lctx.ins().sextend(result_ty, input),
        Cast::Uextend       => lctx.ins().uextend(result_ty, input),
        Cast::Ireduce       => lctx.ins().ireduce(result_ty, input),
        Cast::FcvtFromSint  => lctx.ins().fcvt_from_sint(result_ty, input),
        Cast::FcvtToSintSat => lctx.ins().fcvt_to_sint_sat(result_ty, input),
        Cast::Fpromote      => lctx.ins().fpromote(result_ty, input),
        Cast::Fdemote       => lctx.ins().fdemote(result_ty, input),
    };

    lctx.set_result(op, r);
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops;
    use mlif::ir::attributes::NamedAttribute;
    use mlif::ir::builder::Builder;
    use mlif::ir::location::Location;
    use mlif::LoweringRegistry;

    /// Helper: build a LoweringRegistry with ArithLowering registered.
    fn arith_registry() -> LoweringRegistry {
        let mut registry = LoweringRegistry::new();
        registry.register(Box::new(ArithLowering));
        registry
    }

    /// Helper: build a CIR module with a single main() -> i32 function.
    /// Returns (ctx, module_op, entry_block, i32_type).
    fn build_main_scaffold() -> (mlif::Context, OpId, mlif::BlockId, mlif::TypeId) {
        let mut ctx = mlif::Context::new();
        let i32_ty = ctx.integer_type(32);
        let fn_ty = ctx.function_type(&[], &[i32_ty]);

        let module_block = ctx.create_block();
        let module_region = ctx.create_region();
        ctx.region_push_block(module_region, module_block);

        let module_op = ctx.create_operation(
            "builtin.module",
            &[],
            &[],
            vec![],
            vec![module_region],
            Location::unknown(),
        );

        let mut b = Builder::at_end(&mut ctx, module_block);
        let func_op = b.build_func("main", fn_ty, Location::unknown());
        let entry = b.func_entry_block(func_op);

        (ctx, module_op, entry, i32_ty)
    }

    /// Helper: lower, write, link, execute, check exit code.
    fn run_and_check(ctx: &mlif::Context, module_op: OpId, expected_exit: i32, test_name: &str) {
        let registry = arith_registry();
        let bytes = mlif::codegen::lower_module(ctx, module_op, Some(&registry))
            .expect(&format!("{}: lowering failed", test_name));

        let tmp = std::env::temp_dir();
        let obj = tmp.join(format!("arith_test_{}.o", test_name));
        let exe = tmp.join(format!("arith_test_{}", test_name));

        mlif::codegen::write_object_file(&bytes, obj.to_str().unwrap())
            .expect(&format!("{}: write failed", test_name));
        mlif::codegen::link_executable(obj.to_str().unwrap(), exe.to_str().unwrap())
            .expect(&format!("{}: link failed", test_name));

        let status = std::process::Command::new(exe.to_str().unwrap())
            .status()
            .expect(&format!("{}: execute failed", test_name));

        assert_eq!(
            status.code(),
            Some(expected_exit),
            "{}: expected exit code {}, got {:?}",
            test_name,
            expected_exit,
            status.code()
        );

        let _ = std::fs::remove_file(&obj);
        let _ = std::fs::remove_file(&exe);
    }

    // --- End-to-end tests ---

    #[test]
    fn test_constant_return() {
        let (mut ctx, module_op, entry, i32_ty) = build_main_scaffold();
        let mut b = Builder::at_end(&mut ctx, entry);
        let c = b.create_op_full(
            "cir.constant",
            &[],
            &[i32_ty],
            vec![NamedAttribute::new(
                "value",
                Attribute::Integer {
                    value: 42,
                    ty: i32_ty,
                },
            )],
            vec![],
            Location::unknown(),
        );
        let v = b.op_result(c, 0);
        b.build_return(&[v], Location::unknown());

        run_and_check(&ctx, module_op, 42, "const_ret");
    }

    #[test]
    fn test_sdiv() {
        let (mut ctx, module_op, entry, i32_ty) = build_main_scaffold();

        let c84 = ops::build_constant_int(&mut ctx, entry, i32_ty, 84, Location::unknown());
        let c2 = ops::build_constant_int(&mut ctx, entry, i32_ty, 2, Location::unknown());
        let div = ops::build_divsi(&mut ctx, entry, c84, c2, Location::unknown());

        Builder::at_end(&mut ctx, entry).build_return(&[div], Location::unknown());

        run_and_check(&ctx, module_op, 42, "sdiv");
    }

    #[test]
    fn test_cmp_select() {
        // cmp(20, 10, eq) -> false; select(false, 1, 42) -> 42
        let (mut ctx, module_op, entry, i32_ty) = build_main_scaffold();

        let c20 = ops::build_constant_int(&mut ctx, entry, i32_ty, 20, Location::unknown());
        let c10 = ops::build_constant_int(&mut ctx, entry, i32_ty, 10, Location::unknown());
        let cmp = ops::build_cmp(
            &mut ctx,
            entry,
            ops::IntPredicate::Eq,
            c20,
            c10,
            Location::unknown(),
        );

        let c1 = ops::build_constant_int(&mut ctx, entry, i32_ty, 1, Location::unknown());
        let c42 = ops::build_constant_int(&mut ctx, entry, i32_ty, 42, Location::unknown());
        let sel = ops::build_select(&mut ctx, entry, cmp, c1, c42, Location::unknown());

        Builder::at_end(&mut ctx, entry).build_return(&[sel], Location::unknown());

        run_and_check(&ctx, module_op, 42, "cmp_select");
    }

    #[test]
    fn test_sextend() {
        // i8 constant 42, sign-extend to i32, return
        let (mut ctx, module_op, entry, i32_ty) = build_main_scaffold();

        let i8_ty = ctx.integer_type(8);
        let c42 = ops::build_constant_int(&mut ctx, entry, i8_ty, 42, Location::unknown());
        let ext = ops::build_extsi(&mut ctx, entry, c42, i32_ty, Location::unknown());

        Builder::at_end(&mut ctx, entry).build_return(&[ext], Location::unknown());

        run_and_check(&ctx, module_op, 42, "sextend");
    }

    #[test]
    fn test_bitwise_and() {
        // 0xFF & 0x2A = 0x2A = 42
        let (mut ctx, module_op, entry, i32_ty) = build_main_scaffold();

        let cff = ops::build_constant_int(&mut ctx, entry, i32_ty, 0xFF, Location::unknown());
        let c2a = ops::build_constant_int(&mut ctx, entry, i32_ty, 0x2A, Location::unknown());
        let result = ops::build_bit_and(&mut ctx, entry, cff, c2a, Location::unknown());

        Builder::at_end(&mut ctx, entry).build_return(&[result], Location::unknown());

        run_and_check(&ctx, module_op, 42, "band");
    }

    #[test]
    fn test_neg() {
        // neg(-42) = 42
        let (mut ctx, module_op, entry, i32_ty) = build_main_scaffold();

        let cn42 = ops::build_constant_int(&mut ctx, entry, i32_ty, -42, Location::unknown());
        let result = ops::build_neg(&mut ctx, entry, cn42, Location::unknown());

        Builder::at_end(&mut ctx, entry).build_return(&[result], Location::unknown());

        run_and_check(&ctx, module_op, 42, "neg");
    }

    #[test]
    fn test_shift_left() {
        // 21 << 1 = 42
        let (mut ctx, module_op, entry, i32_ty) = build_main_scaffold();

        let c21 = ops::build_constant_int(&mut ctx, entry, i32_ty, 21, Location::unknown());
        let c1 = ops::build_constant_int(&mut ctx, entry, i32_ty, 1, Location::unknown());
        let result = ops::build_shl(&mut ctx, entry, c21, c1, Location::unknown());

        Builder::at_end(&mut ctx, entry).build_return(&[result], Location::unknown());

        run_and_check(&ctx, module_op, 42, "shl");
    }
}
