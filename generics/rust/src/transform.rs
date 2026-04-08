//! # Generic Transforms
//!
//! Contains the GenericSpecializer pass (monomorphization). This pass:
//! 1. Collects all `generic_apply` call sites and their concrete type arguments
//! 2. For each unique combination, clones the generic function body
//! 3. Substitutes all `!cir.type_param<"T">` occurrences with concrete types
//! 4. Replaces `generic_apply` calls with direct calls to the specialized version
//! 5. Removes unused generic function definitions
//!
//! After this pass, no `!cir.type_param` or `generic_apply` ops remain in the IR.

use mlif::Context;

/// Run the GenericSpecializer pass: monomorphize all generic functions.
pub fn generic_specializer(_ctx: &mut Context) {
    todo!()
}
