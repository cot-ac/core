// RUN: %cir-opt --cir-to-llvm %s | %FileCheck %s

// Concrete implementation.
func.func @Point_sum(%arg0: i32, %arg1: i32) -> i32 {
  %0 = cir.add %arg0, %arg1 : i32
  return %0 : i32
}

// Witness table declaration → should produce LLVM global.
// CHECK: llvm.mlir.global internal constant @Summable_Point()
// CHECK:   llvm.mlir.addressof @Point_sum
cir.witness_table @Summable_Point ["sum" = @Point_sum] {protocol = "Summable", conforming_type = "Point"}

// Test witness_method with method_index → GEP into PWT + load.
func.func @test_witness_method(%pwt: !cir.ptr) -> !cir.ptr {
  // CHECK: llvm.getelementptr
  // CHECK: llvm.load
  %fn = cir.witness_method %pwt, "sum"[0] : (!cir.ptr) -> !cir.ptr
  return %fn : !cir.ptr
}
