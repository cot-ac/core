// RUN: %cir-opt --cir-to-llvm %s | %FileCheck %s

// --- Value-based lowering (i32 payload -> struct<(i32, i1)>) ---

// CHECK-LABEL: llvm.func @test_none_value
func.func @test_none_value() -> !cir.optional<i32> {
  // CHECK: %[[U:.*]] = llvm.mlir.undef : !llvm.struct<(i32, i1)>
  // CHECK: %[[Z:.*]] = llvm.mlir.constant(false) : i1
  // CHECK: %[[R:.*]] = llvm.insertvalue %[[Z]], %[[U]][1]
  %n = cir.none : !cir.optional<i32>
  // CHECK: llvm.return %[[R]]
  return %n : !cir.optional<i32>
}

// CHECK-LABEL: llvm.func @test_wrap_value
func.func @test_wrap_value(%val: i32) -> !cir.optional<i32> {
  // CHECK: %[[U:.*]] = llvm.mlir.undef : !llvm.struct<(i32, i1)>
  // CHECK: %[[S1:.*]] = llvm.insertvalue %{{.*}}, %[[U]][0]
  // CHECK: %[[ONE:.*]] = llvm.mlir.constant(true) : i1
  // CHECK: %[[S2:.*]] = llvm.insertvalue %[[ONE]], %[[S1]][1]
  %w = cir.wrap_optional %val : i32 to !cir.optional<i32>
  // CHECK: llvm.return %[[S2]]
  return %w : !cir.optional<i32>
}

// CHECK-LABEL: llvm.func @test_is_non_null_value
func.func @test_is_non_null_value(%opt: !cir.optional<i32>) -> i1 {
  // CHECK: llvm.extractvalue %{{.*}}[1] : !llvm.struct<(i32, i1)>
  %b = cir.is_non_null %opt : !cir.optional<i32> to i1
  return %b : i1
}

// CHECK-LABEL: llvm.func @test_payload_value
func.func @test_payload_value(%opt: !cir.optional<i32>) -> i32 {
  // CHECK: llvm.extractvalue %{{.*}}[0] : !llvm.struct<(i32, i1)>
  %v = cir.optional_payload %opt : !cir.optional<i32> to i32
  return %v : i32
}

// --- Pointer-based lowering (!cir.ptr payload -> null-pointer opt) ---

// CHECK-LABEL: llvm.func @test_none_ptr
func.func @test_none_ptr() -> !cir.optional<!cir.ptr> {
  // CHECK: %[[N:.*]] = llvm.mlir.zero : !llvm.ptr
  %n = cir.none : !cir.optional<!cir.ptr>
  // CHECK: llvm.return %[[N]]
  return %n : !cir.optional<!cir.ptr>
}

// CHECK-LABEL: llvm.func @test_wrap_ptr
func.func @test_wrap_ptr(%p: !cir.ptr) -> !cir.optional<!cir.ptr> {
  // CHECK-NOT: llvm.insertvalue
  %w = cir.wrap_optional %p : !cir.ptr to !cir.optional<!cir.ptr>
  // CHECK: llvm.return %{{.*}} : !llvm.ptr
  return %w : !cir.optional<!cir.ptr>
}

// CHECK-LABEL: llvm.func @test_is_non_null_ptr
func.func @test_is_non_null_ptr(%opt: !cir.optional<!cir.ptr>) -> i1 {
  // CHECK: %[[NULL:.*]] = llvm.mlir.zero : !llvm.ptr
  // CHECK: llvm.icmp "ne" %{{.*}}, %[[NULL]]
  %b = cir.is_non_null %opt : !cir.optional<!cir.ptr> to i1
  return %b : i1
}

// CHECK-LABEL: llvm.func @test_payload_ptr
func.func @test_payload_ptr(%opt: !cir.optional<!cir.ptr>) -> !cir.ptr {
  // CHECK-NOT: llvm.extractvalue
  %v = cir.optional_payload %opt : !cir.optional<!cir.ptr> to !cir.ptr
  // CHECK: llvm.return %{{.*}} : !llvm.ptr
  return %v : !cir.ptr
}
