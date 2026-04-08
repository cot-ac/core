//! # Arith Transforms
//!
//! Contains the `ArithSemaStep` — the type coercion step that inserts implicit
//! casts at function call boundaries. When an argument type does not exactly
//! match the callee's parameter type, this step inserts the appropriate cast
//! operation (`extsi`, `trunci`, `sitofp`, `fptosi`, `extf`, or `truncf`)
//! before the call.
//!
//! Mirrors the C++ `SemanticAnalysisPass` in `arith/cpp/Transform.cpp`.

use mlif::{
    Attribute, Context, DiagnosticError, Location, OpId, SemaState, SemaStep, StepPosition, TypeId,
    ValueId,
};

/// Semantic analysis step for arith: insert implicit casts at call sites.
pub struct ArithSemaStep;

impl SemaStep for ArithSemaStep {
    fn name(&self) -> &str {
        "arith-type-coercion"
    }

    fn position(&self) -> StepPosition {
        StepPosition::Types
    }

    fn visit_op(
        &mut self,
        op: OpId,
        ctx: &mut Context,
        state: &SemaState,
    ) -> Result<bool, DiagnosticError> {
        // Only handle func.call ops.
        if !ctx[op].is_a("func.call") {
            return Ok(false);
        }

        // Look up callee function from symbol table.
        let callee_name = match ctx[op].get_attribute("callee") {
            Some(Attribute::SymbolRef(name)) => name.clone(),
            _ => return Ok(false),
        };

        let callee_op = match state.symbol_table.lookup(&callee_name) {
            Some(op) => op,
            None => return Ok(false),
        };

        // Get callee's function type from its "function_type" attribute.
        let func_type = match ctx[callee_op].get_attribute("function_type") {
            Some(Attribute::Type(ty)) => *ty,
            _ => return Ok(false),
        };

        let param_types = match ctx.function_type_params(func_type) {
            Some(params) => params.to_vec(),
            None => return Ok(false),
        };

        // Snapshot operands — we'll modify them in place.
        let operands = ctx[op].operands().to_vec();
        let loc = ctx[op].location().clone();

        let mut changed = false;
        for (i, &arg) in operands.iter().enumerate() {
            if i >= param_types.len() {
                break;
            }
            let arg_ty = ctx.value_type(arg);
            let param_ty = param_types[i];

            if arg_ty == param_ty {
                continue;
            }

            // Insert the appropriate cast op before the call.
            if let Some(cast_value) = insert_cast(ctx, op, arg, arg_ty, param_ty, &loc) {
                // Replace this specific operand on the call op.
                // We use replace_all_uses which replaces every use — this is safe
                // for sema because each SSA value has a single defining op and the
                // cast narrows the type for all downstream consumers.
                ctx.replace_all_uses(arg, cast_value);
                changed = true;
            }
        }

        Ok(changed)
    }
}

/// Insert a cast op before `before_op`. Returns the cast result, or `None`
/// if no valid cast exists for the given type pair.
fn insert_cast(
    ctx: &mut Context,
    before_op: OpId,
    value: ValueId,
    src_ty: TypeId,
    dst_ty: TypeId,
    loc: &Location,
) -> Option<ValueId> {
    let src_int = ctx.is_integer_type(src_ty);
    let dst_int = ctx.is_integer_type(dst_ty);
    let src_float = ctx.is_float_type(src_ty);
    let dst_float = ctx.is_float_type(dst_ty);

    let op_name = if src_int && dst_int {
        let sw = ctx.integer_type_width(src_ty)?;
        let dw = ctx.integer_type_width(dst_ty)?;
        if sw < dw {
            "cir.extsi"
        } else if sw > dw {
            "cir.trunci"
        } else {
            return None;
        }
    } else if src_int && dst_float {
        "cir.sitofp"
    } else if src_float && dst_int {
        "cir.fptosi"
    } else if src_float && dst_float {
        let sw = ctx.float_type_width(src_ty)?;
        let dw = ctx.float_type_width(dst_ty)?;
        if sw < dw {
            "cir.extf"
        } else if sw > dw {
            "cir.truncf"
        } else {
            return None;
        }
    } else {
        return None;
    };

    let cast_op = ctx.create_operation(op_name, &[value], &[dst_ty], vec![], vec![], loc.clone());
    ctx.block_insert_op_before(before_op, cast_op);
    Some(ctx.op_result(cast_op, 0))
}
