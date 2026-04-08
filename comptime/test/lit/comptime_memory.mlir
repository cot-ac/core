// RUN: %cir-opt --cir-sema %s | %FileCheck %s

// Test: comptime memory simulation (alloca → store → load → propagate).
// Values flow through simulated memory and continue folding downstream.

func.func @test_through_memory() -> i32 {
  // Comptime arithmetic: 20 + 22 = 42
  %a = cir.constant 20 : i32
  %b = cir.constant 22 : i32
  // CHECK: cir.constant 42 : i32
  %c = cir.add %a, %b : i32

  // Store to alloca, load back — value propagates through memory
  %ptr = cir.alloca i32 : !cir.ptr
  cir.store %c, %ptr : i32, !cir.ptr
  %loaded = cir.load %ptr : !cir.ptr to i32

  // Downstream op using loaded value — should also fold
  %one = cir.constant 0 : i32
  // 42 + 0 = 42 (identity fold via AddOp::fold)
  // CHECK: return
  %result = cir.add %loaded, %one : i32
  return %result : i32
}
