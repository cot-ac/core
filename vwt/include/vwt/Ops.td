//===- Ops.td - VWT operation definitions ------------------*- tablegen -*-===//
//
// Value Witness Table ops: runtime queries for type size, stride,
// alignment, and memory operations (copy, destroy, move).
// VWT is a struct of function pointers + data, indexed by known offsets.
// Reference: Swift include/swift/ABI/ValueWitnessTable.h
//
// VWT struct layout (12 entries):
//   [0] initBufferWithCopy  fn ptr
//   [1] destroy             fn ptr
//   [2] initializeWithCopy  fn ptr
//   [3] assignWithCopy      fn ptr
//   [4] initializeWithTake  fn ptr
//   [5] assignWithTake      fn ptr
//   [6] getEnumTagSingle    fn ptr
//   [7] storeEnumTagSingle  fn ptr
//   [8] size                i64
//   [9] stride              i64
//   [10] flags              i32 (alignment mask in low bits)
//   [11] extraInhabitants   i32
//
//===----------------------------------------------------------------------===//

#ifndef VWT_OPS
#define VWT_OPS

include "cot/CIR/CIROps.td"

//===----------------------------------------------------------------------===//
// Data queries — load fields from the VWT struct
//===----------------------------------------------------------------------===//

def CIR_VWTSizeOp : CIR_Op<"vwt_size", [Pure]> {
  let summary = "get type size from value witness table";
  let description = [{
    Loads the size field (index 8) from a Value Witness Table pointer.
    Returns the number of bytes needed to store a value of this type.
  }];
  let arguments = (ins AnyType:$vwt);
  let results = (outs AnyType:$result);
  let assemblyFormat = "$vwt attr-dict `:` type($vwt) `->` type($result)";
}

def CIR_VWTStrideOp : CIR_Op<"vwt_stride", [Pure]> {
  let summary = "get type stride from value witness table";
  let description = [{
    Loads the stride field (index 9) from a Value Witness Table pointer.
    Returns the distance between consecutive elements in an array.
  }];
  let arguments = (ins AnyType:$vwt);
  let results = (outs AnyType:$result);
  let assemblyFormat = "$vwt attr-dict `:` type($vwt) `->` type($result)";
}

def CIR_VWTAlignOp : CIR_Op<"vwt_align", [Pure]> {
  let summary = "get type alignment from value witness table";
  let description = [{
    Loads the flags field (index 10), masks the low bits to get
    the alignment mask, then adds 1 to get the alignment value.
  }];
  let arguments = (ins AnyType:$vwt);
  let results = (outs AnyType:$result);
  let assemblyFormat = "$vwt attr-dict `:` type($vwt) `->` type($result)";
}

//===----------------------------------------------------------------------===//
// Function calls — load fn ptr from VWT, call indirectly
//===----------------------------------------------------------------------===//

def CIR_VWTCopyOp : CIR_Op<"vwt_copy", []> {
  let summary = "copy a value using the VWT initializeWithCopy witness";
  let description = [{
    Loads the initializeWithCopy function pointer (index 2) from the VWT,
    then calls it indirectly: fn(dest, src, metadata) → dest.
    For trivial types this is memcpy. For reference types this is retain.
  }];
  let arguments = (ins AnyType:$vwt, AnyType:$src, AnyType:$dst);
  let assemblyFormat = [{
    $vwt `,` $src `,` $dst attr-dict
    `:` type($vwt) `,` type($src) `,` type($dst)
  }];
}

def CIR_VWTDestroyOp : CIR_Op<"vwt_destroy", []> {
  let summary = "destroy a value using the VWT destroy witness";
  let description = [{
    Loads the destroy function pointer (index 1) from the VWT,
    then calls it indirectly: fn(ptr, metadata).
    For trivial types this is a no-op. For reference types this is release.
  }];
  let arguments = (ins AnyType:$vwt, AnyType:$ptr);
  let assemblyFormat = [{
    $vwt `,` $ptr attr-dict `:` type($vwt) `,` type($ptr)
  }];
}

def CIR_VWTMoveOp : CIR_Op<"vwt_move", []> {
  let summary = "move a value using the VWT initializeWithTake witness";
  let description = [{
    Loads the initializeWithTake function pointer (index 4) from the VWT,
    then calls it indirectly: fn(dest, src, metadata) → dest.
    Transfers ownership without retain/release.
  }];
  let arguments = (ins AnyType:$vwt, AnyType:$src, AnyType:$dst);
  let assemblyFormat = [{
    $vwt `,` $src `,` $dst attr-dict
    `:` type($vwt) `,` type($src) `,` type($dst)
  }];
}

def CIR_VWTInitBufferOp : CIR_Op<"vwt_init_buffer", []> {
  let summary = "initialize existential buffer using VWT witness";
  let description = [{
    Loads the initializeBufferWithCopyOfBuffer function pointer (index 0)
    from the VWT, then calls it indirectly: fn(dest_buf, src_buf, metadata) → ptr.
    Handles inline vs heap allocation of the existential buffer.
  }];
  let arguments = (ins AnyType:$vwt, AnyType:$src, AnyType:$dst);
  let results = (outs AnyType:$result);
  let assemblyFormat = [{
    $vwt `,` $src `,` $dst attr-dict
    `:` type($vwt) `,` type($src) `,` type($dst) `->` type($result)
  }];
}

#endif // VWT_OPS
