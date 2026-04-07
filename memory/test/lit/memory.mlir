// RUN: %cir-opt %s | %cir-opt | %FileCheck %s

// CHECK-LABEL: func.func @test_alloca
func.func @test_alloca() {
  // CHECK: cir.alloca i32 : !cir.ptr
  %p = cir.alloca i32 : !cir.ptr
  return
}

// CHECK-LABEL: func.func @test_store
func.func @test_store(%val: i32, %ptr: !cir.ptr) {
  // CHECK: cir.store %{{.*}}, %{{.*}} : i32, !cir.ptr
  cir.store %val, %ptr : i32, !cir.ptr
  return
}

// CHECK-LABEL: func.func @test_load
func.func @test_load(%ptr: !cir.ptr) -> i32 {
  // CHECK: cir.load %{{.*}} : !cir.ptr to i32
  %v = cir.load %ptr : !cir.ptr to i32
  return %v : i32
}

// CHECK-LABEL: func.func @test_addr_of
func.func @test_addr_of(%ptr: !cir.ptr) -> !cir.ref<i32> {
  // CHECK: cir.addr_of %{{.*}} : !cir.ptr to <i32>
  %ref = cir.addr_of %ptr : !cir.ptr to !cir.ref<i32>
  return %ref : !cir.ref<i32>
}

// CHECK-LABEL: func.func @test_deref
func.func @test_deref(%ref: !cir.ref<i32>) -> i32 {
  // CHECK: cir.deref %{{.*}} : <i32> to i32
  %v = cir.deref %ref : !cir.ref<i32> to i32
  return %v : i32
}

// CHECK-LABEL: func.func @test_alloca_float
func.func @test_alloca_float() {
  // CHECK: cir.alloca f64 : !cir.ptr
  %p = cir.alloca f64 : !cir.ptr
  return
}

// CHECK-LABEL: func.func @test_memory_pipeline
func.func @test_memory_pipeline() -> i32 {
  // CHECK: %[[PTR:.*]] = cir.alloca i32 : !cir.ptr
  %p = cir.alloca i32 : !cir.ptr
  // CHECK: %[[C:.*]] = cir.constant 42 : i32
  %c = cir.constant 42 : i32
  // CHECK: cir.store %[[C]], %[[PTR]] : i32, !cir.ptr
  cir.store %c, %p : i32, !cir.ptr
  // CHECK: %[[V:.*]] = cir.load %[[PTR]] : !cir.ptr to i32
  %v = cir.load %p : !cir.ptr to i32
  return %v : i32
}
