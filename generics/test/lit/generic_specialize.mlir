// RUN: %cir-opt --cir-specialize %s | %FileCheck %s

// A generic identity function.
func.func @identity(%arg0: !cir.type_param<"T">) -> !cir.type_param<"T"> {
  return %arg0 : !cir.type_param<"T">
}

// Call with i32 substitution.
func.func @test_i32() -> i32 {
  %0 = cir.constant 42 : i32
  %1 = cir.generic_apply @identity(%0) subs ["T" = i32] : (i32) -> i32
  return %1 : i32
}

// Call with i64 substitution — should produce a different specialization.
func.func @test_i64() -> i64 {
  %0 = cir.constant 100 : i64
  %1 = cir.generic_apply @identity(%0) subs ["T" = i64] : (i64) -> i64
  return %1 : i64
}

// Verify: specialized functions created with concrete types.
// CHECK: func.func @identity__i32(%arg0: i32) -> i32
// CHECK:   return %arg0 : i32
// CHECK: func.func @identity__i64(%arg0: i64) -> i64
// CHECK:   return %arg0 : i64

// Verify: original generic template preserved.
// CHECK: func.func @identity(%arg0: !cir.type_param<"T">) -> !cir.type_param<"T">

// Verify: call sites rewritten to specialized functions.
// CHECK: func.func @test_i32
// CHECK:   call @identity__i32(%{{.*}}) : (i32) -> i32
// CHECK: func.func @test_i64
// CHECK:   call @identity__i64(%{{.*}}) : (i64) -> i64
