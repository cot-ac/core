// RUN: %cir-opt --cir-to-llvm %s | %FileCheck %s

// CHECK-LABEL: llvm.func @test_br
func.func @test_br(%x: i32) -> i32 {
  // CHECK: llvm.br ^bb1(%{{.*}} : i32)
  cir.br ^bb1(%x : i32)
^bb1(%arg: i32):
  // CHECK: llvm.return %{{.*}} : i32
  return %arg : i32
}

// CHECK-LABEL: llvm.func @test_condbr
func.func @test_condbr(%cond: i1) -> i32 {
  // CHECK: llvm.cond_br %{{.*}}, ^bb1, ^bb2
  cir.condbr %cond, ^bb1, ^bb2
^bb1:
  %a = cir.constant 1 : i32
  return %a : i32
^bb2:
  %b = cir.constant 2 : i32
  return %b : i32
}

// CHECK-LABEL: llvm.func @test_switch
func.func @test_switch(%val: i32) -> i32 {
  // CHECK: llvm.switch %{{.*}} : i32, ^bb3 [
  // CHECK:   0: ^bb1
  // CHECK:   1: ^bb2
  // CHECK: ]
  cir.switch %val : i32, ^default [0 : ^case0, 1 : ^case1]
^case0:
  %a = cir.constant 10 : i32
  return %a : i32
^case1:
  %b = cir.constant 20 : i32
  return %b : i32
^default:
  %c = cir.constant 99 : i32
  return %c : i32
}

// CHECK-LABEL: llvm.func @test_trap
func.func @test_trap() {
  // CHECK: "llvm.intr.trap"() : () -> ()
  // CHECK: llvm.unreachable
  cir.trap
}
