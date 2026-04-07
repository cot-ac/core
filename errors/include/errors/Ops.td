//===- Ops.td - cot-errors operation definitions -------------*- tablegen -*-===//
//
// cot-errors: wrap_result, wrap_error, is_error, error_payload, error_code.
// Reference: Zig error unions, Rust Result pattern matching.
//
//===----------------------------------------------------------------------===//

#ifndef ERRORS_OPS
#define ERRORS_OPS

include "cot/CIR/CIROps.td"
include "errors/Types.td"

//===----------------------------------------------------------------------===//
// WrapResult — wrap a success value into an error union
//===----------------------------------------------------------------------===//

def CIR_WrapResultOp : CIR_Op<"wrap_result", [Pure]> {
  let summary = "wrap a success value into an error union";
  let description = [{
    Wraps a value as a successful result with error code 0.
    Lowers to {val, i16 0}.
    Reference: Zig implicit wrap, Rust Ok(val).
  }];
  let arguments = (ins AnyType:$input);
  let results = (outs CIR_ErrorUnionType:$result);
  let assemblyFormat = "$input attr-dict `:` type($input) `to` qualified(type($result))";
}

//===----------------------------------------------------------------------===//
// WrapError — wrap an error code into an error union
//===----------------------------------------------------------------------===//

def CIR_WrapErrorOp : CIR_Op<"wrap_error", [Pure]> {
  let summary = "wrap an error code into an error union";
  let description = [{
    Creates an error result with the given error code and undefined payload.
    Lowers to {undef, code}.
    Reference: Zig error return, Rust Err(code).
  }];
  let arguments = (ins I16:$code);
  let results = (outs CIR_ErrorUnionType:$result);
  let assemblyFormat = "$code attr-dict `:` qualified(type($result))";
}

//===----------------------------------------------------------------------===//
// IsError — check if error union contains an error
//===----------------------------------------------------------------------===//

def CIR_IsErrorOp : CIR_Op<"is_error", [Pure]> {
  let summary = "check if error union contains an error (code != 0)";
  let description = [{
    Returns true (i1 1) if the error union contains an error (code != 0).
    Lowers to extractvalue [1] + icmp ne 0.
    Reference: Zig catch, Rust is_err().
  }];
  let arguments = (ins CIR_ErrorUnionType:$input);
  let results = (outs I1:$result);
  let hasCustomAssemblyFormat = 1;
}

//===----------------------------------------------------------------------===//
// ErrorPayload — extract the success value from an error union
//===----------------------------------------------------------------------===//

def CIR_ErrorPayloadOp : CIR_Op<"error_payload", [Pure]> {
  let summary = "extract the payload value from a successful error union";
  let description = [{
    Extracts the wrapped value. Undefined behavior if the error union
    contains an error. Lowers to extractvalue [0].
    Reference: Zig catch/orelse, Rust unwrap().
  }];
  let arguments = (ins CIR_ErrorUnionType:$input);
  let results = (outs AnyType:$result);
  let assemblyFormat = "$input attr-dict `:` qualified(type($input)) `to` type($result)";
}

//===----------------------------------------------------------------------===//
// ErrorCode — extract the error code from an error union
//===----------------------------------------------------------------------===//

def CIR_ErrorCodeOp : CIR_Op<"error_code", [Pure]> {
  let summary = "extract the error code from an error union";
  let description = [{
    Extracts the i16 error code. Returns 0 if the error union contains a
    success value. Lowers to extractvalue [1].
    Reference: Zig error value, Rust err().
  }];
  let arguments = (ins CIR_ErrorUnionType:$input);
  let results = (outs I16:$result);
  let hasCustomAssemblyFormat = 1;
}

#endif // ERRORS_OPS
