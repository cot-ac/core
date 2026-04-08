// RUN: %cir-opt --cir-sema %s | %FileCheck %s

// Test: comptime branch elimination via ComptimeEvaluable on CondBrOp.
// When condition is comptime-known, condbr is replaced with br.

func.func @test_branch_elim() -> i32 {
  %true = cir.constant true
  // CHECK-NOT: cir.condbr
  // CHECK: cir.br ^bb1
  cir.condbr %true, ^bb1, ^bb2
^bb1:
  %result = cir.constant 42 : i32
  return %result : i32
^bb2:
  %zero = cir.constant 0 : i32
  return %zero : i32
}
