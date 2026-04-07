// RUN: %cir-opt %s | %FileCheck %s

func.func @test_extsi(%a: i32) -> i64 {
  // CHECK-LABEL: func @test_extsi
  // CHECK-NEXT:    %[[R:.*]] = cir.extsi %arg0 : i32 to i64
  %r = cir.extsi %a : i32 to i64
  return %r : i64
}

func.func @test_extui(%a: i32) -> i64 {
  // CHECK-LABEL: func @test_extui
  // CHECK-NEXT:    %[[R:.*]] = cir.extui %arg0 : i32 to i64
  %r = cir.extui %a : i32 to i64
  return %r : i64
}

func.func @test_trunci(%a: i64) -> i32 {
  // CHECK-LABEL: func @test_trunci
  // CHECK-NEXT:    %[[R:.*]] = cir.trunci %arg0 : i64 to i32
  %r = cir.trunci %a : i64 to i32
  return %r : i32
}

func.func @test_sitofp(%a: i32) -> f64 {
  // CHECK-LABEL: func @test_sitofp
  // CHECK-NEXT:    %[[R:.*]] = cir.sitofp %arg0 : i32 to f64
  %r = cir.sitofp %a : i32 to f64
  return %r : f64
}

func.func @test_fptosi(%a: f64) -> i32 {
  // CHECK-LABEL: func @test_fptosi
  // CHECK-NEXT:    %[[R:.*]] = cir.fptosi %arg0 : f64 to i32
  %r = cir.fptosi %a : f64 to i32
  return %r : i32
}

func.func @test_extf(%a: f32) -> f64 {
  // CHECK-LABEL: func @test_extf
  // CHECK-NEXT:    %[[R:.*]] = cir.extf %arg0 : f32 to f64
  %r = cir.extf %a : f32 to f64
  return %r : f64
}

func.func @test_truncf(%a: f64) -> f32 {
  // CHECK-LABEL: func @test_truncf
  // CHECK-NEXT:    %[[R:.*]] = cir.truncf %arg0 : f64 to f32
  %r = cir.truncf %a : f64 to f32
  return %r : f32
}
