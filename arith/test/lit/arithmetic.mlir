// RUN: %cir-opt %s | %FileCheck %s

func.func @test_add(%a: i32, %b: i32) -> i32 {
  // CHECK-LABEL: func @test_add
  // CHECK-NEXT:    %[[R:.*]] = cir.add %arg0, %arg1 : i32
  // CHECK-NEXT:    return %[[R]] : i32
  %r = cir.add %a, %b : i32
  return %r : i32
}

func.func @test_sub(%a: i32, %b: i32) -> i32 {
  // CHECK-LABEL: func @test_sub
  // CHECK-NEXT:    %[[R:.*]] = cir.sub %arg0, %arg1 : i32
  %r = cir.sub %a, %b : i32
  return %r : i32
}

func.func @test_mul(%a: i32, %b: i32) -> i32 {
  // CHECK-LABEL: func @test_mul
  // CHECK-NEXT:    %[[R:.*]] = cir.mul %arg0, %arg1 : i32
  %r = cir.mul %a, %b : i32
  return %r : i32
}

func.func @test_divsi(%a: i32, %b: i32) -> i32 {
  // CHECK-LABEL: func @test_divsi
  // CHECK-NEXT:    %[[R:.*]] = cir.divsi %arg0, %arg1 : i32
  %r = cir.divsi %a, %b : i32
  return %r : i32
}

func.func @test_divui(%a: i32, %b: i32) -> i32 {
  // CHECK-LABEL: func @test_divui
  // CHECK-NEXT:    %[[R:.*]] = cir.divui %arg0, %arg1 : i32
  %r = cir.divui %a, %b : i32
  return %r : i32
}

func.func @test_remsi(%a: i32, %b: i32) -> i32 {
  // CHECK-LABEL: func @test_remsi
  // CHECK-NEXT:    %[[R:.*]] = cir.remsi %arg0, %arg1 : i32
  %r = cir.remsi %a, %b : i32
  return %r : i32
}

func.func @test_remui(%a: i32, %b: i32) -> i32 {
  // CHECK-LABEL: func @test_remui
  // CHECK-NEXT:    %[[R:.*]] = cir.remui %arg0, %arg1 : i32
  %r = cir.remui %a, %b : i32
  return %r : i32
}

func.func @test_neg(%a: i32) -> i32 {
  // CHECK-LABEL: func @test_neg
  // CHECK-NEXT:    %[[R:.*]] = cir.neg %arg0 : i32
  %r = cir.neg %a : i32
  return %r : i32
}

func.func @test_divf(%a: f32, %b: f32) -> f32 {
  // CHECK-LABEL: func @test_divf
  // CHECK-NEXT:    %[[R:.*]] = cir.divf %arg0, %arg1 : f32
  %r = cir.divf %a, %b : f32
  return %r : f32
}

func.func @test_remf(%a: f32, %b: f32) -> f32 {
  // CHECK-LABEL: func @test_remf
  // CHECK-NEXT:    %[[R:.*]] = cir.remf %arg0, %arg1 : f32
  %r = cir.remf %a, %b : f32
  return %r : f32
}

func.func @test_negf(%a: f32) -> f32 {
  // CHECK-LABEL: func @test_negf
  // CHECK-NEXT:    %[[R:.*]] = cir.negf %arg0 : f32
  %r = cir.negf %a : f32
  return %r : f32
}
