// RUN: %cir-opt %s | %FileCheck %s

// CHECK: cir.enum_constant "Red" : !cir.enum<"Color", i32, "Red", "Green", "Blue">
%0 = cir.enum_constant "Red" : !cir.enum<"Color", i32, "Red", "Green", "Blue">

// CHECK: cir.enum_constant "Blue" : !cir.enum<"Color", i32, "Red", "Green", "Blue">
%1 = cir.enum_constant "Blue" : !cir.enum<"Color", i32, "Red", "Green", "Blue">

// CHECK: cir.enum_value %0 : !cir.enum<"Color", i32, "Red", "Green", "Blue"> to i32
%2 = cir.enum_value %0 : !cir.enum<"Color", i32, "Red", "Green", "Blue"> to i32
