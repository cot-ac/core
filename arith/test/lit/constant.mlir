// RUN: %cir-opt %s | %FileCheck %s

func.func @test_constant_i32() -> i32 {
  // CHECK-LABEL: func @test_constant_i32
  // CHECK-NEXT:    %[[C:.*]] = cir.constant 42 : i32
  // CHECK-NEXT:    return %[[C]] : i32
  %c = cir.constant 42 : i32
  return %c : i32
}

func.func @test_constant_i64() -> i64 {
  // CHECK-LABEL: func @test_constant_i64
  // CHECK-NEXT:    %[[C:.*]] = cir.constant 100 : i64
  %c = cir.constant 100 : i64
  return %c : i64
}

func.func @test_constant_i1() -> i1 {
  // CHECK-LABEL: func @test_constant_i1
  // CHECK-NEXT:    %[[C:.*]] = cir.constant true
  %c = cir.constant true
  return %c : i1
}
