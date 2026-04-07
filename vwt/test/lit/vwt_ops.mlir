// RUN: %cir-opt %s | %FileCheck %s

// Test: VWT ops roundtrip correctly.

// CHECK: func.func @query_vwt(%arg0: !cir.ptr)
func.func @query_vwt(%vwt: !cir.ptr) -> i64 {
  // CHECK: cir.vwt_size %arg0 : !cir.ptr -> i64
  %size = cir.vwt_size %vwt : !cir.ptr -> i64

  // CHECK: cir.vwt_stride %arg0 : !cir.ptr -> i64
  %stride = cir.vwt_stride %vwt : !cir.ptr -> i64

  // CHECK: cir.vwt_align %arg0 : !cir.ptr -> i64
  %align = cir.vwt_align %vwt : !cir.ptr -> i64

  return %size : i64
}

// CHECK: func.func @vwt_memory_ops
func.func @vwt_memory_ops(%vwt: !cir.ptr, %src: !cir.ptr, %dst: !cir.ptr) {
  // CHECK: cir.vwt_copy %arg0, %arg1, %arg2
  cir.vwt_copy %vwt, %src, %dst : !cir.ptr, !cir.ptr, !cir.ptr

  // CHECK: cir.vwt_destroy %arg0, %arg1
  cir.vwt_destroy %vwt, %src : !cir.ptr, !cir.ptr

  // CHECK: cir.vwt_move %arg0, %arg1, %arg2
  cir.vwt_move %vwt, %src, %dst : !cir.ptr, !cir.ptr, !cir.ptr

  // CHECK: cir.vwt_init_buffer %arg0, %arg1, %arg2
  %buf = cir.vwt_init_buffer %vwt, %src, %dst : !cir.ptr, !cir.ptr, !cir.ptr -> !cir.ptr

  return
}
