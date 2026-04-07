// RUN: %cir-opt %s | %FileCheck %s

// A concrete function that implements a protocol method.
func.func @Point_sum(%arg0: i32, %arg1: i32) -> i32 {
  %0 = cir.add %arg0, %arg1 : i32
  return %0 : i32
}

// CHECK: cir.witness_table @Summable_Point ["sum" = @Point_sum]
cir.witness_table @Summable_Point ["sum" = @Point_sum] {protocol = "Summable", conforming_type = "Point"}
