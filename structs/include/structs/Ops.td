//===- Ops.td - cot-structs operation definitions ------------*- tablegen -*-===//
//
// cot-structs: struct_init, field_val, field_ptr.
// Reference: LLVM insertvalue/extractvalue/getelementptr.
//
//===----------------------------------------------------------------------===//

#ifndef STRUCTS_OPS
#define STRUCTS_OPS

include "cot/CIR/CIROps.td"
include "structs/Types.td"

//===----------------------------------------------------------------------===//
// StructInit
//===----------------------------------------------------------------------===//

def CIR_StructInitOp : CIR_Op<"struct_init", [Pure]> {
  let summary = "initialize a struct from field values";
  let description = [{
    Creates a struct value from its constituent fields. Field count and
    types must match the result struct type's definition.
    Lowers to undef + insertvalue chain.
    Reference: LLVM insertvalue.
  }];
  let arguments = (ins Variadic<AnyType>:$fields);
  let results = (outs CIR_StructType:$result);
  let hasCustomAssemblyFormat = 1;
}

//===----------------------------------------------------------------------===//
// FieldVal
//===----------------------------------------------------------------------===//

def CIR_FieldValOp : CIR_Op<"field_val", [Pure]> {
  let summary = "extract a field value from a struct by index";
  let description = [{
    Extracts the field at `index` from a struct value.
    Lowers to llvm.extractvalue.
    Reference: LLVM extractvalue.
  }];
  let arguments = (ins CIR_StructType:$input, I64Attr:$index);
  let results = (outs AnyType:$result);
  let assemblyFormat = "$input `,` $index attr-dict `:` type($input) `to` type($result)";
}

//===----------------------------------------------------------------------===//
// FieldPtr
//===----------------------------------------------------------------------===//

def CIR_FieldPtrOp : CIR_Op<"field_ptr", [Pure]> {
  let summary = "get a pointer to a struct field by index";
  let description = [{
    Computes the address of field `index` in a struct pointed to by `base`.
    The `struct_type` attribute tells GEP the struct layout.
    Lowers to llvm.getelementptr [0, idx].
    Reference: LLVM getelementptr.
  }];
  let arguments = (ins AnyType:$base, I64Attr:$index,
                       TypeAttr:$struct_type);
  let results = (outs AnyType:$result);
  let assemblyFormat = "$base `,` $index `,` $struct_type attr-dict `:` type($base) `to` type($result)";
}

#endif // STRUCTS_OPS
