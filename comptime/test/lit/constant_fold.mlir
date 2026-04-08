// RUN: %cir-opt --cir-sema %s | %FileCheck %s

// Test: comptime step folds constant arithmetic via MLIR fold().
// Chained folding works because the single walk propagates comptime values.

func.func @test_fold() -> i32 {
  %a = cir.constant 10 : i32
  %b = cir.constant 32 : i32
  // 10 + 32 → 42 (folded via AddOp::fold)
  // CHECK: cir.constant 42 : i32
  %c = cir.add %a, %b : i32
  return %c : i32
}

func.func @test_chain() -> i32 {
  %a = cir.constant 5 : i32
  %b = cir.constant 10 : i32
  %c = cir.add %a, %b : i32
  %d = cir.constant 27 : i32
  // 5 + 10 = 15, then 15 + 27 = 42 (chained propagation)
  // CHECK: cir.constant 42 : i32
  // CHECK: return
  %e = cir.add %c, %d : i32
  return %e : i32
}
