// RUN: %cir-opt %s --cir-to-llvm | %FileCheck %s

func.func @test_add_lower(%a: i32, %b: i32) -> i32 {
  // CHECK-LABEL: llvm.func @test_add_lower
  // CHECK:         llvm.add
  %r = cir.add %a, %b : i32
  return %r : i32
}

func.func @test_sub_lower(%a: i32, %b: i32) -> i32 {
  // CHECK-LABEL: llvm.func @test_sub_lower
  // CHECK:         llvm.sub
  %r = cir.sub %a, %b : i32
  return %r : i32
}

func.func @test_mul_lower(%a: i32, %b: i32) -> i32 {
  // CHECK-LABEL: llvm.func @test_mul_lower
  // CHECK:         llvm.mul
  %r = cir.mul %a, %b : i32
  return %r : i32
}

func.func @test_divsi_lower(%a: i32, %b: i32) -> i32 {
  // CHECK-LABEL: llvm.func @test_divsi_lower
  // CHECK:         llvm.sdiv
  %r = cir.divsi %a, %b : i32
  return %r : i32
}

func.func @test_divf_lower(%a: f32, %b: f32) -> f32 {
  // CHECK-LABEL: llvm.func @test_divf_lower
  // CHECK:         llvm.fdiv
  %r = cir.divf %a, %b : f32
  return %r : f32
}

func.func @test_neg_lower(%a: i32) -> i32 {
  // CHECK-LABEL: llvm.func @test_neg_lower
  // CHECK:         llvm.mlir.constant(0
  // CHECK-NEXT:    llvm.sub
  %r = cir.neg %a : i32
  return %r : i32
}

func.func @test_negf_lower(%a: f32) -> f32 {
  // CHECK-LABEL: llvm.func @test_negf_lower
  // CHECK:         llvm.fneg
  %r = cir.negf %a : f32
  return %r : f32
}

func.func @test_constant_lower() -> i32 {
  // CHECK-LABEL: llvm.func @test_constant_lower
  // CHECK:         llvm.mlir.constant(42 : i32)
  %c = cir.constant 42 : i32
  return %c : i32
}
