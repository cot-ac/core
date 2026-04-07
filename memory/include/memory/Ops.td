//===- Ops.td - cot-memory operation definitions -------------*- tablegen -*-===//
//
// cot-memory: alloca, store, load, addr_of, deref.
// Reference: LLVM IR memory ops, Flang FIR alloca/load/store.
//
//===----------------------------------------------------------------------===//

#ifndef MEMORY_OPS
#define MEMORY_OPS

include "cot/CIR/CIROps.td"
include "memory/Types.td"

//===----------------------------------------------------------------------===//
// Alloca
//===----------------------------------------------------------------------===//

def CIR_AllocaOp : CIR_Op<"alloca", [
    MemoryEffects<[MemAlloc<DefaultResource>]>
  ]> {
  let summary = "stack-allocate a slot for the given type";
  let description = [{
    Allocates a stack slot for one value of `elem_type`. Returns an opaque
    pointer to the slot. Reference: LLVM alloca, FIR fir.alloca.
  }];
  let arguments = (ins TypeAttr:$elem_type);
  let results = (outs CIR_PointerType:$result);
  let assemblyFormat = "$elem_type attr-dict `:` type($result)";
}

//===----------------------------------------------------------------------===//
// Store
//===----------------------------------------------------------------------===//

def CIR_StoreOp : CIR_Op<"store", [
    MemoryEffects<[MemWrite<DefaultResource>]>
  ]> {
  let summary = "store a value to memory through a pointer";
  let description = [{
    Writes `value` to the address `addr`. Errata E7: addr constrained to
    CIR_PointerType. Reference: LLVM store, FIR fir.store.
  }];
  let arguments = (ins AnyType:$value, CIR_PointerType:$addr);
  let assemblyFormat = "$value `,` $addr attr-dict `:` type($value) `,` type($addr)";
}

//===----------------------------------------------------------------------===//
// Load
//===----------------------------------------------------------------------===//

def CIR_LoadOp : CIR_Op<"load", [
    MemoryEffects<[MemRead<DefaultResource>]>
  ]> {
  let summary = "load a value from memory through a pointer";
  let description = [{
    Reads a value from address `addr`. Result type must match what was
    stored. Errata E7: addr constrained to CIR_PointerType.
    Reference: LLVM load, FIR fir.load.
  }];
  let arguments = (ins CIR_PointerType:$addr);
  let results = (outs AnyType:$result);
  let assemblyFormat = "$addr attr-dict `:` type($addr) `to` type($result)";
}

//===----------------------------------------------------------------------===//
// AddrOf
//===----------------------------------------------------------------------===//

def CIR_AddrOfOp : CIR_Op<"addr_of", [Pure]> {
  let summary = "convert a raw pointer to a typed safe reference";
  let description = [{
    Wraps a raw pointer in a typed reference. Zero-cost: both lower to
    !llvm.ptr. The reference carries the pointee type for verification.
  }];
  let arguments = (ins CIR_PointerType:$addr);
  let results = (outs CIR_RefType:$result);
  let assemblyFormat = "$addr attr-dict `:` type($addr) `to` type($result)";
}

//===----------------------------------------------------------------------===//
// Deref
//===----------------------------------------------------------------------===//

def CIR_DerefOp : CIR_Op<"deref", [
    MemoryEffects<[MemRead<DefaultResource>]>
  ]> {
  let summary = "dereference a typed reference to get the value";
  let description = [{
    Loads the value through a typed reference. Errata E6: NOT Pure —
    performs a memory read. Reference: Rust deref, Swift load.
  }];
  let arguments = (ins CIR_RefType:$ref);
  let results = (outs AnyType:$result);
  let assemblyFormat = "$ref attr-dict `:` type($ref) `to` type($result)";
}

#endif // MEMORY_OPS
