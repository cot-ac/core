// RUN: %cir-opt --cir-sema %s | %FileCheck %s

// Test: GenericSpecializer step works through CIRSema single-walk pass.

func.func @identity(%arg0: !cir.type_param<"T">) -> !cir.type_param<"T"> {
  return %arg0 : !cir.type_param<"T">
}

func.func @test() -> i32 {
  %0 = cir.constant 42 : i32
  %1 = cir.generic_apply @identity(%0) subs ["T" = i32] : (i32) -> i32
  return %1 : i32
}

// CHECK: func.func @identity__i32(%arg0: i32) -> i32
// CHECK: func.func @test
// CHECK:   call @identity__i32(%{{.*}}) : (i32) -> i32
