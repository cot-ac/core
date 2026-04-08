//! Lowers CIR memory ops to Cranelift IR instructions.
//!
//! Implements `ConstructLowering` for the memory construct, handling 5 ops:
//! alloca (stack slot), store, load, addr_of (identity), deref (load).
//!
//! Gated behind the `codegen` feature.

#![cfg(feature = "codegen")]

use cranelift_codegen::ir::{self as clir, types, InstBuilder, MemFlags};

use mlif::codegen::lowering_ctx::LoweringCtx;
use mlif::entity::OpId;
use mlif::ir::context::Context;
use mlif::ConstructLowering;

/// Cranelift lowering for the CIR memory construct (5 ops).
pub struct MemoryLowering;

impl ConstructLowering for MemoryLowering {
    fn name(&self) -> &str {
        "memory"
    }

    fn lower_op(&self, op: OpId, lctx: &mut LoweringCtx) -> Result<bool, String> {
        let handled = match lctx.ir[op].name() {
            "cir.alloca" => { lower_alloca(op, lctx)?; true }
            "cir.store" => { lower_store(op, lctx)?; true }
            "cir.load" => { lower_load(op, lctx)?; true }
            "cir.addr_of" => { lower_addr_of(op, lctx)?; true }
            "cir.deref" => { lower_deref(op, lctx)?; true }
            _ => false,
        };
        Ok(handled)
    }

    fn map_type(&self, ctx: &Context, ty: mlif::TypeId) -> Option<clir::Type> {
        match ctx.type_kind(ty) {
            mlif::TypeKind::Extension(ext) if ext.dialect == "cir" => match ext.name.as_str() {
                "ptr" | "ref" => Some(types::I64),
                _ => None,
            },
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Op lowering
// ---------------------------------------------------------------------------

/// Lower `cir.alloca` to a Cranelift stack slot + stack_addr.
fn lower_alloca(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let elem_type = lctx.type_attr(op, "elem_type")?;
    let ptr = lctx.stack_alloc(lctx.type_size(elem_type));
    lctx.set_result(op, ptr);
    Ok(())
}

/// Lower `cir.store %value, %addr` to a Cranelift store.
fn lower_store(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let value = lctx.operand(op, 0)?;
    let addr = lctx.operand(op, 1)?;
    lctx.ins().store(MemFlags::new(), value, addr, 0);
    Ok(())
}

/// Lower `cir.load %addr` to a Cranelift load.
fn lower_load(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let addr = lctx.unary_operand(op)?;
    let cl_type = lctx.result_cranelift_type(op)?;
    let r = lctx.ins().load(cl_type, MemFlags::new(), addr, 0);
    lctx.set_result(op, r);
    Ok(())
}

/// Lower `cir.addr_of %ptr` — identity (both ptr and ref are I64).
fn lower_addr_of(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let addr = lctx.unary_operand(op)?;
    lctx.set_result(op, addr);
    Ok(())
}

/// Lower `cir.deref %ref` — load through the pointer.
fn lower_deref(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let addr = lctx.unary_operand(op)?;
    let cl_type = lctx.result_cranelift_type(op)?;
    let r = lctx.ins().load(cl_type, MemFlags::new(), addr, 0);
    lctx.set_result(op, r);
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops as mem_ops;
    use mlif::ir::builder::Builder;
    use mlif::ir::location::Location;
    use mlif::LoweringRegistry;

    fn memory_registry() -> LoweringRegistry {
        let mut registry = LoweringRegistry::new();
        registry.register(Box::new(cot_arith::lowering::ArithLowering));
        registry.register(Box::new(MemoryLowering));
        registry
    }

    fn build_main_scaffold() -> (mlif::Context, OpId, mlif::BlockId, mlif::TypeId) {
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
        let entry = b.func_entry_block(func_op);

        (ctx, module_op, entry, i32_ty)
    }

    fn run_and_check(ctx: &mlif::Context, module_op: OpId, expected: i32, name: &str) {
        let registry = memory_registry();
        let bytes = mlif::codegen::lower_module(ctx, module_op, Some(&registry))
            .expect(&format!("{}: lowering failed", name));

        let tmp = std::env::temp_dir();
        let obj = tmp.join(format!("mem_test_{}.o", name));
        let exe = tmp.join(format!("mem_test_{}", name));

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
    fn test_alloca_store_load() {
        // alloca i32, store 42, load, return
        let (mut ctx, module_op, entry, i32_ty) = build_main_scaffold();

        let ptr = mem_ops::build_alloca(&mut ctx, entry, i32_ty, Location::unknown());
        let c42 = cot_arith::ops::build_constant_int(&mut ctx, entry, i32_ty, 42, Location::unknown());
        mem_ops::build_store(&mut ctx, entry, c42, ptr, Location::unknown());
        let loaded = mem_ops::build_load(&mut ctx, entry, ptr, i32_ty, Location::unknown());

        Builder::at_end(&mut ctx, entry).build_return(&[loaded], Location::unknown());

        run_and_check(&ctx, module_op, 42, "alloca_store_load");
    }

    #[test]
    fn test_addr_of_deref() {
        // alloca i32, store 42, addr_of -> ref, deref -> value, return
        let (mut ctx, module_op, entry, i32_ty) = build_main_scaffold();

        let ptr = mem_ops::build_alloca(&mut ctx, entry, i32_ty, Location::unknown());
        let c42 = cot_arith::ops::build_constant_int(&mut ctx, entry, i32_ty, 42, Location::unknown());
        mem_ops::build_store(&mut ctx, entry, c42, ptr, Location::unknown());
        let reference = mem_ops::build_addr_of(&mut ctx, entry, ptr, i32_ty, Location::unknown());
        let value = mem_ops::build_deref(&mut ctx, entry, reference, i32_ty, Location::unknown());

        Builder::at_end(&mut ctx, entry).build_return(&[value], Location::unknown());

        run_and_check(&ctx, module_op, 42, "addr_of_deref");
    }
}
