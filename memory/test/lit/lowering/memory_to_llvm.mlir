// RUN: %cir-opt --cir-to-llvm %s | %FileCheck %s

// CHECK-LABEL: llvm.func @test_alloca_store_load
func.func @test_alloca_store_load() -> i32 {
  // CHECK: %[[ONE:.*]] = llvm.mlir.constant(1 : i64) : i64
  // CHECK: %[[PTR:.*]] = llvm.alloca %[[ONE]] x i32 : (i64) -> !llvm.ptr
  %p = cir.alloca i32 : !cir.ptr
  // CHECK: %[[C:.*]] = llvm.mlir.constant(42 : i32) : i32
  %c = cir.constant 42 : i32
  // CHECK: llvm.store %[[C]], %[[PTR]] : i32, !llvm.ptr
  cir.store %c, %p : i32, !cir.ptr
  // CHECK: %[[V:.*]] = llvm.load %[[PTR]] : !llvm.ptr -> i32
  %v = cir.load %p : !cir.ptr to i32
  // CHECK: llvm.return %[[V]] : i32
  return %v : i32
}

// CHECK-LABEL: llvm.func @test_addr_of_deref
func.func @test_addr_of_deref() -> i32 {
  // CHECK: %[[ONE:.*]] = llvm.mlir.constant(1 : i64) : i64
  // CHECK: %[[PTR:.*]] = llvm.alloca %[[ONE]] x i32 : (i64) -> !llvm.ptr
  %p = cir.alloca i32 : !cir.ptr
  // CHECK: %[[C:.*]] = llvm.mlir.constant(42 : i32) : i32
  %c = cir.constant 42 : i32
  // CHECK: llvm.store %[[C]], %[[PTR]] : i32, !llvm.ptr
  cir.store %c, %p : i32, !cir.ptr
  // addr_of is identity — both !cir.ptr and !cir.ref<T> lower to !llvm.ptr
  %ref = cir.addr_of %p : !cir.ptr to !cir.ref<i32>
  // deref lowers to llvm.load
  // CHECK: %[[D:.*]] = llvm.load %[[PTR]] : !llvm.ptr -> i32
  %d = cir.deref %ref : !cir.ref<i32> to i32
  // CHECK: llvm.return %[[D]] : i32
  return %d : i32
}

// CHECK-LABEL: llvm.func @test_alloca_f64
func.func @test_alloca_f64() -> f64 {
  // CHECK: llvm.alloca %{{.*}} x f64 : (i64) -> !llvm.ptr
  %p = cir.alloca f64 : !cir.ptr
  %c = cir.constant 3.14 : f64
  cir.store %c, %p : f64, !cir.ptr
  // CHECK: llvm.load %{{.*}} : !llvm.ptr -> f64
  %v = cir.load %p : !cir.ptr to f64
  return %v : f64
}
