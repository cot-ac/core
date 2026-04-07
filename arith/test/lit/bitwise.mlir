// RUN: %cir-opt %s | %FileCheck %s

func.func @test_bit_and(%a: i32, %b: i32) -> i32 {
  // CHECK-LABEL: func @test_bit_and
  // CHECK-NEXT:    %[[R:.*]] = cir.bit_and %arg0, %arg1 : i32
  %r = cir.bit_and %a, %b : i32
  return %r : i32
}

func.func @test_bit_or(%a: i32, %b: i32) -> i32 {
  // CHECK-LABEL: func @test_bit_or
  // CHECK-NEXT:    %[[R:.*]] = cir.bit_or %arg0, %arg1 : i32
  %r = cir.bit_or %a, %b : i32
  return %r : i32
}

func.func @test_bit_xor(%a: i32, %b: i32) -> i32 {
  // CHECK-LABEL: func @test_bit_xor
  // CHECK-NEXT:    %[[R:.*]] = cir.bit_xor %arg0, %arg1 : i32
  %r = cir.bit_xor %a, %b : i32
  return %r : i32
}

func.func @test_bit_not(%a: i32) -> i32 {
  // CHECK-LABEL: func @test_bit_not
  // CHECK-NEXT:    %[[R:.*]] = cir.bit_not %arg0 : i32
  %r = cir.bit_not %a : i32
  return %r : i32
}

func.func @test_shl(%a: i32, %b: i32) -> i32 {
  // CHECK-LABEL: func @test_shl
  // CHECK-NEXT:    %[[R:.*]] = cir.shl %arg0, %arg1 : i32
  %r = cir.shl %a, %b : i32
  return %r : i32
}

func.func @test_shr(%a: i32, %b: i32) -> i32 {
  // CHECK-LABEL: func @test_shr
  // CHECK-NEXT:    %[[R:.*]] = cir.shr %arg0, %arg1 : i32
  %r = cir.shr %a, %b : i32
  return %r : i32
}

func.func @test_shr_s(%a: i32, %b: i32) -> i32 {
  // CHECK-LABEL: func @test_shr_s
  // CHECK-NEXT:    %[[R:.*]] = cir.shr_s %arg0, %arg1 : i32
  %r = cir.shr_s %a, %b : i32
  return %r : i32
}
