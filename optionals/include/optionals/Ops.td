//===- Ops.td - cot-optionals operation definitions ----------*- tablegen -*-===//
//
// cot-optionals: none, wrap_optional, is_non_null, optional_payload.
// Reference: Zig optional operations, Swift Optional pattern matching.
//
//===----------------------------------------------------------------------===//

#ifndef OPTIONALS_OPS
#define OPTIONALS_OPS

include "cot/CIR/CIROps.td"
include "optionals/Types.td"

//===----------------------------------------------------------------------===//
// None — create a null optional
//===----------------------------------------------------------------------===//

def CIR_NoneOp : CIR_Op<"none", [Pure]> {
  let summary = "create a null optional value";
  let description = [{
    Creates a null/none optional. For pointer-like payloads, lowers to a
    null pointer. For value payloads, lowers to {undef, i1 0}.
    Reference: Zig null, Swift Optional.none.
  }];
  let results = (outs CIR_OptionalType:$result);
  let assemblyFormat = "attr-dict `:` qualified(type($result))";
}

//===----------------------------------------------------------------------===//
// WrapOptional — wrap a value into an optional
//===----------------------------------------------------------------------===//

def CIR_WrapOptionalOp : CIR_Op<"wrap_optional", [Pure]> {
  let summary = "wrap a value into an optional";
  let description = [{
    Wraps a non-null value into an optional. For pointer-like payloads,
    this is an identity (the pointer IS the optional). For value payloads,
    lowers to {val, i1 1}.
    Reference: Zig implicit wrap, Swift Optional.some().
  }];
  let arguments = (ins AnyType:$input);
  let results = (outs CIR_OptionalType:$result);
  let assemblyFormat = "$input attr-dict `:` type($input) `to` qualified(type($result))";
}

//===----------------------------------------------------------------------===//
// IsNonNull — check if optional has a value
//===----------------------------------------------------------------------===//

def CIR_IsNonNullOp : CIR_Op<"is_non_null", [Pure]> {
  let summary = "check if optional contains a value (not null)";
  let description = [{
    Returns true (i1 1) if the optional contains a value, false if null.
    For pointer-like payloads, lowers to icmp ne null.
    For value payloads, lowers to extractvalue [1] (the has_value flag).
    Reference: Zig != null, Swift != nil.
  }];
  let arguments = (ins CIR_OptionalType:$input);
  let results = (outs I1:$result);
  let hasCustomAssemblyFormat = 1;
}

//===----------------------------------------------------------------------===//
// OptionalPayload — extract the value from an optional
//===----------------------------------------------------------------------===//

def CIR_OptionalPayloadOp : CIR_Op<"optional_payload", [Pure]> {
  let summary = "extract the payload value from a non-null optional";
  let description = [{
    Extracts the wrapped value. Undefined behavior if the optional is null.
    For pointer-like payloads, this is an identity. For value payloads,
    lowers to extractvalue [0].
    Reference: Zig orelse/unwrap, Swift force unwrap (!).
  }];
  let arguments = (ins CIR_OptionalType:$input);
  let results = (outs AnyType:$result);
  let assemblyFormat = "$input attr-dict `:` qualified(type($input)) `to` type($result)";
}

#endif // OPTIONALS_OPS
