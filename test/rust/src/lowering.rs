//! Lowers CIR test ops to Cranelift IR instructions.
//!
//! Mirrors C++ test/Lowering.cpp — 1 pattern:
//! AssertOpLowering — condbr: true → continue, false → call cir_test_fail + trap.
//!
//! test_case ops are handled by the TestSemaStep transform (converted
//! to regular functions before lowering).

#![cfg(feature = "codegen")]

use cranelift_codegen::ir::{types, InstBuilder, TrapCode};

use mlif::codegen::lowering_ctx::LoweringCtx;
use mlif::entity::OpId;
use mlif::ConstructLowering;

/// Global counter for unique assert message names.
static ASSERT_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Cranelift lowering for the CIR test construct (1 lowered op).
pub struct TestLowering;

impl ConstructLowering for TestLowering {
    fn name(&self) -> &str {
        "test"
    }

    fn lower_op(&self, op: OpId, lctx: &mut LoweringCtx) -> Result<bool, String> {
        let handled = match lctx.ir[op].name() {
            "cir.assert" => { lower_assert(op, lctx)?; true }
            _ => false,
        };
        Ok(handled)
    }
}

/// Lower `cir.assert` — brif: true → continue, false → call cir_test_fail + trap.
/// C++ equivalent: creates global string, declares cir_test_fail, splits block,
/// condbr to pass/fail, fail calls runtime then traps.
fn lower_assert(op: OpId, lctx: &mut LoweringCtx) -> Result<(), String> {
    let cond = lctx.unary_operand(op)?;

    let pass_block = lctx.create_block();
    let fail_block = lctx.create_block();

    // Branch: condition true → pass, false → fail.
    lctx.ins().brif(cond, pass_block, &[], fail_block, &[]);

    // Fail block: call cir_test_fail with error message, then trap.
    lctx.switch_to_block(fail_block);
    lctx.seal_block(fail_block);

    // Create error message in data section.
    let msg = lctx.string_attr(op, "message").unwrap_or_else(|_| "assertion failed".to_string());
    let msg_bytes = msg.as_bytes();
    let msg_len = msg_bytes.len() as i64;
    let idx = ASSERT_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let data_name = format!("__assert_msg_{}", idx);

    // Try to create the global data and call the runtime.
    // If define_data fails (e.g., duplicate name), fall through to just trap.
    if let Ok(msg_ptr) = lctx.define_data(&data_name, msg_bytes) {
        let len_val = lctx.ins().iconst(types::I64, msg_len);
        // Declare cir_test_fail(ptr, i64) → void.
        if let Ok(fail_fn) = lctx.declare_import_func("cir_test_fail", &[types::I64, types::I64], &[]) {
            lctx.ins().call(fail_fn, &[msg_ptr, len_val]);
        }
    }

    lctx.ins().trap(TrapCode::unwrap_user(1));

    // Continue in pass block.
    lctx.switch_to_block(pass_block);
    lctx.seal_block(pass_block);

    Ok(())
}
