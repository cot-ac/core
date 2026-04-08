//! # cot-memory
//!
//! The memory construct for CIR. Defines two types -- opaque pointers and typed
//! references -- and five operations for stack allocation, store, load,
//! address-of, and dereference.
//!
//! ## Types
//!
//! - `!cir.ptr` -- opaque pointer, like LLVM's `ptr`
//! - `!cir.ref<T>` -- typed reference carrying the pointee type `T`
//!
//! ## Operations
//!
//! - `cir.alloca` -- stack-allocate a slot, returns `!cir.ptr`
//! - `cir.store` -- write a value through a pointer
//! - `cir.load` -- read a value through a pointer
//! - `cir.addr_of` -- wrap a raw pointer in a typed reference (Pure)
//! - `cir.deref` -- dereference a typed reference to get the value
//!
//! ## Registration
//!
//! Call [`register`] to register all 5 ops and 2 types with the CIR dialect.

pub mod lowering;
pub mod ops;
pub mod types;

/// Register the memory construct's types and operations.
///
/// - Registers 5 ops under the `cir` dialect namespace.
/// - Types (`!cir.ptr`, `!cir.ref<T>`) are interned on-demand.
pub fn register(ctx: &mut mlif::Context, _sema: &mut mlif::CIRSema) {
    types::register_types(ctx);
    ops::register_ops(ctx);
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlif::*;

    #[test]
    fn register_memory_ops() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        let dialect = ctx.get_dialect("cir").unwrap();
        assert!(dialect.get_op("cir.alloca").is_some());
        assert!(dialect.get_op("cir.store").is_some());
        assert!(dialect.get_op("cir.load").is_some());
        assert!(dialect.get_op("cir.addr_of").is_some());
        assert!(dialect.get_op("cir.deref").is_some());
    }

    #[test]
    fn register_all_5_ops() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        let dialect = ctx.get_dialect("cir").unwrap();

        let expected_ops = [
            "cir.alloca",
            "cir.store",
            "cir.load",
            "cir.addr_of",
            "cir.deref",
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

        // alloca has no traits (MemAlloc, not Pure)
        let alloca = dialect.get_op("cir.alloca").unwrap();
        assert!(!alloca.has_trait(&OpTrait::Pure));
        assert!(!alloca.has_trait(&OpTrait::Terminator));

        // store has no traits (MemWrite, not Pure)
        let store = dialect.get_op("cir.store").unwrap();
        assert!(!store.has_trait(&OpTrait::Pure));

        // load has no traits (MemRead, not Pure)
        let load = dialect.get_op("cir.load").unwrap();
        assert!(!load.has_trait(&OpTrait::Pure));

        // addr_of IS Pure (zero-cost type wrapper)
        let addr_of = dialect.get_op("cir.addr_of").unwrap();
        assert!(addr_of.has_trait(&OpTrait::Pure));

        // deref has no traits (MemRead, not Pure)
        let deref = dialect.get_op("cir.deref").unwrap();
        assert!(!deref.has_trait(&OpTrait::Pure));
    }

    #[test]
    fn build_alloca_store_load() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);

        let i32_ty = ctx.integer_type(32);
        let block = ctx.create_block();

        // Allocate stack slot for i32
        let ptr = ops::build_alloca(&mut ctx, block, i32_ty, Location::unknown());
        let ptr_ty = types::ptr_type(&mut ctx);
        assert_eq!(ctx.value_type(ptr), ptr_ty);

        // Store a value
        let val = ctx.block_add_argument(block, i32_ty);
        ops::build_store(&mut ctx, block, val, ptr, Location::unknown());

        // Load from the pointer
        let loaded = ops::build_load(&mut ctx, block, ptr, i32_ty, Location::unknown());
        assert_eq!(ctx.value_type(loaded), i32_ty);

        // Verify ops in block
        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(block_ops.len(), 3);
        assert!(ctx[block_ops[0]].is_a("cir.alloca"));
        assert!(ctx[block_ops[1]].is_a("cir.store"));
        assert!(ctx[block_ops[2]].is_a("cir.load"));
    }

    #[test]
    fn alloca_has_elem_type_attribute() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);

        let i64_ty = ctx.integer_type(64);
        let block = ctx.create_block();

        let _ptr = ops::build_alloca(&mut ctx, block, i64_ty, Location::unknown());

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        match ctx[block_ops[0]].get_attribute("elem_type") {
            Some(Attribute::Type(ty)) => assert_eq!(*ty, i64_ty),
            _ => panic!("expected Type attribute for elem_type"),
        }
    }

    #[test]
    fn build_addr_of_and_deref() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);

        let i32_ty = ctx.integer_type(32);
        let block = ctx.create_block();

        // Allocate and get a pointer
        let ptr = ops::build_alloca(&mut ctx, block, i32_ty, Location::unknown());

        // Convert pointer to typed reference
        let reference = ops::build_addr_of(&mut ctx, block, ptr, i32_ty, Location::unknown());
        let ref_ty = types::ref_type(&mut ctx, i32_ty);
        assert_eq!(ctx.value_type(reference), ref_ty);

        // Dereference the typed reference
        let value = ops::build_deref(&mut ctx, block, reference, i32_ty, Location::unknown());
        assert_eq!(ctx.value_type(value), i32_ty);

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(block_ops.len(), 3);
        assert!(ctx[block_ops[0]].is_a("cir.alloca"));
        assert!(ctx[block_ops[1]].is_a("cir.addr_of"));
        assert!(ctx[block_ops[2]].is_a("cir.deref"));
    }

    #[test]
    fn ptr_type_is_stable() {
        let mut ctx = Context::new();
        let ptr1 = types::ptr_type(&mut ctx);
        let ptr2 = types::ptr_type(&mut ctx);
        assert_eq!(ptr1, ptr2, "ptr_type should return the same interned TypeId");
    }

    #[test]
    fn ref_type_parameterized() {
        let mut ctx = Context::new();
        let i32_ty = ctx.integer_type(32);
        let i64_ty = ctx.integer_type(64);

        let ref_i32 = types::ref_type(&mut ctx, i32_ty);
        let ref_i64 = types::ref_type(&mut ctx, i64_ty);
        let ref_i32_again = types::ref_type(&mut ctx, i32_ty);

        assert_ne!(ref_i32, ref_i64, "ref<i32> and ref<i64> should differ");
        assert_eq!(ref_i32, ref_i32_again, "ref<i32> should be interned");
    }

    #[test]
    fn store_has_no_results() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);

        let i32_ty = ctx.integer_type(32);
        let block = ctx.create_block();

        let ptr = ops::build_alloca(&mut ctx, block, i32_ty, Location::unknown());
        let val = ctx.block_add_argument(block, i32_ty);
        ops::build_store(&mut ctx, block, val, ptr, Location::unknown());

        let block_ops: Vec<OpId> = ctx.block_ops(block).collect();
        assert_eq!(ctx[block_ops[1]].num_results(), 0);
    }
}
