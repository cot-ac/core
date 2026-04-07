// RUN: %cir-opt --cir-to-llvm %s | %FileCheck %s

// CHECK: llvm.func @cir_test_fail(!llvm.ptr, i64)
// CHECK: llvm.mlir.global internal constant @__assert_msg_{{.*}}("should pass")

// CHECK-LABEL: llvm.func @test_assert_pass
func.func @test_assert_pass() {
  %t = cir.constant true
  // CHECK: llvm.cond_br
  // CHECK: llvm.call @cir_test_fail
  // CHECK: llvm.intr.trap
  // CHECK: llvm.unreachable
  cir.assert %t, "should pass"
  return
}
