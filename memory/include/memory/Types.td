//===- Types.td - cot-memory type definitions ----------------*- tablegen -*-===//
//
// CIR pointer and reference types for memory operations.
// Reference: LLVM IR opaque pointers, Swift SIL address types.
//
//===----------------------------------------------------------------------===//

#ifndef MEMORY_TYPES
#define MEMORY_TYPES

include "cot/CIR/CIRTypes.td"
include "cot/CIR/CIRInterfaces.td"

def CIR_PointerType : CIR_Type<"Pointer", "ptr",
    [CIR_PointerLikeTypeInterface]> {
  let summary = "opaque pointer (address of stack/heap value)";
  let description = [{
    Opaque pointer — no element type. Matches LLVM's modern opaque ptr.
    Used by cir.alloca, cir.load, cir.store.
    Lowers to !llvm.ptr.
  }];
}

def CIR_RefType : CIR_Type<"Ref", "ref",
    [CIR_PointerLikeTypeInterface]> {
  let summary = "typed safe reference (!cir.ref<T> — non-null, known pointee)";
  let description = [{
    A safe, non-null reference to a value of type T. Carries the pointee
    type for verification. Lowers to !llvm.ptr — zero runtime cost.
    Use !cir.ref<T> for safe code. Use !cir.ptr for C interop.
  }];
  let parameters = (ins "mlir::Type":$pointeeType);
  let assemblyFormat = "`<` $pointeeType `>`";
}

#endif // MEMORY_TYPES
