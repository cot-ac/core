// RUN: %cir-opt %s | %FileCheck %s

func.func @test_cmp_eq(%a: i32, %b: i32) -> i1 {
  // CHECK-LABEL: func @test_cmp_eq
  // CHECK-NEXT:    %[[R:.*]] = cir.cmp eq, %arg0, %arg1 : i32
  %r = cir.cmp eq, %a, %b : i32
  return %r : i1
}

func.func @test_cmp_slt(%a: i32, %b: i32) -> i1 {
  // CHECK-LABEL: func @test_cmp_slt
  // CHECK-NEXT:    %[[R:.*]] = cir.cmp slt, %arg0, %arg1 : i32
  %r = cir.cmp slt, %a, %b : i32
  return %r : i1
}

func.func @test_cmpf_oeq(%a: f32, %b: f32) -> i1 {
  // CHECK-LABEL: func @test_cmpf_oeq
  // CHECK-NEXT:    %[[R:.*]] = cir.cmpf oeq, %arg0, %arg1 : f32
  %r = cir.cmpf oeq, %a, %b : f32
  return %r : i1
}

func.func @test_select(%c: i1, %a: i32, %b: i32) -> i32 {
  // CHECK-LABEL: func @test_select
  // CHECK-NEXT:    %[[R:.*]] = cir.select %arg0, %arg1, %arg2 : i32
  %r = cir.select %c, %a, %b : i32
  return %r : i32
}
