// RUN: %cir-opt --cir-sema %s | %FileCheck %s

// Test: CIRSema with comptime step runs without crashing.
// The comptime step tracks constant values; the generics step resolves
// generic_apply; the types step inserts casts. All in one walk.

func.func @identity(%arg0: !cir.type_param<"T">) -> !cir.type_param<"T"> {
  return %arg0 : !cir.type_param<"T">
}

func.func @main() -> i32 {
  %0 = cir.constant 42 : i32
  // Generics step resolves this:
  // CHECK: call @identity__i32
  %1 = cir.generic_apply @identity(%0) subs ["T" = i32] : (i32) -> i32
  return %1 : i32
}
