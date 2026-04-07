// RUN: %cir-opt %s | %cir-opt | %FileCheck %s

// CHECK-LABEL: func.func @test_array_init
func.func @test_array_init() -> !cir.array<3 x i32> {
  %a = cir.constant 1 : i32
  %b = cir.constant 2 : i32
  %c = cir.constant 3 : i32
  // CHECK: cir.array_init(%{{.*}}, %{{.*}}, %{{.*}}) : !cir.array<3 x i32>
  %arr = cir.array_init(%a, %b, %c) : !cir.array<3 x i32>
  return %arr : !cir.array<3 x i32>
}

// CHECK-LABEL: func.func @test_elem_val
func.func @test_elem_val(%arr: !cir.array<3 x i32>) -> i32 {
  // CHECK: cir.elem_val %{{.*}}, 1 : <3 x i32> to i32
  %v = cir.elem_val %arr, 1 : !cir.array<3 x i32> to i32
  return %v : i32
}

// CHECK-LABEL: func.func @test_elem_ptr
func.func @test_elem_ptr(%base: !cir.ptr, %idx: i64) -> !cir.ptr {
  // CHECK: cir.elem_ptr %{{.*}}, %{{.*}}, !cir.array<3 x i32> : (!cir.ptr, i64) to !cir.ptr
  %p = cir.elem_ptr %base, %idx, !cir.array<3 x i32> : (!cir.ptr, i64) to !cir.ptr
  return %p : !cir.ptr
}

// CHECK-LABEL: func.func @test_array_float
func.func @test_array_float() -> !cir.array<2 x f64> {
  %a = cir.constant 1.0 : f64
  %b = cir.constant 2.0 : f64
  // CHECK: cir.array_init(%{{.*}}, %{{.*}}) : !cir.array<2 x f64>
  %arr = cir.array_init(%a, %b) : !cir.array<2 x f64>
  return %arr : !cir.array<2 x f64>
}
