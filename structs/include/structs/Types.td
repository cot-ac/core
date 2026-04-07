//===- Types.td - cot-structs type definitions ---------------*- tablegen -*-===//
//
// CIR struct type: named struct with typed fields.
// Reference: LLVM IR struct types, Swift SIL struct.
//
//===----------------------------------------------------------------------===//

#ifndef STRUCTS_TYPES
#define STRUCTS_TYPES

include "cot/CIR/CIRTypes.td"

def CIR_StructType : CIR_Type<"Struct", "struct"> {
  let summary = "named struct with typed fields";
  let description = [{
    A named struct type with named, typed fields.
    Syntax: `!cir.struct<"Point", "x": i32, "y": i32>`
    Lowers to `!llvm.struct<(i32, i32)>` (literal LLVM struct).
  }];
  let parameters = (ins
    "::mlir::StringAttr":$name,
    ArrayRefParameter<"::mlir::StringAttr", "field names">:$fieldNames,
    ArrayRefParameter<"::mlir::Type", "field types">:$fieldTypes
  );
  let hasCustomAssemblyFormat = 1;
}

#endif // STRUCTS_TYPES
