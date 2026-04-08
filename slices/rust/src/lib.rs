//! # cot-slices
//!
//! The slices construct for CIR. Defines the slice type (a fat pointer of
//! base pointer + length) and five operations for string constants, pointer/
//! length extraction, element access, and array-to-slice conversion.
//!
//! ## Type
//!
//! `!cir.slice<T>` -- a fat pointer with `{ ptr: *T, len: i64 }`.
//!
//! ## Operations
//!
//! - `cir.string_constant` -- create a `!cir.slice<i8>` from a string literal
//! - `cir.slice_ptr` -- extract the data pointer from a slice
//! - `cir.slice_len` -- extract the length from a slice
//! - `cir.slice_elem` -- load an element from a slice by index (NOT Pure)
//! - `cir.array_to_slice` -- convert a pointer + range to a slice
//!
//! ## Registration
//!
//! Call [`register`] to register 1 type and 5 ops with the CIR dialect.

pub mod lowering;
pub mod ops;
pub mod types;

/// Register the slices construct's types and operations.
///
/// - Registers the `!cir.slice` extension type (on-demand via interner).
/// - Registers 5 ops under the `cir` dialect namespace.
pub fn register(ctx: &mut mlif::Context, _sema: &mut mlif::CIRSema) {
    types::register_types(ctx);
    ops::register_ops(ctx);
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlif::*;

    #[test]
    fn register_slice_ops() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        let dialect = ctx.get_dialect("cir").unwrap();
        assert!(dialect.get_op("cir.string_constant").is_some());
        assert!(dialect.get_op("cir.slice_ptr").is_some());
        assert!(dialect.get_op("cir.slice_len").is_some());
        assert!(dialect.get_op("cir.slice_elem").is_some());
        assert!(dialect.get_op("cir.array_to_slice").is_some());
    }

    #[test]
    fn register_all_5_ops() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        let dialect = ctx.get_dialect("cir").unwrap();

        let expected_ops = [
            "cir.string_constant",
            "cir.slice_ptr",
            "cir.slice_len",
            "cir.slice_elem",
            "cir.array_to_slice",
        ];
        assert_eq!(dialect.ops().len(), 5);
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

        // Pure ops
        for name in &[
            "cir.string_constant",
            "cir.slice_ptr",
            "cir.slice_len",
            "cir.array_to_slice",
        ] {
            let op = dialect.get_op(name).unwrap();
            assert!(op.has_trait(&OpTrait::Pure), "{} should be Pure", name);
        }

        // slice_elem is NOT Pure (memory read, Errata E6)
        let slice_elem = dialect.get_op("cir.slice_elem").unwrap();
        assert!(
            !slice_elem.has_trait(&OpTrait::Pure),
            "cir.slice_elem should NOT be Pure"
        );
    }

    #[test]
    fn create_slice_type() {
        let mut ctx = Context::new();
        let i32_ty = ctx.integer_type(32);

        let ty = types::slice_type(&mut ctx, i32_ty);

        match ctx.type_kind(ty) {
            TypeKind::Extension(ext) => {
                assert_eq!(ext.dialect, "cir");
                assert_eq!(ext.name, "slice");
                assert_eq!(ext.type_params.len(), 1);
                assert_eq!(ext.type_params[0], i32_ty);
                assert!(ext.int_params.is_empty());
                assert!(ext.string_params.is_empty());
            }
            _ => panic!("expected Extension type"),
        }
    }

    #[test]
    fn slice_type_interning() {
        let mut ctx = Context::new();
        let i8_ty = ctx.integer_type(8);

        let ty1 = types::slice_type(&mut ctx, i8_ty);
        let ty2 = types::slice_type(&mut ctx, i8_ty);

        assert_eq!(ty1, ty2);
    }

    #[test]
    fn different_elem_types_different_slices() {
        let mut ctx = Context::new();
        let i8_ty = ctx.integer_type(8);
        let i32_ty = ctx.integer_type(32);

        let s1 = types::slice_type(&mut ctx, i8_ty);
        let s2 = types::slice_type(&mut ctx, i32_ty);

        assert_ne!(s1, s2);
    }

    #[test]
    fn build_string_constant_op() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);

        let i8_ty = ctx.integer_type(8);
        let slice_i8_ty = types::slice_type(&mut ctx, i8_ty);

        let block = ctx.create_block();
        let result = ops::build_string_constant(
            &mut ctx,
            block,
            "hello, world",
            slice_i8_ty,
            Location::unknown(),
        );
        assert_eq!(ctx.value_type(result), slice_i8_ty);

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(block_ops.len(), 1);
        assert!(ctx[block_ops[0]].is_a("cir.string_constant"));
        assert_eq!(ctx[block_ops[0]].num_operands(), 0);

        // Verify string attribute.
        match ctx[block_ops[0]].get_attribute("value") {
            Some(Attribute::String(s)) => assert_eq!(s, "hello, world"),
            _ => panic!("expected string attribute for value"),
        }
    }

    #[test]
    fn build_slice_ptr_op() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);

        let i32_ty = ctx.integer_type(32);
        let slice_ty = types::slice_type(&mut ctx, i32_ty);
        let ptr_ty = ctx.extension_type(ExtensionType::new("cir", "ptr"));

        let block = ctx.create_block();
        let slice = ctx.block_add_argument(block, slice_ty);

        let result = ops::build_slice_ptr(&mut ctx, block, slice, ptr_ty, Location::unknown());
        assert_eq!(ctx.value_type(result), ptr_ty);

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(block_ops.len(), 1);
        assert!(ctx[block_ops[0]].is_a("cir.slice_ptr"));
        assert_eq!(ctx[block_ops[0]].num_operands(), 1);
    }

    #[test]
    fn build_slice_len_op() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);

        let i32_ty = ctx.integer_type(32);
        let i64_ty = ctx.integer_type(64);
        let slice_ty = types::slice_type(&mut ctx, i32_ty);

        let block = ctx.create_block();
        let slice = ctx.block_add_argument(block, slice_ty);

        let result = ops::build_slice_len(&mut ctx, block, slice, Location::unknown());
        assert_eq!(ctx.value_type(result), i64_ty);

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(block_ops.len(), 1);
        assert!(ctx[block_ops[0]].is_a("cir.slice_len"));
        assert_eq!(ctx[block_ops[0]].num_operands(), 1);
    }

    #[test]
    fn build_slice_elem_op() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);

        let i32_ty = ctx.integer_type(32);
        let i64_ty = ctx.integer_type(64);
        let slice_ty = types::slice_type(&mut ctx, i32_ty);

        let block = ctx.create_block();
        let slice = ctx.block_add_argument(block, slice_ty);
        let idx = ctx.block_add_argument(block, i64_ty);

        let result =
            ops::build_slice_elem(&mut ctx, block, slice, idx, i32_ty, Location::unknown());
        assert_eq!(ctx.value_type(result), i32_ty);

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(block_ops.len(), 1);
        assert!(ctx[block_ops[0]].is_a("cir.slice_elem"));
        assert_eq!(ctx[block_ops[0]].num_operands(), 2);
    }

    #[test]
    fn build_array_to_slice_op() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);

        let i32_ty = ctx.integer_type(32);
        let i64_ty = ctx.integer_type(64);
        let slice_ty = types::slice_type(&mut ctx, i32_ty);
        let ptr_ty = ctx.extension_type(ExtensionType::new("cir", "ptr"));

        let block = ctx.create_block();
        let base = ctx.block_add_argument(block, ptr_ty);
        let start = ctx.block_add_argument(block, i64_ty);
        let end = ctx.block_add_argument(block, i64_ty);

        let result = ops::build_array_to_slice(
            &mut ctx,
            block,
            base,
            start,
            end,
            slice_ty,
            Location::unknown(),
        );
        assert_eq!(ctx.value_type(result), slice_ty);

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(block_ops.len(), 1);
        assert!(ctx[block_ops[0]].is_a("cir.array_to_slice"));
        assert_eq!(ctx[block_ops[0]].num_operands(), 3);
    }
}
