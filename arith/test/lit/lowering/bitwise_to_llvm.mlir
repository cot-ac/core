// RUN: %cir-opt %s --cir-to-llvm | %FileCheck %s

func.func @test_bit_and_lower(%a: i32, %b: i32) -> i32 {
  // CHECK-LABEL: llvm.func @test_bit_and_lower
  // CHECK:         llvm.and
  %r = cir.bit_and %a, %b : i32
  return %r : i32
}

func.func @test_bit_or_lower(%a: i32, %b: i32) -> i32 {
  // CHECK-LABEL: llvm.func @test_bit_or_lower
  // CHECK:         llvm.or
  %r = cir.bit_or %a, %b : i32
  return %r : i32
}

func.func @test_bit_xor_lower(%a: i32, %b: i32) -> i32 {
  // CHECK-LABEL: llvm.func @test_bit_xor_lower
  // CHECK:         llvm.xor
  %r = cir.bit_xor %a, %b : i32
  return %r : i32
}

func.func @test_bit_not_lower(%a: i32) -> i32 {
  // CHECK-LABEL: llvm.func @test_bit_not_lower
  // CHECK:         llvm.mlir.constant(-1
  // CHECK-NEXT:    llvm.xor
  %r = cir.bit_not %a : i32
  return %r : i32
}

func.func @test_shl_lower(%a: i32, %b: i32) -> i32 {
  // CHECK-LABEL: llvm.func @test_shl_lower
  // CHECK:         llvm.shl
  %r = cir.shl %a, %b : i32
  return %r : i32
}

func.func @test_shr_lower(%a: i32, %b: i32) -> i32 {
  // CHECK-LABEL: llvm.func @test_shr_lower
  // CHECK:         llvm.lshr
  %r = cir.shr %a, %b : i32
  return %r : i32
}

func.func @test_shr_s_lower(%a: i32, %b: i32) -> i32 {
  // CHECK-LABEL: llvm.func @test_shr_s_lower
  // CHECK:         llvm.ashr
  %r = cir.shr_s %a, %b : i32
  return %r : i32
}

func.func @test_cmp_lower(%a: i32, %b: i32) -> i1 {
  // CHECK-LABEL: llvm.func @test_cmp_lower
  // CHECK:         llvm.icmp "eq"
  %r = cir.cmp eq, %a, %b : i32
  return %r : i1
}

func.func @test_select_lower(%c: i1, %a: i32, %b: i32) -> i32 {
  // CHECK-LABEL: llvm.func @test_select_lower
  // CHECK:         llvm.select
  %r = cir.select %c, %a, %b : i32
  return %r : i32
}
