// RUN: %cir-opt %s | %cir-opt | %FileCheck %s

// CHECK-LABEL: func.func @test_br
func.func @test_br() {
  // CHECK: cir.br ^bb1
  cir.br ^bb1
^bb1:
  return
}

// CHECK-LABEL: func.func @test_br_args
func.func @test_br_args(%x: i32) -> i32 {
  // CHECK: cir.br ^bb1(%{{.*}} : i32)
  cir.br ^bb1(%x : i32)
^bb1(%arg: i32):
  return %arg : i32
}

// CHECK-LABEL: func.func @test_condbr
func.func @test_condbr(%cond: i1) {
  // CHECK: cir.condbr %{{.*}}, ^bb1, ^bb2
  cir.condbr %cond, ^bb1, ^bb2
^bb1:
  return
^bb2:
  return
}

// CHECK-LABEL: func.func @test_condbr_args
func.func @test_condbr_args(%cond: i1, %x: i32) -> i32 {
  // CHECK: cir.condbr %{{.*}}, ^bb1(%{{.*}} : i32), ^bb2(%{{.*}} : i32)
  cir.condbr %cond, ^bb1(%x : i32), ^bb2(%x : i32)
^bb1(%a: i32):
  return %a : i32
^bb2(%b: i32):
  return %b : i32
}

// CHECK-LABEL: func.func @test_switch
func.func @test_switch(%val: i32) {
  // CHECK: cir.switch %{{.*}} : i32, ^bb3 [0 : ^bb1, 1 : ^bb2]
  cir.switch %val : i32, ^default [0 : ^case0, 1 : ^case1]
^case0:
  return
^case1:
  return
^default:
  return
}

// CHECK-LABEL: func.func @test_trap
func.func @test_trap() {
  // CHECK: cir.trap
  cir.trap
}
