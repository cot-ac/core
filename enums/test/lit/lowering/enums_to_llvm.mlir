// RUN: %cir-opt --cir-to-llvm %s | %FileCheck %s

func.func @test_enum() -> i32 {
  // Red = 0, Green = 1, Blue = 2
  %red = cir.enum_constant "Red" : !cir.enum<"Color", i32, "Red", "Green", "Blue">
  %blue = cir.enum_constant "Blue" : !cir.enum<"Color", i32, "Red", "Green", "Blue">

  // Extract integer tag
  %tag = cir.enum_value %blue : !cir.enum<"Color", i32, "Red", "Green", "Blue"> to i32

  // CHECK: llvm.mlir.constant(2 : i32) : i32
  // CHECK: llvm.return
  return %tag : i32
}
