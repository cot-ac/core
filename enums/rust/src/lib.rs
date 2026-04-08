//! # cot-enums
//!
//! Enum construct for CIR: `!cir.enum<"Name", TagType, "V0", "V1", ...>`.
//! 2 ops: enum_constant, enum_value.
//! Reference: Rust C-like enum, C/C++ enum.

pub mod lowering;
pub mod ops;
pub mod types;

/// Register enum type and ops with the MLIF context.
pub fn register(ctx: &mut mlif::Context, _sema: &mut mlif::CIRSema) {
    types::register_types(ctx);
    ops::register_ops(ctx);
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlif::*;

    #[test]
    fn register_enum_ops() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        let d = ctx.get_dialect("cir").unwrap();
        assert!(d.get_op("cir.enum_constant").is_some());
        assert!(d.get_op("cir.enum_value").is_some());
    }

    #[test]
    fn register_all_2_ops() {
        let mut ctx = Context::new();
        let mut sema = CIRSema::new();
        register(&mut ctx, &mut sema);
        assert_eq!(ctx.get_dialect("cir").unwrap().ops().len(), 2);
    }

    #[test]
    fn create_enum_type() {
        let mut ctx = Context::new();
        let i32_ty = ctx.integer_type(32);
        let color = types::enum_type(&mut ctx, "Color", i32_ty, &["Red", "Green", "Blue"]);
        let color2 = types::enum_type(&mut ctx, "Color", i32_ty, &["Red", "Green", "Blue"]);
        assert_eq!(color, color2, "same enum type should intern");

        let dir = types::enum_type(&mut ctx, "Dir", i32_ty, &["N", "S", "E", "W"]);
        assert_ne!(color, dir, "different enum → different type");
    }

    #[test]
    fn build_enum_constant_op() {
        let mut ctx = Context::new();
        let i32_ty = ctx.integer_type(32);
        let color_ty = types::enum_type(&mut ctx, "Color", i32_ty, &["Red", "Green", "Blue"]);
        let block = ctx.create_block();

        let val =
            ops::build_enum_constant(&mut ctx, block, color_ty, "Green", Location::unknown());
        assert_eq!(ctx.value_type(val), color_ty);

        let op_list: Vec<OpId> = ctx.block_ops(block).collect();
        assert!(ctx[op_list[0]].is_a("cir.enum_constant"));
        match ctx[op_list[0]].get_attribute("variant") {
            Some(Attribute::String(s)) => assert_eq!(s, "Green"),
            _ => panic!("expected variant string attribute"),
        }
    }

    #[test]
    fn build_enum_value_op() {
        let mut ctx = Context::new();
        let i32_ty = ctx.integer_type(32);
        let color_ty = types::enum_type(&mut ctx, "Color", i32_ty, &["Red", "Green", "Blue"]);
        let block = ctx.create_block();

        let val =
            ops::build_enum_constant(&mut ctx, block, color_ty, "Red", Location::unknown());
        let tag = ops::build_enum_value(&mut ctx, block, val, i32_ty, Location::unknown());
        assert_eq!(ctx.value_type(tag), i32_ty);
    }
}
