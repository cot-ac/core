//===- Ops.td - cot-core operation definitions ----------------*- tablegen -*-===//
//
// cot-core: constants, arithmetic, comparison, casts, select.
// Reference: MLIR Arith dialect, Flang FIR arithmetic ops.
//
//===----------------------------------------------------------------------===//

#ifndef ARITH_OPS
#define ARITH_OPS

include "cot/CIR/CIROps.td"  // Base classes: CIR_Op, CIR_BinaryOp, etc.
include "mlir/IR/BuiltinAttributeInterfaces.td"  // TypedAttrInterface (E7)

//===----------------------------------------------------------------------===//
// Constants
//===----------------------------------------------------------------------===//

def CIR_ConstantOp : CIR_Op<"constant", [Pure, ConstantLike]> {
  let summary = "integer/float/bool constant";
  let description = [{
    Produces a constant value. The value attribute holds the literal.
    Reference: Every IR has constants — Zig int literal, Swift integer_literal.
  }];
  let arguments = (ins TypedAttrInterface:$value);  // E7: TypedAttrInterface
  let results = (outs AnyType:$result);
  let hasCustomAssemblyFormat = 1;
  let hasVerifier = 1;
  let hasFolder = 1;
}

//===----------------------------------------------------------------------===//
// Arithmetic — integer+float (E3: separate int/float variants for div/rem/neg)
//===----------------------------------------------------------------------===//

def CIR_AddOp : CIR_BinaryOp<"add", [Commutative]> {
  let summary = "integer/float addition";
  let hasFolder = 1;
  let hasCanonicalizeMethod = 1;
}
def CIR_SubOp : CIR_BinaryOp<"sub"> {
  let summary = "integer/float subtraction";
  let hasFolder = 1;
  let hasCanonicalizeMethod = 1;
}
def CIR_MulOp : CIR_BinaryOp<"mul", [Commutative]> {
  let summary = "integer/float multiplication";
  let hasFolder = 1;
  let hasCanonicalizeMethod = 1;
}

// Signed integer division
def CIR_DivSIOp : CIR_IntBinaryOp<"divsi"> {
  let summary = "signed integer division";
}
// Unsigned integer division
def CIR_DivUIOp : CIR_IntBinaryOp<"divui"> {
  let summary = "unsigned integer division";
}
// Float division
def CIR_DivFOp : CIR_FloatBinaryOp<"divf"> {
  let summary = "float division";
}

// Signed integer remainder
def CIR_RemSIOp : CIR_IntBinaryOp<"remsi"> {
  let summary = "signed integer remainder";
}
// Unsigned integer remainder
def CIR_RemUIOp : CIR_IntBinaryOp<"remui"> {
  let summary = "unsigned integer remainder";
}
// Float remainder
def CIR_RemFOp : CIR_FloatBinaryOp<"remf"> {
  let summary = "float remainder";
}

// Integer negation
def CIR_NegOp : CIR_IntUnaryOp<"neg"> {
  let summary = "integer negation (0 - x)";
}

// Float negation
def CIR_NegFOp : CIR_Op<"negf", [Pure, SameOperandsAndResultType]> {
  let summary = "float negation";
  let arguments = (ins AnyFloat:$operand);
  let results = (outs AnyFloat:$result);
  let assemblyFormat = "$operand attr-dict `:` type($result)";
}

//===----------------------------------------------------------------------===//
// Bitwise
//===----------------------------------------------------------------------===//

def CIR_BitAndOp : CIR_IntBinaryOp<"bit_and", [Commutative]> {
  let summary = "bitwise AND";
}
def CIR_BitOrOp : CIR_IntBinaryOp<"bit_or", [Commutative]> {
  let summary = "bitwise OR";
}
def CIR_BitXorOp : CIR_IntBinaryOp<"bit_xor", [Commutative]> {
  let summary = "bitwise XOR";
}
def CIR_BitNotOp : CIR_IntUnaryOp<"bit_not"> {
  let summary = "bitwise NOT";
}

//===----------------------------------------------------------------------===//
// Shifts
//===----------------------------------------------------------------------===//

def CIR_ShlOp : CIR_IntBinaryOp<"shl"> {
  let summary = "shift left";
}
def CIR_ShrOp : CIR_IntBinaryOp<"shr"> {
  let summary = "logical (unsigned) shift right";
}
def CIR_ShrSOp : CIR_IntBinaryOp<"shr_s"> {
  let summary = "arithmetic (signed) shift right";
}

//===----------------------------------------------------------------------===//
// Comparison — E3: separate integer and float variants
//===----------------------------------------------------------------------===//

// Integer comparison predicates (E4: do NOT match LLVM values)
def CIR_CmpIPredicate_eq  : I64EnumAttrCase<"eq", 0>;
def CIR_CmpIPredicate_ne  : I64EnumAttrCase<"ne", 1>;
def CIR_CmpIPredicate_slt : I64EnumAttrCase<"slt", 2>;
def CIR_CmpIPredicate_sle : I64EnumAttrCase<"sle", 3>;
def CIR_CmpIPredicate_sgt : I64EnumAttrCase<"sgt", 4>;
def CIR_CmpIPredicate_sge : I64EnumAttrCase<"sge", 5>;
def CIR_CmpIPredicate_ult : I64EnumAttrCase<"ult", 6>;
def CIR_CmpIPredicate_ule : I64EnumAttrCase<"ule", 7>;
def CIR_CmpIPredicate_ugt : I64EnumAttrCase<"ugt", 8>;
def CIR_CmpIPredicate_uge : I64EnumAttrCase<"uge", 9>;

def CIR_CmpIPredicateAttr : I64EnumAttr<
    "CmpIPredicate", "integer comparison predicate",
    [CIR_CmpIPredicate_eq, CIR_CmpIPredicate_ne,
     CIR_CmpIPredicate_slt, CIR_CmpIPredicate_sle,
     CIR_CmpIPredicate_sgt, CIR_CmpIPredicate_sge,
     CIR_CmpIPredicate_ult, CIR_CmpIPredicate_ule,
     CIR_CmpIPredicate_ugt, CIR_CmpIPredicate_uge]> {
  let cppNamespace = "::cir";
}

def CIR_CmpOp : CIR_Op<"cmp", [Pure, SameTypeOperands]> {
  let summary = "integer comparison with predicate";
  let arguments = (ins CIR_CmpIPredicateAttr:$predicate,
                       CIR_IntLike:$lhs, CIR_IntLike:$rhs);
  let results = (outs I1:$result);
  let assemblyFormat = "$predicate `,` $lhs `,` $rhs attr-dict `:` type($lhs)";
}

// Float comparison predicates — E3/E5: ordered/unordered NaN variants
def CIR_CmpFPredicate_oeq : I64EnumAttrCase<"oeq", 0>;
def CIR_CmpFPredicate_ogt : I64EnumAttrCase<"ogt", 1>;
def CIR_CmpFPredicate_oge : I64EnumAttrCase<"oge", 2>;
def CIR_CmpFPredicate_olt : I64EnumAttrCase<"olt", 3>;
def CIR_CmpFPredicate_ole : I64EnumAttrCase<"ole", 4>;
def CIR_CmpFPredicate_one : I64EnumAttrCase<"one", 5>;
def CIR_CmpFPredicate_ord : I64EnumAttrCase<"ord", 6>;
def CIR_CmpFPredicate_ueq : I64EnumAttrCase<"ueq", 7>;
def CIR_CmpFPredicate_ugt : I64EnumAttrCase<"ugt", 8>;
def CIR_CmpFPredicate_uge : I64EnumAttrCase<"uge", 9>;
def CIR_CmpFPredicate_ult : I64EnumAttrCase<"ult", 10>;
def CIR_CmpFPredicate_ule : I64EnumAttrCase<"ule", 11>;
def CIR_CmpFPredicate_une : I64EnumAttrCase<"une", 12>;
def CIR_CmpFPredicate_uno : I64EnumAttrCase<"uno", 13>;

def CIR_CmpFPredicateAttr : I64EnumAttr<
    "CmpFPredicate", "float comparison predicate",
    [CIR_CmpFPredicate_oeq, CIR_CmpFPredicate_ogt,
     CIR_CmpFPredicate_oge, CIR_CmpFPredicate_olt,
     CIR_CmpFPredicate_ole, CIR_CmpFPredicate_one,
     CIR_CmpFPredicate_ord, CIR_CmpFPredicate_ueq,
     CIR_CmpFPredicate_ugt, CIR_CmpFPredicate_uge,
     CIR_CmpFPredicate_ult, CIR_CmpFPredicate_ule,
     CIR_CmpFPredicate_une, CIR_CmpFPredicate_uno]> {
  let cppNamespace = "::cir";
}

def CIR_CmpFOp : CIR_Op<"cmpf", [Pure, SameTypeOperands]> {
  let summary = "float comparison with predicate (NaN-aware)";
  let arguments = (ins CIR_CmpFPredicateAttr:$predicate,
                       AnyFloat:$lhs, AnyFloat:$rhs);
  let results = (outs I1:$result);
  let assemblyFormat = "$predicate `,` $lhs `,` $rhs attr-dict `:` type($lhs)";
}

//===----------------------------------------------------------------------===//
// Select (ternary)
//===----------------------------------------------------------------------===//

def CIR_SelectOp : CIR_Op<"select", [Pure,
    AllTypesMatch<["trueValue", "falseValue", "result"]>]> {
  let summary = "select value based on condition (eager ternary)";
  let arguments = (ins I1:$condition, AnyType:$trueValue, AnyType:$falseValue);
  let results = (outs AnyType:$result);
  let assemblyFormat =
      "$condition `,` $trueValue `,` $falseValue attr-dict `:` type($result)";
}

//===----------------------------------------------------------------------===//
// Type casts — one op per direction, maps 1:1 to LLVM
//===----------------------------------------------------------------------===//

def CIR_ExtSIOp : CIR_IToICastOp<"extsi"> {
  let summary = "sign-extend integer (i32 -> i64)";
  let hasVerifier = 1;
}
def CIR_ExtUIOp : CIR_IToICastOp<"extui"> {
  let summary = "zero-extend integer (u32 -> u64)";
  let hasVerifier = 1;
}
def CIR_TruncIOp : CIR_IToICastOp<"trunci"> {
  let summary = "truncate integer (i64 -> i32)";
  let hasVerifier = 1;
}
def CIR_SIToFPOp : CIR_IToFCastOp<"sitofp"> {
  let summary = "signed int to float";
}
def CIR_FPToSIOp : CIR_FToICastOp<"fptosi"> {
  let summary = "float to signed int";
}
def CIR_ExtFOp : CIR_FToFCastOp<"extf"> {
  let summary = "extend float (f32 -> f64)";
  let hasVerifier = 1;
}
def CIR_TruncFOp : CIR_FToFCastOp<"truncf"> {
  let summary = "truncate float (f64 -> f32)";
  let hasVerifier = 1;
}

#endif // ARITH_OPS
