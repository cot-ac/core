//===- Ops.td - cot-slices operation definitions -------------*- tablegen -*-===//
//
// cot-slices: string_constant, slice_ptr, slice_len, slice_elem, array_to_slice.
// Reference: Rust slice ops, Go slice lowering.
//
//===----------------------------------------------------------------------===//

#ifndef SLICES_OPS
#define SLICES_OPS

include "cot/CIR/CIROps.td"
include "slices/Types.td"

//===----------------------------------------------------------------------===//
// StringConstant
//===----------------------------------------------------------------------===//

def CIR_StringConstantOp : CIR_Op<"string_constant", [Pure]> {
  let summary = "create a slice<i8> from a string literal";
  let description = [{
    Creates a constant slice pointing to a null-terminated global string.
    Lowers to: LLVM global constant + addressof + struct{ptr, len}.
    Reference: LLVM global string constants.
  }];
  let arguments = (ins StrAttr:$value);
  let results = (outs CIR_SliceType:$result);
  let assemblyFormat = "$value attr-dict `:` type($result)";
}

//===----------------------------------------------------------------------===//
// SlicePtr
//===----------------------------------------------------------------------===//

def CIR_SlicePtrOp : CIR_Op<"slice_ptr", [Pure]> {
  let summary = "extract the data pointer from a slice";
  let description = [{
    Returns the base pointer of the slice. Lowers to extractvalue [0].
  }];
  let arguments = (ins CIR_SliceType:$input);
  let results = (outs AnyType:$result);
  let assemblyFormat = "$input attr-dict `:` type($input) `to` type($result)";
}

//===----------------------------------------------------------------------===//
// SliceLen
//===----------------------------------------------------------------------===//

def CIR_SliceLenOp : CIR_Op<"slice_len", [Pure]> {
  let summary = "extract the length from a slice";
  let description = [{
    Returns the element count of the slice as i64. Lowers to extractvalue [1].
  }];
  let arguments = (ins CIR_SliceType:$input);
  let results = (outs I64:$result);
  let assemblyFormat = "$input attr-dict `:` type($input)";
}

//===----------------------------------------------------------------------===//
// SliceElem — Errata E6: NOT Pure — performs a memory read
//===----------------------------------------------------------------------===//

def CIR_SliceElemOp : CIR_Op<"slice_elem", [
    MemoryEffects<[MemRead<DefaultResource>]>
  ]> {
  let summary = "load an element from a slice by index";
  let description = [{
    Loads the element at position `idx` from the slice.
    Lowers to: extract ptr + GEP + load.
    Errata E6: NOT Pure — performs a memory read.
  }];
  let arguments = (ins CIR_SliceType:$input, AnyType:$idx);
  let results = (outs AnyType:$result);
  let assemblyFormat = "$input `,` $idx attr-dict `:` type($input) `,` type($idx) `to` type($result)";
}

//===----------------------------------------------------------------------===//
// ArrayToSlice
//===----------------------------------------------------------------------===//

def CIR_ArrayToSliceOp : CIR_Op<"array_to_slice", [Pure]> {
  let summary = "create a slice from a pointer and start/end indices";
  let description = [{
    Creates a slice from a base pointer and start/end indices.
    Lowers to: GEP(base, start) + (end - start) → struct{ptr, len}.
  }];
  let arguments = (ins AnyType:$base, AnyType:$start, AnyType:$end);
  let results = (outs CIR_SliceType:$result);
  let assemblyFormat = "$base `,` $start `,` $end attr-dict `:` `(` type($base) `,` type($start) `,` type($end) `)` `to` type($result)";
}

#endif // SLICES_OPS
