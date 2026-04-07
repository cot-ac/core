//===- Ops.td - cot-arrays operation definitions -------------*- tablegen -*-===//
//
// cot-arrays: array_init, elem_val, elem_ptr.
// Reference: LLVM insertvalue/extractvalue/getelementptr.
//
//===----------------------------------------------------------------------===//

#ifndef ARRAYS_OPS
#define ARRAYS_OPS

include "cot/CIR/CIROps.td"
include "arrays/Types.td"

//===----------------------------------------------------------------------===//
// ArrayInit
//===----------------------------------------------------------------------===//

def CIR_ArrayInitOp : CIR_Op<"array_init", [Pure]> {
  let summary = "initialize an array from element values";
  let description = [{
    Creates an array value from its elements. Element count must match
    the result array type's size. Lowers to undef + insertvalue chain.
    Reference: LLVM insertvalue.
  }];
  let arguments = (ins Variadic<AnyType>:$elements);
  let results = (outs CIR_ArrayType:$result);
  let hasCustomAssemblyFormat = 1;
}

//===----------------------------------------------------------------------===//
// ElemVal
//===----------------------------------------------------------------------===//

def CIR_ElemValOp : CIR_Op<"elem_val", [Pure]> {
  let summary = "extract an element value from an array by index";
  let description = [{
    Extracts element at `index` from an array value.
    Lowers to llvm.extractvalue.
    Reference: LLVM extractvalue.
  }];
  let arguments = (ins CIR_ArrayType:$input, I64Attr:$index);
  let results = (outs AnyType:$result);
  let assemblyFormat = "$input `,` $index attr-dict `:` type($input) `to` type($result)";
}

//===----------------------------------------------------------------------===//
// ElemPtr
//===----------------------------------------------------------------------===//

def CIR_ElemPtrOp : CIR_Op<"elem_ptr", [Pure]> {
  let summary = "get a pointer to an array element by dynamic index";
  let description = [{
    Computes the address of element `idx` in an array pointed to by `base`.
    The `array_type` attribute tells GEP the array layout.
    Lowers to llvm.getelementptr.
    Reference: LLVM getelementptr.
  }];
  let arguments = (ins AnyType:$base, AnyType:$idx,
                       TypeAttr:$array_type);
  let results = (outs AnyType:$result);
  let assemblyFormat = "$base `,` $idx `,` $array_type attr-dict `:` `(` type($base) `,` type($idx) `)` `to` type($result)";
}

#endif // ARRAYS_OPS
