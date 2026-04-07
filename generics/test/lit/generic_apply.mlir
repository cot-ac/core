// RUN: %cir-opt %s | %FileCheck %s

// Test: generic_apply op is parsed and printed correctly.

// A generic identity function.
func.func @identity(%arg0: !cir.type_param<"T">) -> !cir.type_param<"T"> {
  return %arg0 : !cir.type_param<"T">
}

// Call the generic function with a concrete type substitution.
// CHECK: func.func @main
func.func @main() -> i32 {
  %0 = cir.constant 42 : i32
  // CHECK: cir.generic_apply @identity(%{{.*}}) subs ["T" = i32] : (i32) -> i32
  %1 = cir.generic_apply @identity(%0) subs ["T" = i32] : (i32) -> i32
  return %1 : i32
}
