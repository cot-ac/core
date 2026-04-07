// RUN: %cir-opt --cir-to-llvm %s | %FileCheck %s

func.func @test_union() -> i32 {
  // Create a union with i32 payload
  %val = cir.constant 42 : i32
  %u = cir.union_init "A"(%val) : i32 to !cir.tagged_union<"U", "A": i32, "B": i64>

  // Extract tag
  // CHECK: llvm.extractvalue
  %tag = cir.union_tag %u : !cir.tagged_union<"U", "A": i32, "B": i64>

  // Extract payload
  // CHECK: llvm.extractvalue
  // CHECK: llvm.store
  // CHECK: llvm.load
  %payload = cir.union_payload "A" %u : !cir.tagged_union<"U", "A": i32, "B": i64> to i32

  return %payload : i32
}
