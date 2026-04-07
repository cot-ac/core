// RUN: %cir-opt %s | %FileCheck %s

// Test: type_param type is parsed and printed correctly.

// A generic identity function with a type parameter.
// CHECK: func.func @identity(%arg0: !cir.type_param<"T">) -> !cir.type_param<"T">
func.func @identity(%arg0: !cir.type_param<"T">) -> !cir.type_param<"T"> {
  return %arg0 : !cir.type_param<"T">
}

// Multiple type parameters in one function.
// CHECK: func.func @pair(%arg0: !cir.type_param<"A">, %arg1: !cir.type_param<"B">) -> !cir.type_param<"A">
func.func @pair(%arg0: !cir.type_param<"A">, %arg1: !cir.type_param<"B">) -> !cir.type_param<"A"> {
  return %arg0 : !cir.type_param<"A">
}
