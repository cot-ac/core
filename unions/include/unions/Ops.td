//===- Ops.td - unions operation definitions ---------------*- tablegen -*-===//
//
// unions: union_init, union_tag, union_payload.
// Reference: Rust enum pattern matching, Swift switch on enum.
//
//===----------------------------------------------------------------------===//

#ifndef UNIONS_OPS
#define UNIONS_OPS

include "cot/CIR/CIROps.td"
include "unions/Types.td"

//===----------------------------------------------------------------------===//
// UnionInit — create a tagged union value
//===----------------------------------------------------------------------===//

def CIR_UnionInitOp : CIR_Op<"union_init", [Pure]> {
  let summary = "create a tagged union value from a variant";
  let description = [{
    Creates a tagged union with the given variant tag and optional payload.
    Lowers to: undef + insertvalue tag + bitcast payload into byte array.
    Reference: Rust enum variant construction.
  }];
  let arguments = (ins StrAttr:$variant, Optional<AnyType>:$payload);
  let results = (outs CIR_TaggedUnionType:$result);
  let hasCustomAssemblyFormat = 1;
}

//===----------------------------------------------------------------------===//
// UnionTag — extract the discriminator tag
//===----------------------------------------------------------------------===//

def CIR_UnionTagOp : CIR_Op<"union_tag", [Pure]> {
  let summary = "extract the i8 discriminator tag from a tagged union";
  let description = [{
    Returns the tag byte identifying which variant is active.
    Lowers to extractvalue [0].
    Reference: Rust discriminant.
  }];
  let arguments = (ins CIR_TaggedUnionType:$input);
  let results = (outs I8:$result);
  let hasCustomAssemblyFormat = 1;
}

//===----------------------------------------------------------------------===//
// UnionPayload — extract the payload for a specific variant
//===----------------------------------------------------------------------===//

def CIR_UnionPayloadOp : CIR_Op<"union_payload", [Pure]> {
  let summary = "extract the payload value for a specific variant";
  let description = [{
    Extracts the payload assuming the given variant is active.
    Undefined behavior if the wrong variant is active.
    Lowers to: extractvalue [1] (byte array) + bitcast to variant type.
    Reference: Rust pattern match payload extraction.
  }];
  let arguments = (ins StrAttr:$variant, CIR_TaggedUnionType:$input);
  let results = (outs AnyType:$result);
  let hasCustomAssemblyFormat = 1;
}

#endif // UNIONS_OPS
