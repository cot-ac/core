// RUN: %cir-opt --cir-to-llvm %s | %FileCheck %s

// CHECK-LABEL: llvm.func @test_array_init
func.func @test_array_init() -> !cir.array<3 x i32> {
  %a = cir.constant 1 : i32
  %b = cir.constant 2 : i32
  %c = cir.constant 3 : i32
  // CHECK: %[[U:.*]] = llvm.mlir.undef : !llvm.array<3 x i32>
  // CHECK: %[[A1:.*]] = llvm.insertvalue %{{.*}}, %[[U]][0]
  // CHECK: %[[A2:.*]] = llvm.insertvalue %{{.*}}, %[[A1]][1]
  // CHECK: %[[A3:.*]] = llvm.insertvalue %{{.*}}, %[[A2]][2]
  %arr = cir.array_init(%a, %b, %c) : !cir.array<3 x i32>
  // CHECK: llvm.return %[[A3]]
  return %arr : !cir.array<3 x i32>
}

// CHECK-LABEL: llvm.func @test_elem_val
func.func @test_elem_val(%arr: !cir.array<3 x i32>) -> i32 {
  // CHECK: llvm.extractvalue %{{.*}}[1] : !llvm.array<3 x i32>
  %v = cir.elem_val %arr, 1 : !cir.array<3 x i32> to i32
  return %v : i32
}

// CHECK-LABEL: llvm.func @test_elem_ptr
func.func @test_elem_ptr(%base: !cir.ptr, %idx: i64) -> !cir.ptr {
  // CHECK: llvm.getelementptr %{{.*}}[0, %{{.*}}] : (!llvm.ptr, i64) -> !llvm.ptr, !llvm.array<3 x i32>
  %p = cir.elem_ptr %base, %idx, !cir.array<3 x i32> : (!cir.ptr, i64) to !cir.ptr
  return %p : !cir.ptr
}
