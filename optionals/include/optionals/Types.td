//===- Types.td - cot-optionals type definitions -------------*- tablegen -*-===//
//
// CIR optional type: nullable wrapper around any payload type.
// Reference: Zig optional types, Swift Optional<T>.
//
//===----------------------------------------------------------------------===//

#ifndef OPTIONALS_TYPES
#define OPTIONALS_TYPES

include "cot/CIR/CIRTypes.td"

def CIR_OptionalType : CIR_Type<"Optional", "optional"> {
  let summary = "nullable value (T or null)";
  let description = [{
    An optional type wrapping a payload. `!cir.optional<i32>` is "i32 or null".
    Pointer-like payloads (those implementing PointerLikeTypeInterface) use
    null-pointer optimization (lower to `!llvm.ptr`).
    Value payloads lower to `!llvm.struct<(T, i1)>` with a has_value flag.
    Reference: Zig ?T, Swift Optional<T>.
  }];
  let parameters = (ins "::mlir::Type":$payloadType);
  let assemblyFormat = "`<` $payloadType `>`";

  let extraClassDeclaration = [{
    /// Returns true if the payload type is pointer-like (implements
    /// PointerLikeTypeInterface), enabling null-pointer optimization.
    bool isPointerLike() const;
  }];
}

#endif // OPTIONALS_TYPES
