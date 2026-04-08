//! GenericSpecializerStep — CIRSema step for monomorphization.
//!
//! Visits `cir.generic_apply` ops, clones the callee function with
//! concrete type substitutions, and replaces with `func.call`.
//! After this step, no `!cir.type_param` or `generic_apply` ops remain.
//!
//! Reference: `core/generics/cpp/GenericsConstruct.cpp` (GenericSpecializerStep)

use std::collections::HashMap;

use mlif::*;

/// Sema step that monomorphizes generic function calls.
pub struct GenericsSemaStep {
    /// Cache of already-specialized functions: mangled_name → func OpId.
    specializations: HashMap<String, OpId>,
}

impl GenericsSemaStep {
    pub fn new() -> Self {
        Self {
            specializations: HashMap::new(),
        }
    }
}

impl Default for GenericsSemaStep {
    fn default() -> Self {
        Self::new()
    }
}

impl SemaStep for GenericsSemaStep {
    fn name(&self) -> &str {
        "generic-specializer"
    }

    fn position(&self) -> StepPosition {
        StepPosition::Generics
    }

    fn visit_op(
        &mut self,
        op: OpId,
        ctx: &mut Context,
        state: &SemaState,
    ) -> Result<bool, DiagnosticError> {
        if !ctx[op].is_a("cir.generic_apply") {
            return Ok(false);
        }

        // Extract substitution map from attributes.
        let callee = match ctx[op].get_attribute("callee") {
            Some(Attribute::SymbolRef(name)) => name.clone(),
            _ => return Ok(false),
        };
        let sub_keys = match ctx[op].get_attribute("sub_keys") {
            Some(Attribute::Array(arr)) => arr
                .iter()
                .filter_map(|a| match a {
                    Attribute::String(s) => Some(s.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>(),
            _ => return Ok(false),
        };
        let sub_types = match ctx[op].get_attribute("sub_types") {
            Some(Attribute::Array(arr)) => arr
                .iter()
                .filter_map(|a| match a {
                    Attribute::Type(t) => Some(*t),
                    _ => None,
                })
                .collect::<Vec<_>>(),
            _ => return Ok(false),
        };

        // Build mangled name: callee__type1__type2
        let mut mangled = callee.clone();
        for &ty in &sub_types {
            mangled.push_str("__");
            mangled.push_str(&ctx.format_type(ty));
        }

        // Look up or create specialization.
        let specialized_name = if let Some(&existing) = self.specializations.get(&mangled) {
            // Already specialized — get its sym_name.
            match ctx[existing].get_attribute("sym_name") {
                Some(Attribute::String(name)) => name.clone(),
                _ => mangled.clone(),
            }
        } else {
            // Look up the generic callee function.
            let callee_op = match state.symbol_table.lookup(&callee) {
                Some(op) => op,
                None => {
                    return Err(DiagnosticError::single(
                        ctx[op].location().clone(),
                        format!("generic callee '{}' not found", callee),
                    ));
                }
            };

            // Build substitution map: "T" → i32, etc.
            let mut subs: HashMap<String, TypeId> = HashMap::new();
            for (key, &ty) in sub_keys.iter().zip(sub_types.iter()) {
                subs.insert(key.clone(), ty);
            }

            // Clone and specialize the function.
            let specialized = specialize_function(ctx, callee_op, &subs, &mangled);
            self.specializations.insert(mangled.clone(), specialized);

            // Insert the specialized function into the module body.
            ctx.block_insert_op_before(callee_op, specialized);

            mangled
        };

        // Rewrite: generic_apply → func.call.
        let args = ctx[op].operands().to_vec();
        let result_types: Vec<TypeId> = ctx[op].results().iter().map(|&v| ctx.value_type(v)).collect();
        let loc = ctx[op].location().clone();
        let call_op = ctx.create_operation(
            "func.call",
            &args,
            &result_types,
            vec![NamedAttribute::new(
                "callee",
                Attribute::SymbolRef(specialized_name),
            )],
            vec![],
            loc,
        );
        ctx.block_insert_op_before(op, call_op);

        // Replace uses of generic_apply results with call results.
        let old_results = ctx[op].results().to_vec();
        let new_results: Vec<ValueId> = (0..result_types.len())
            .map(|i| ctx.op_result(call_op, i))
            .collect();
        for (old, new) in old_results.iter().zip(new_results.iter()) {
            ctx.replace_all_uses(*old, *new);
        }

        // Erase the generic_apply op.
        ctx.erase_op(op);

        Ok(true)
    }
}

/// Clone a generic function with type substitutions applied.
///
/// This is the Rust equivalent of `GenericSpecializerStep::specializeFunction`
/// in the C++ implementation. Uses a HashMap<ValueId, ValueId> for value
/// remapping since MLIF doesn't have MLIR's IRMapping.
fn specialize_function(
    ctx: &mut Context,
    generic_func: OpId,
    subs: &HashMap<String, TypeId>,
    mangled_name: &str,
) -> OpId {
    // Get the generic function's type and substitute.
    let func_type = match ctx[generic_func].get_attribute("function_type") {
        Some(Attribute::Type(ty)) => *ty,
        _ => panic!("func.func missing function_type attribute"),
    };

    let old_params = ctx.function_type_params(func_type).unwrap().to_vec();
    let old_results = ctx.function_type_results(func_type).unwrap().to_vec();

    let new_params: Vec<TypeId> = old_params.iter().map(|&t| substitute_type(ctx, t, subs)).collect();
    let new_results: Vec<TypeId> = old_results.iter().map(|&t| substitute_type(ctx, t, subs)).collect();
    let new_func_type = ctx.function_type(&new_params, &new_results);

    // Snapshot the generic function's structure before mutating.
    let generic_body = ctx[generic_func].region(0);
    let old_blocks: Vec<BlockId> = ctx[generic_body].blocks().to_vec();

    // Snapshot each block's arguments and operations.
    struct BlockSnapshot {
        args: Vec<(ValueId, TypeId)>,
        ops: Vec<OpSnapshot>,
    }
    struct OpSnapshot {
        name: String,
        operands: Vec<ValueId>,
        result_vals: Vec<ValueId>,
        result_types: Vec<TypeId>,
        attrs: Vec<NamedAttribute>,
        loc: Location,
        num_regions: usize,
    }

    let snapshots: Vec<BlockSnapshot> = old_blocks
        .iter()
        .map(|&block| {
            let args: Vec<(ValueId, TypeId)> = ctx[block]
                .arguments()
                .iter()
                .map(|&v| (v, ctx.value_type(v)))
                .collect();
            let ops: Vec<OpSnapshot> = ctx
                .block_ops(block)
                .map(|op| OpSnapshot {
                    name: ctx[op].name().to_string(),
                    operands: ctx[op].operands().to_vec(),
                    result_vals: ctx[op].results().to_vec(),
                    result_types: ctx[op]
                        .results()
                        .iter()
                        .map(|&v| ctx.value_type(v))
                        .collect(),
                    attrs: ctx[op].attributes().to_vec(),
                    loc: ctx[op].location().clone(),
                    num_regions: ctx[op].num_regions(),
                })
                .collect();
            BlockSnapshot { args, ops }
        })
        .collect();

    // Now create the cloned structure using the snapshots.
    let new_body_region = ctx.create_region();
    let mut value_map: HashMap<ValueId, ValueId> = HashMap::new();

    for snap in &snapshots {
        let new_block = ctx.create_block();

        // Clone block arguments with substituted types.
        for &(old_arg, old_ty) in &snap.args {
            let new_ty = substitute_type(ctx, old_ty, subs);
            let new_arg = ctx.block_add_argument(new_block, new_ty);
            value_map.insert(old_arg, new_arg);
        }

        // Clone operations.
        for op_snap in &snap.ops {
            let new_operands: Vec<ValueId> = op_snap
                .operands
                .iter()
                .map(|&v| value_map.get(&v).copied().unwrap_or(v))
                .collect();

            let new_result_types: Vec<TypeId> = op_snap
                .result_types
                .iter()
                .map(|&t| substitute_type(ctx, t, subs))
                .collect();

            // Create empty regions for nested ops (flat functions for Phase 2).
            let new_regions: Vec<RegionId> = (0..op_snap.num_regions)
                .map(|_| ctx.create_region())
                .collect();

            let new_op = ctx.create_operation(
                &op_snap.name,
                &new_operands,
                &new_result_types,
                op_snap.attrs.clone(),
                new_regions,
                op_snap.loc.clone(),
            );
            ctx.block_push_op(new_block, new_op);

            // Map old results to new results.
            for (i, &old_result) in op_snap.result_vals.iter().enumerate() {
                value_map.insert(old_result, ctx.op_result(new_op, i));
            }
        }

        ctx.region_push_block(new_body_region, new_block);
    }

    // Create the specialized func.func operation.
    ctx.create_operation(
        "func.func",
        &[],
        &[],
        vec![
            NamedAttribute::new("sym_name", Attribute::String(mangled_name.to_string())),
            NamedAttribute::new("function_type", Attribute::Type(new_func_type)),
        ],
        vec![new_body_region],
        Location::unknown(),
    )
}

/// Substitute type_param types with concrete types from the substitution map.
fn substitute_type(ctx: &mut Context, ty: TypeId, subs: &HashMap<String, TypeId>) -> TypeId {
    match ctx.type_kind(ty) {
        TypeKind::Extension(ext) if ext.dialect == "cir" && ext.name == "type_param" => {
            if let Some(param_name) = ext.string_params.first() {
                if let Some(&concrete) = subs.get(param_name) {
                    return concrete;
                }
            }
            ty
        }
        _ => ty,
    }
}
