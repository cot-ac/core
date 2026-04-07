// RUN: %cir-opt --cir-to-llvm %s | %FileCheck %s

// CHECK-LABEL: llvm.func @test_struct_init
func.func @test_struct_init() -> !cir.struct<"Point", "x": i32, "y": i32> {
  %x = cir.constant 1 : i32
  %y = cir.constant 2 : i32
  // CHECK: %[[U:.*]] = llvm.mlir.undef : !llvm.struct<(i32, i32)>
  // CHECK: %[[S1:.*]] = llvm.insertvalue %{{.*}}, %[[U]][0]
  // CHECK: %[[S2:.*]] = llvm.insertvalue %{{.*}}, %[[S1]][1]
  %s = cir.struct_init(%x, %y) : !cir.struct<"Point", "x": i32, "y": i32>
  // CHECK: llvm.return %[[S2]]
  return %s : !cir.struct<"Point", "x": i32, "y": i32>
}

// CHECK-LABEL: llvm.func @test_field_val
func.func @test_field_val(%s: !cir.struct<"Point", "x": i32, "y": i32>) -> i32 {
  // CHECK: llvm.extractvalue %{{.*}}[0] : !llvm.struct<(i32, i32)>
  %v = cir.field_val %s, 0 : !cir.struct<"Point", "x": i32, "y": i32> to i32
  return %v : i32
}

// CHECK-LABEL: llvm.func @test_field_ptr
func.func @test_field_ptr(%base: !cir.ptr) -> !cir.ptr {
  // CHECK: llvm.getelementptr %{{.*}}[0, 1] : (!llvm.ptr) -> !llvm.ptr, !llvm.struct<(i32, i32)>
  %p = cir.field_ptr %base, 1, !cir.struct<"Point", "x": i32, "y": i32> : !cir.ptr to !cir.ptr
  return %p : !cir.ptr
}
