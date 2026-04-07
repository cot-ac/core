//===- Ops.td - enums operation definitions ----------------*- tablegen -*-===//
//
// enums: enum_constant, enum_value.
// Reference: Rust C-like enum lowering, MLIR ConversionPatterns.
//
//===----------------------------------------------------------------------===//

#ifndef ENUMS_OPS
#define ENUMS_OPS

include "cot/CIR/CIROps.td"
include "enums/Types.td"

//===----------------------------------------------------------------------===//
// EnumConstant — produce an enum value from a variant name
//===----------------------------------------------------------------------===//

def CIR_EnumConstantOp : CIR_Op<"enum_constant", [Pure]> {
  let summary = "create an enum value from a variant name";
  let description = [{
    Produces the enum value for the given variant. The variant attribute
    is a StringAttr naming the variant. Lowers to the variant's integer
    index as an LLVM constant.
    Reference: Rust enum variant, C enum constant.
  }];
  let arguments = (ins StrAttr:$variant);
  let results = (outs CIR_EnumType:$result);
  let assemblyFormat = "$variant attr-dict `:` qualified(type($result))";
}

//===----------------------------------------------------------------------===//
// EnumValue — extract the integer tag from an enum
//===----------------------------------------------------------------------===//

def CIR_EnumValueOp : CIR_Op<"enum_value", [Pure]> {
  let summary = "extract the integer tag value from an enum";
  let description = [{
    Returns the underlying integer tag of an enum value.
    Identity after lowering — the enum IS the integer.
    Reference: Rust `as` cast on C-like enum.
  }];
  let arguments = (ins CIR_EnumType:$input);
  let results = (outs AnyType:$result);
  let hasCustomAssemblyFormat = 1;
}

#endif // ENUMS_OPS
