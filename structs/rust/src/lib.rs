//! # cot-structs
//!
//! The structs construct for CIR. Defines the named struct type and three
//! operations for initialization, field value extraction, and field pointer
//! access.
//!
//! ## Type
//!
//! `!cir.struct<"Name", "f1": T1, "f2": T2, ...>` -- a named aggregate with
//! ordered, named fields.
//!
//! ## Operations
//!
//! - `cir.struct_init` -- create a struct value from field values
//! - `cir.field_val` -- extract a field value by index
//! - `cir.field_ptr` -- get a pointer to a field by index
//!
//! ## Registration
//!
//! Call [`register`] to register 1 type and 3 ops with the CIR dialect.

pub mod lowering;
pub mod ops;
pub mod types;

/// Register the structs construct's types and operations.
///
/// - Registers the `!cir.struct` extension type (on-demand via interner).
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
    fn register_struct_ops() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        let dialect = ctx.get_dialect("cir").unwrap();
        assert!(dialect.get_op("cir.struct_init").is_some());
        assert!(dialect.get_op("cir.field_val").is_some());
        assert!(dialect.get_op("cir.field_ptr").is_some());
    }

    #[test]
    fn register_all_3_ops() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        let dialect = ctx.get_dialect("cir").unwrap();

        let expected_ops = ["cir.struct_init", "cir.field_val", "cir.field_ptr"];
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
        for name in &["cir.struct_init", "cir.field_val", "cir.field_ptr"] {
            let op = dialect.get_op(name).unwrap();
            assert!(op.has_trait(&OpTrait::Pure), "{} should be Pure", name);
        }
    }

    #[test]
    fn create_struct_type() {
        let mut ctx = Context::new();
        let i32_ty = ctx.integer_type(32);
        let f64_ty = ctx.float_type(64);

        let ty = types::struct_type(&mut ctx, "Point", &["x", "y"], &[i32_ty, f64_ty]);

        // Verify the type is an extension type with the correct parameters.
        match ctx.type_kind(ty) {
            TypeKind::Extension(ext) => {
                assert_eq!(ext.dialect, "cir");
                assert_eq!(ext.name, "struct");
                assert_eq!(ext.string_params.len(), 3); // "Point", "x", "y"
                assert_eq!(ext.string_params[0], "Point");
                assert_eq!(ext.string_params[1], "x");
                assert_eq!(ext.string_params[2], "y");
                assert_eq!(ext.type_params.len(), 2);
                assert_eq!(ext.type_params[0], i32_ty);
                assert_eq!(ext.type_params[1], f64_ty);
            }
            _ => panic!("expected Extension type"),
        }
    }

    #[test]
    fn struct_type_interning() {
        let mut ctx = Context::new();
        let i32_ty = ctx.integer_type(32);

        let ty1 = types::struct_type(&mut ctx, "Pair", &["a", "b"], &[i32_ty, i32_ty]);
        let ty2 = types::struct_type(&mut ctx, "Pair", &["a", "b"], &[i32_ty, i32_ty]);

        // Same parameters should produce the same interned TypeId.
        assert_eq!(ty1, ty2);
    }

    #[test]
    fn build_struct_init_op() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);

        let i32_ty = ctx.integer_type(32);
        let f64_ty = ctx.float_type(64);
        let struct_ty = types::struct_type(&mut ctx, "Point", &["x", "y"], &[i32_ty, f64_ty]);

        let block = ctx.create_block();
        let x = ctx.block_add_argument(block, i32_ty);
        let y = ctx.block_add_argument(block, f64_ty);

        let result = ops::build_struct_init(&mut ctx, block, &[x, y], struct_ty, Location::unknown());
        assert_eq!(ctx.value_type(result), struct_ty);

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(block_ops.len(), 1);
        assert!(ctx[block_ops[0]].is_a("cir.struct_init"));
        assert_eq!(ctx[block_ops[0]].num_operands(), 2);
    }

    #[test]
    fn build_field_val_op() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);

        let i32_ty = ctx.integer_type(32);
        let f64_ty = ctx.float_type(64);
        let struct_ty = types::struct_type(&mut ctx, "Point", &["x", "y"], &[i32_ty, f64_ty]);

        let block = ctx.create_block();
        let s = ctx.block_add_argument(block, struct_ty);

        // Extract field 0 (x: i32)
        let x = ops::build_field_val(&mut ctx, block, s, 0, i32_ty, Location::unknown());
        assert_eq!(ctx.value_type(x), i32_ty);

        // Extract field 1 (y: f64)
        let y = ops::build_field_val(&mut ctx, block, s, 1, f64_ty, Location::unknown());
        assert_eq!(ctx.value_type(y), f64_ty);

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(block_ops.len(), 2);
        assert!(ctx[block_ops[0]].is_a("cir.field_val"));
        assert!(ctx[block_ops[1]].is_a("cir.field_val"));

        // Verify index attribute on first op.
        match ctx[block_ops[0]].get_attribute("index") {
            Some(Attribute::Integer { value, .. }) => assert_eq!(*value, 0),
            _ => panic!("expected integer index attribute"),
        }
        // Verify index attribute on second op.
        match ctx[block_ops[1]].get_attribute("index") {
            Some(Attribute::Integer { value, .. }) => assert_eq!(*value, 1),
            _ => panic!("expected integer index attribute"),
        }
    }

    #[test]
    fn build_field_ptr_op() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);

        let i32_ty = ctx.integer_type(32);
        let f64_ty = ctx.float_type(64);
        let struct_ty = types::struct_type(&mut ctx, "Point", &["x", "y"], &[i32_ty, f64_ty]);
        let ptr_ty = ctx.extension_type(ExtensionType::new("cir", "ptr"));

        let block = ctx.create_block();
        let base = ctx.block_add_argument(block, ptr_ty);

        let result = ops::build_field_ptr(
            &mut ctx,
            block,
            base,
            1, // field index
            struct_ty,
            ptr_ty,
            Location::unknown(),
        );
        assert_eq!(ctx.value_type(result), ptr_ty);

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(block_ops.len(), 1);
        assert!(ctx[block_ops[0]].is_a("cir.field_ptr"));

        // Verify index attribute.
        match ctx[block_ops[0]].get_attribute("index") {
            Some(Attribute::Integer { value, .. }) => assert_eq!(*value, 1),
            _ => panic!("expected integer index attribute"),
        }

        // Verify struct_type attribute.
        match ctx[block_ops[0]].get_attribute("struct_type") {
            Some(Attribute::Type(ty)) => assert_eq!(*ty, struct_ty),
            _ => panic!("expected type attribute for struct_type"),
        }
    }
}
