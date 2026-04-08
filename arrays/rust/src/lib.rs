//! # cot-arrays
//!
//! The arrays construct for CIR. Defines the fixed-size array type and three
//! operations for initialization, element value extraction, and element pointer
//! access.
//!
//! ## Type
//!
//! `!cir.array<N x T>` -- a fixed-size array of N elements of type T.
//!
//! ## Operations
//!
//! - `cir.array_init` -- create an array value from element values
//! - `cir.elem_val` -- extract an element value by static index
//! - `cir.elem_ptr` -- get a pointer to an element by dynamic index
//!
//! ## Registration
//!
//! Call [`register`] to register 1 type and 3 ops with the CIR dialect.

pub mod lowering;
pub mod ops;
pub mod types;

/// Register the arrays construct's types and operations.
///
/// - Registers the `!cir.array` extension type (on-demand via interner).
/// - Registers 3 ops under the `cir` dialect namespace.
pub fn register(ctx: &mut mlif::Context, _sema: &mut mlif::CIRSema) {
    types::register_types(ctx);
    ops::register_ops(ctx);
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlif::*;

    #[test]
    fn register_array_ops() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        let dialect = ctx.get_dialect("cir").unwrap();
        assert!(dialect.get_op("cir.array_init").is_some());
        assert!(dialect.get_op("cir.elem_val").is_some());
        assert!(dialect.get_op("cir.elem_ptr").is_some());
    }

    #[test]
    fn register_all_3_ops() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        let dialect = ctx.get_dialect("cir").unwrap();

        let expected_ops = ["cir.array_init", "cir.elem_val", "cir.elem_ptr"];
        assert_eq!(dialect.ops().len(), 3);
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

        // All 3 ops are Pure
        for name in &["cir.array_init", "cir.elem_val", "cir.elem_ptr"] {
            let op = dialect.get_op(name).unwrap();
            assert!(op.has_trait(&OpTrait::Pure), "{} should be Pure", name);
        }
    }

    #[test]
    fn create_array_type() {
        let mut ctx = Context::new();
        let i32_ty = ctx.integer_type(32);

        let ty = types::array_type(&mut ctx, 4, i32_ty);

        // Verify the type is an extension type with the correct parameters.
        match ctx.type_kind(ty) {
            TypeKind::Extension(ext) => {
                assert_eq!(ext.dialect, "cir");
                assert_eq!(ext.name, "array");
                assert_eq!(ext.int_params, vec![4]);
                assert_eq!(ext.type_params.len(), 1);
                assert_eq!(ext.type_params[0], i32_ty);
            }
            _ => panic!("expected Extension type"),
        }
    }

    #[test]
    fn array_type_interning() {
        let mut ctx = Context::new();
        let f64_ty = ctx.float_type(64);

        let ty1 = types::array_type(&mut ctx, 3, f64_ty);
        let ty2 = types::array_type(&mut ctx, 3, f64_ty);

        // Same parameters should produce the same interned TypeId.
        assert_eq!(ty1, ty2);
    }

    #[test]
    fn different_sizes_different_types() {
        let mut ctx = Context::new();
        let i32_ty = ctx.integer_type(32);

        let ty3 = types::array_type(&mut ctx, 3, i32_ty);
        let ty4 = types::array_type(&mut ctx, 4, i32_ty);

        assert_ne!(ty3, ty4);
    }

    #[test]
    fn build_array_init_op() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);

        let i32_ty = ctx.integer_type(32);
        let arr_ty = types::array_type(&mut ctx, 3, i32_ty);

        let block = ctx.create_block();
        let a = ctx.block_add_argument(block, i32_ty);
        let b = ctx.block_add_argument(block, i32_ty);
        let c = ctx.block_add_argument(block, i32_ty);

        let result =
            ops::build_array_init(&mut ctx, block, &[a, b, c], arr_ty, Location::unknown());
        assert_eq!(ctx.value_type(result), arr_ty);

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(block_ops.len(), 1);
        assert!(ctx[block_ops[0]].is_a("cir.array_init"));
        assert_eq!(ctx[block_ops[0]].num_operands(), 3);
    }

    #[test]
    fn build_elem_val_op() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);

        let i32_ty = ctx.integer_type(32);
        let arr_ty = types::array_type(&mut ctx, 3, i32_ty);

        let block = ctx.create_block();
        let arr = ctx.block_add_argument(block, arr_ty);

        let elem = ops::build_elem_val(&mut ctx, block, arr, 1, i32_ty, Location::unknown());
        assert_eq!(ctx.value_type(elem), i32_ty);

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(block_ops.len(), 1);
        assert!(ctx[block_ops[0]].is_a("cir.elem_val"));

        // Verify index attribute.
        match ctx[block_ops[0]].get_attribute("index") {
            Some(Attribute::Integer { value, .. }) => assert_eq!(*value, 1),
            _ => panic!("expected integer index attribute"),
        }
    }

    #[test]
    fn build_elem_ptr_op() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);

        let i32_ty = ctx.integer_type(32);
        let i64_ty = ctx.integer_type(64);
        let arr_ty = types::array_type(&mut ctx, 4, i32_ty);
        let ptr_ty = ctx.extension_type(ExtensionType::new("cir", "ptr"));

        let block = ctx.create_block();
        let base = ctx.block_add_argument(block, ptr_ty);
        let idx = ctx.block_add_argument(block, i64_ty);

        let result = ops::build_elem_ptr(
            &mut ctx,
            block,
            base,
            idx,
            arr_ty,
            ptr_ty,
            Location::unknown(),
        );
        assert_eq!(ctx.value_type(result), ptr_ty);

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(block_ops.len(), 1);
        assert!(ctx[block_ops[0]].is_a("cir.elem_ptr"));
        assert_eq!(ctx[block_ops[0]].num_operands(), 2); // base + idx

        // Verify array_type attribute.
        match ctx[block_ops[0]].get_attribute("array_type") {
            Some(Attribute::Type(ty)) => assert_eq!(*ty, arr_ty),
            _ => panic!("expected type attribute for array_type"),
        }
    }
}
