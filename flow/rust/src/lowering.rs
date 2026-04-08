//! Lowers CIR flow ops to Cranelift IR instructions.
//!
//! Implements `ConstructLowering` for the flow construct, handling 4 ops:
//! br (jump), condbr (brif), switch (chain of icmp+brif), trap.
//!
//! Gated behind the `codegen` feature.

#![cfg(feature = "codegen")]

use cranelift_codegen::ir::condcodes::IntCC;
use cranelift_codegen::ir::{InstBuilder, TrapCode};

use mlif::codegen::lowering_ctx::LoweringCtx;
use mlif::entity::{BlockId, EntityRef, OpId};
use mlif::ir::attributes::Attribute;
use mlif::ConstructLowering;

/// Cranelift lowering for the CIR flow construct (4 ops).
pub struct FlowLowering;

impl ConstructLowering for FlowLowering {
    fn name(&self) -> &str {
        "flow"
    }

    fn lower_op(&self, op: OpId, lctx: &mut LoweringCtx) -> Result<bool, String> {
        let handled = match lctx.ir[op].name() {
            "cir.br" => { lower_br(op, lctx)?; true }
            "cir.condbr" => { lower_condbr(op, lctx)?; true }
            "cir.switch" => { lower_switch(op, lctx)?; true }
            "cir.trap" => { lctx.ins().trap(TrapCode::unwrap_user(1)); true }
            _ => false,
        };
        Ok(handled)
    }
}

// ---------------------------------------------------------------------------
// Op lowering
// ---------------------------------------------------------------------------

/// Lower `cir.br ^dest` to Cranelift `jump`.
fn lower_br(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let dest = lctx.block_dest(op, "dest")?;
    lctx.ins().jump(dest, &[]);
    Ok(())
}

/// Lower `cir.condbr %cond, ^true_dest, ^false_dest` to Cranelift `brif`.
fn lower_condbr(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let cond = lctx.unary_operand(op)?;
    let true_dest = lctx.block_dest(op, "true_dest")?;
    let false_dest = lctx.block_dest(op, "false_dest")?;
    lctx.ins().brif(cond, true_dest, &[], false_dest, &[]);
    Ok(())
}

/// Lower `cir.switch` to a chain of icmp + brif comparisons.
fn lower_switch(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let switch_val = lctx.unary_operand(op)?;
    let default_dest = lctx.block_dest(op, "default_dest")?;

    let case_values = lctx.array_attr(op, "case_values")?;
    let case_dests = lctx.array_attr(op, "case_dests")?;

    if case_values.len() != case_dests.len() {
        return Err("cir.switch: case_values and case_dests length mismatch".into());
    }

    // Get the Cranelift type of the switch value for iconst.
    let switch_cir_val = lctx.ir[op].operands()[0];
    let cl_type = lctx.cranelift_type(lctx.value_type(switch_cir_val))?;

    if case_values.is_empty() {
        lctx.ins().jump(default_dest, &[]);
        return Ok(());
    }

    // Emit a chain of comparisons.
    for i in 0..case_values.len() {
        let case_val = match &case_values[i] {
            Attribute::Integer { value, .. } => *value,
            _ => return Err("cir.switch: case_values must be integers".into()),
        };
        let case_dest_idx = match &case_dests[i] {
            Attribute::Integer { value, .. } => *value as usize,
            _ => return Err("cir.switch: case_dests must be integers".into()),
        };
        let case_block = lctx.block_dest_raw(BlockId::new(case_dest_idx))?;

        let case_const = lctx.ins().iconst(cl_type, case_val);
        let cmp = lctx.ins().icmp(IntCC::Equal, switch_val, case_const);

        if i < case_values.len() - 1 {
            let next_check = lctx.create_block();
            lctx.ins().brif(cmp, case_block, &[], next_check, &[]);
            lctx.switch_to_block(next_check);
        } else {
            lctx.ins().brif(cmp, case_block, &[], default_dest, &[]);
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops as flow_ops;
    use mlif::ir::builder::Builder;
    use mlif::ir::location::Location;
    use mlif::LoweringRegistry;

    fn flow_registry() -> LoweringRegistry {
        let mut registry = LoweringRegistry::new();
        registry.register(Box::new(cot_arith::lowering::ArithLowering));
        registry.register(Box::new(FlowLowering));
        registry
    }

    /// Build a module with main() -> i32, returning (ctx, module_op, func_op, i32_ty).
    fn build_main_scaffold() -> (mlif::Context, OpId, OpId, mlif::TypeId) {
        let mut ctx = mlif::Context::new();
        let i32_ty = ctx.integer_type(32);
        let fn_ty = ctx.function_type(&[], &[i32_ty]);

        let module_block = ctx.create_block();
        let module_region = ctx.create_region();
        ctx.region_push_block(module_region, module_block);

        let module_op = ctx.create_operation(
            "builtin.module", &[], &[], vec![], vec![module_region], Location::unknown(),
        );

        let mut b = Builder::at_end(&mut ctx, module_block);
        let func_op = b.build_func("main", fn_ty, Location::unknown());

        (ctx, module_op, func_op, i32_ty)
    }

    fn run_and_check(ctx: &mlif::Context, module_op: OpId, expected: i32, name: &str) {
        let registry = flow_registry();
        let bytes = mlif::codegen::lower_module(ctx, module_op, Some(&registry))
            .expect(&format!("{}: lowering failed", name));

        let tmp = std::env::temp_dir();
        let obj = tmp.join(format!("flow_test_{}.o", name));
        let exe = tmp.join(format!("flow_test_{}", name));

        mlif::codegen::write_object_file(&bytes, obj.to_str().unwrap())
            .expect(&format!("{}: write failed", name));
        mlif::codegen::link_executable(obj.to_str().unwrap(), exe.to_str().unwrap())
            .expect(&format!("{}: link failed", name));

        let status = std::process::Command::new(exe.to_str().unwrap())
            .status()
            .expect(&format!("{}: execute failed", name));

        assert_eq!(status.code(), Some(expected), "{}: wrong exit code", name);

        let _ = std::fs::remove_file(&obj);
        let _ = std::fs::remove_file(&exe);
    }

    #[test]
    fn test_br() {
        // entry: br ^target; target: return 42
        let (mut ctx, module_op, func_op, i32_ty) = build_main_scaffold();
        let entry = {
            let b = Builder::at_end(&mut ctx, mlif::BlockId::new(0));
            b.func_entry_block(func_op)
        };

        let target = ctx.create_block();
        let body_region = ctx[func_op].region(0);
        ctx.region_push_block(body_region, target);

        flow_ops::build_br(&mut ctx, entry, target, Location::unknown());

        let c42 = cot_arith::ops::build_constant_int(&mut ctx, target, i32_ty, 42, Location::unknown());
        Builder::at_end(&mut ctx, target).build_return(&[c42], Location::unknown());

        run_and_check(&ctx, module_op, 42, "br");
    }

    #[test]
    fn test_condbr_true() {
        // entry: condbr true, ^then, ^else; then: return 42; else: return 0
        let (mut ctx, module_op, func_op, i32_ty) = build_main_scaffold();
        let entry = {
            let b = Builder::at_end(&mut ctx, mlif::BlockId::new(0));
            b.func_entry_block(func_op)
        };

        let then_block = ctx.create_block();
        let else_block = ctx.create_block();
        let body_region = ctx[func_op].region(0);
        ctx.region_push_block(body_region, then_block);
        ctx.region_push_block(body_region, else_block);

        let cond = cot_arith::ops::build_constant_bool(&mut ctx, entry, true, Location::unknown());
        flow_ops::build_condbr(&mut ctx, entry, cond, then_block, else_block, Location::unknown());

        let c42 = cot_arith::ops::build_constant_int(&mut ctx, then_block, i32_ty, 42, Location::unknown());
        Builder::at_end(&mut ctx, then_block).build_return(&[c42], Location::unknown());

        let c0 = cot_arith::ops::build_constant_int(&mut ctx, else_block, i32_ty, 0, Location::unknown());
        Builder::at_end(&mut ctx, else_block).build_return(&[c0], Location::unknown());

        run_and_check(&ctx, module_op, 42, "condbr_true");
    }

    #[test]
    fn test_condbr_false() {
        let (mut ctx, module_op, func_op, i32_ty) = build_main_scaffold();
        let entry = {
            let b = Builder::at_end(&mut ctx, mlif::BlockId::new(0));
            b.func_entry_block(func_op)
        };

        let then_block = ctx.create_block();
        let else_block = ctx.create_block();
        let body_region = ctx[func_op].region(0);
        ctx.region_push_block(body_region, then_block);
        ctx.region_push_block(body_region, else_block);

        let cond = cot_arith::ops::build_constant_bool(&mut ctx, entry, false, Location::unknown());
        flow_ops::build_condbr(&mut ctx, entry, cond, then_block, else_block, Location::unknown());

        let c1 = cot_arith::ops::build_constant_int(&mut ctx, then_block, i32_ty, 1, Location::unknown());
        Builder::at_end(&mut ctx, then_block).build_return(&[c1], Location::unknown());

        let c7 = cot_arith::ops::build_constant_int(&mut ctx, else_block, i32_ty, 7, Location::unknown());
        Builder::at_end(&mut ctx, else_block).build_return(&[c7], Location::unknown());

        run_and_check(&ctx, module_op, 7, "condbr_false");
    }
}
