// RUN: %cir-opt %s | %cir-opt | %FileCheck %s

// CHECK-LABEL: func.func @test_string_constant
func.func @test_string_constant() -> !cir.slice<i8> {
  // CHECK: cir.string_constant "hello" : <i8>
  %s = cir.string_constant "hello" : !cir.slice<i8>
  return %s : !cir.slice<i8>
}

// CHECK-LABEL: func.func @test_slice_ptr
func.func @test_slice_ptr(%s: !cir.slice<i32>) -> !cir.ptr {
  // CHECK: cir.slice_ptr %{{.*}} : <i32> to !cir.ptr
  %p = cir.slice_ptr %s : !cir.slice<i32> to !cir.ptr
  return %p : !cir.ptr
}

// CHECK-LABEL: func.func @test_slice_len
func.func @test_slice_len(%s: !cir.slice<i32>) -> i64 {
  // CHECK: cir.slice_len %{{.*}} : <i32>
  %n = cir.slice_len %s : !cir.slice<i32>
  return %n : i64
}

// CHECK-LABEL: func.func @test_slice_elem
func.func @test_slice_elem(%s: !cir.slice<i32>, %idx: i64) -> i32 {
  // CHECK: cir.slice_elem %{{.*}}, %{{.*}} : <i32>, i64 to i32
  %v = cir.slice_elem %s, %idx : !cir.slice<i32>, i64 to i32
  return %v : i32
}

// CHECK-LABEL: func.func @test_array_to_slice
func.func @test_array_to_slice(%base: !cir.ptr, %start: i64, %end: i64) -> !cir.slice<i32> {
  // CHECK: cir.array_to_slice %{{.*}}, %{{.*}}, %{{.*}} : (!cir.ptr, i64, i64) to <i32>
  %s = cir.array_to_slice %base, %start, %end : (!cir.ptr, i64, i64) to !cir.slice<i32>
  return %s : !cir.slice<i32>
}
