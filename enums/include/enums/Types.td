//===- Types.td - enums type definitions -------------------*- tablegen -*-===//
//
// CIR enum type: named enumeration with integer tag.
// Reference: Rust enum (C-like), C/C++ enum.
//
//===----------------------------------------------------------------------===//

#ifndef ENUMS_TYPES
#define ENUMS_TYPES

include "cot/CIR/CIRTypes.td"

def CIR_EnumType : CIR_Type<"Enum", "enum"> {
  let summary = "named enumeration with integer tag";
  let description = [{
    A named enum type with variant names and an underlying tag type.
    `!cir.enum<"Color", i32, "Red", "Green", "Blue">` represents an
    enum "Color" backed by i32 with variants Red=0, Green=1, Blue=2.
    Lowers directly to the tag type (enum IS the integer).
    Reference: Rust C-like enum, C/C++ enum.
  }];
  let parameters = (ins
    StringRefParameter<"enum name">:$name,
    "::mlir::Type":$tagType,
    ArrayRefParameter<"::mlir::StringAttr", "variant names">:$variants
  );
  let hasCustomAssemblyFormat = 1;
}

#endif // ENUMS_TYPES
