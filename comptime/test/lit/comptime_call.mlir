// RUN: %cir-opt --cir-sema %s | %FileCheck %s

// Test: comptime function call evaluation with memoization.
// When all args are comptime-known, the callee body is evaluated inline.
// Reference: Zig zirCall comptime path.

func.func @add(%a: i32, %b: i32) -> i32 {
  %r = cir.add %a, %b : i32
  return %r : i32
}

func.func @main() -> i32 {
  %x = cir.constant 20 : i32
  %y = cir.constant 22 : i32
  // Comptime call: add(20, 22) evaluated inline → 42
  // CHECK: cir.constant 42 : i32
  // CHECK-NOT: func.call @add
  %result = func.call @add(%x, %y) : (i32, i32) -> i32
  return %result : i32
}
