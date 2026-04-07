// RUN: %cir-opt --cir-to-llvm %s | %FileCheck %s

// CHECK: llvm.mlir.global internal constant @".str.{{.*}}"("hello\00")

// CHECK-LABEL: llvm.func @test_string_constant
func.func @test_string_constant() -> !cir.slice<i8> {
  // CHECK: llvm.mlir.addressof @".str.{{.*}}" : !llvm.ptr
  // CHECK: llvm.mlir.undef : !llvm.struct<(ptr, i64)>
  // CHECK: llvm.insertvalue
  // CHECK: llvm.insertvalue
  %s = cir.string_constant "hello" : !cir.slice<i8>
  return %s : !cir.slice<i8>
}

// CHECK-LABEL: llvm.func @test_slice_ptr
func.func @test_slice_ptr(%s: !cir.slice<i32>) -> !cir.ptr {
  // CHECK: llvm.extractvalue %{{.*}}[0] : !llvm.struct<(ptr, i64)>
  %p = cir.slice_ptr %s : !cir.slice<i32> to !cir.ptr
  return %p : !cir.ptr
}

// CHECK-LABEL: llvm.func @test_slice_len
func.func @test_slice_len(%s: !cir.slice<i32>) -> i64 {
  // CHECK: llvm.extractvalue %{{.*}}[1] : !llvm.struct<(ptr, i64)>
  %n = cir.slice_len %s : !cir.slice<i32>
  return %n : i64
}
