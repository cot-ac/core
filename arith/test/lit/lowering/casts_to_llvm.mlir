// RUN: %cir-opt %s --cir-to-llvm | %FileCheck %s

func.func @test_extsi_lower(%a: i32) -> i64 {
  // CHECK-LABEL: llvm.func @test_extsi_lower
  // CHECK:         llvm.sext
  %r = cir.extsi %a : i32 to i64
  return %r : i64
}

func.func @test_extui_lower(%a: i32) -> i64 {
  // CHECK-LABEL: llvm.func @test_extui_lower
  // CHECK:         llvm.zext
  %r = cir.extui %a : i32 to i64
  return %r : i64
}

func.func @test_trunci_lower(%a: i64) -> i32 {
  // CHECK-LABEL: llvm.func @test_trunci_lower
  // CHECK:         llvm.trunc
  %r = cir.trunci %a : i64 to i32
  return %r : i32
}

func.func @test_sitofp_lower(%a: i32) -> f64 {
  // CHECK-LABEL: llvm.func @test_sitofp_lower
  // CHECK:         llvm.sitofp
  %r = cir.sitofp %a : i32 to f64
  return %r : f64
}

func.func @test_fptosi_lower(%a: f64) -> i32 {
  // CHECK-LABEL: llvm.func @test_fptosi_lower
  // CHECK:         llvm.fptosi
  %r = cir.fptosi %a : f64 to i32
  return %r : i32
}

func.func @test_extf_lower(%a: f32) -> f64 {
  // CHECK-LABEL: llvm.func @test_extf_lower
  // CHECK:         llvm.fpext
  %r = cir.extf %a : f32 to f64
  return %r : f64
}

func.func @test_truncf_lower(%a: f64) -> f32 {
  // CHECK-LABEL: llvm.func @test_truncf_lower
  // CHECK:         llvm.fptrunc
  %r = cir.truncf %a : f64 to f32
  return %r : f32
}
