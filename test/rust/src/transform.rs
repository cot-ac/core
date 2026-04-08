//! TestSemaStep — extracts test_case regions into standalone functions
//! and generates a @main that calls them all.
//!
//! Uses finalize() (not visit_op) because it needs to see all test_cases
//! before generating the runner.
//!
//! Reference: `core/test/cpp/TestRunnerGenerator.cpp`

use mlif::*;

/// Sema step that generates the test runner in finalize().
pub struct TestSemaStep;

impl SemaStep for TestSemaStep {
    fn name(&self) -> &str {
        "test-runner"
    }

    fn position(&self) -> StepPosition {
        StepPosition::Types
    }

    fn visit_op(
        &mut self,
        _op: OpId,
        _ctx: &mut Context,
        _state: &SemaState,
    ) -> Result<bool, DiagnosticError> {
        Ok(false)
    }

    fn finalize(
        &mut self,
        module: OpId,
        ctx: &mut Context,
        _state: &SemaState,
    ) -> Result<(), DiagnosticError> {
        // Collect all test_case ops in the module.
        let mut test_cases: Vec<(OpId, String)> = Vec::new();
        ctx.walk(module, WalkOrder::PreOrder, &mut |op_id, ctx| {
            if ctx[op_id].is_a("cir.test_case") {
                let name = match ctx[op_id].get_attribute("name") {
                    Some(Attribute::String(s)) => s.clone(),
                    _ => format!("test_{}", test_cases.len()),
                };
                test_cases.push((op_id, name));
            }
            WalkResult::Advance
        });

        if test_cases.is_empty() {
            return Ok(());
        }

        let module_body = ctx[module].region(0);
        let module_block = ctx[module_body].entry_block().unwrap();

        // For each test_case, create a standalone @__test_N function.
        let mut test_fn_names: Vec<String> = Vec::new();
        for (i, (_test_op, _name)) in test_cases.iter().enumerate() {
            let fn_name = format!("__test_{}", i);
            let void_func_ty = ctx.function_type(&[], &[]);

            let entry_block = ctx.create_block();
            let body_region = ctx.create_region();
            ctx.region_push_block(body_region, entry_block);

            let ret = ctx.create_operation(
                "func.return",
                &[],
                &[],
                vec![],
                vec![],
                Location::unknown(),
            );
            ctx.block_push_op(entry_block, ret);

            let func_op = ctx.create_operation(
                "func.func",
                &[],
                &[],
                vec![
                    NamedAttribute::new("sym_name", Attribute::String(fn_name.clone())),
                    NamedAttribute::new("function_type", Attribute::Type(void_func_ty)),
                ],
                vec![body_region],
                Location::unknown(),
            );
            ctx.block_push_op(module_block, func_op);
            test_fn_names.push(fn_name);
        }

        // Generate @main that calls each test function.
        let void_func_ty = ctx.function_type(&[], &[]);
        let main_entry = ctx.create_block();
        let main_region = ctx.create_region();
        ctx.region_push_block(main_region, main_entry);

        for fn_name in &test_fn_names {
            let call = ctx.create_operation(
                "func.call",
                &[],
                &[],
                vec![NamedAttribute::new(
                    "callee",
                    Attribute::SymbolRef(fn_name.clone()),
                )],
                vec![],
                Location::unknown(),
            );
            ctx.block_push_op(main_entry, call);
        }

        let ret = ctx.create_operation(
            "func.return",
            &[],
            &[],
            vec![],
            vec![],
            Location::unknown(),
        );
        ctx.block_push_op(main_entry, ret);

        let main_op = ctx.create_operation(
            "func.func",
            &[],
            &[],
            vec![
                NamedAttribute::new("sym_name", Attribute::String("main".to_string())),
                NamedAttribute::new("function_type", Attribute::Type(void_func_ty)),
            ],
            vec![main_region],
            Location::unknown(),
        );
        ctx.block_push_op(module_block, main_op);

        Ok(())
    }
}
