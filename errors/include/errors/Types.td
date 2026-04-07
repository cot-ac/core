//===- Types.td - cot-errors type definitions ----------------*- tablegen -*-===//
//
// CIR error union type: value or error code.
// Reference: Zig error unions, Rust Result<T, E>.
//
//===----------------------------------------------------------------------===//

#ifndef ERRORS_TYPES
#define ERRORS_TYPES

include "cot/CIR/CIRTypes.td"

def CIR_ErrorUnionType : CIR_Type<"ErrorUnion", "error_union"> {
  let summary = "value or error code (T or i16 error)";
  let description = [{
    An error union wrapping a payload type. `!cir.error_union<i32>` is
    "i32 or error code". Always lowers to `!llvm.struct<(T, i16)>` where
    field 0 is the payload and field 1 is the error code (0 = success).
    Reference: Zig error unions, Rust Result<T, E>.
  }];
  let parameters = (ins "::mlir::Type":$payloadType);
  let assemblyFormat = "`<` $payloadType `>`";
}

#endif // ERRORS_TYPES
