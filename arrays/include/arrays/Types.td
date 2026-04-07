//===- Types.td - cot-arrays type definitions ----------------*- tablegen -*-===//
//
// CIR array type: fixed-size array of homogeneous elements.
// Reference: LLVM IR array types.
//
//===----------------------------------------------------------------------===//

#ifndef ARRAYS_TYPES
#define ARRAYS_TYPES

include "cot/CIR/CIRTypes.td"

def CIR_ArrayType : CIR_Type<"Array", "array"> {
  let summary = "fixed-size array of homogeneous elements";
  let description = [{
    A fixed-size array: `!cir.array<4 x i32>` holds exactly 4 i32 values.
    Lowers to `!llvm.array<4 x i32>`.
  }];
  let parameters = (ins
    "int64_t":$size,
    "::mlir::Type":$elementType
  );
  let assemblyFormat = "`<` $size `x` $elementType `>`";
}

#endif // ARRAYS_TYPES
