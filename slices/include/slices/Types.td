//===- Types.td - cot-slices type definitions ----------------*- tablegen -*-===//
//
// CIR slice type: fat pointer {ptr, len}.
// Reference: Rust slice, Go slice.
//
//===----------------------------------------------------------------------===//

#ifndef SLICES_TYPES
#define SLICES_TYPES

include "cot/CIR/CIRTypes.td"

def CIR_SliceType : CIR_Type<"Slice", "slice"> {
  let summary = "fat pointer {ptr, len} — a view into contiguous memory";
  let description = [{
    A slice is a (pointer, length) pair providing a view into memory.
    Syntax: `!cir.slice<i32>` — a slice of i32 values.
    Lowers to `!llvm.struct<(!llvm.ptr, i64)>`.
  }];
  let parameters = (ins "::mlir::Type":$elementType);
  let assemblyFormat = "`<` $elementType `>`";
}

#endif // SLICES_TYPES
