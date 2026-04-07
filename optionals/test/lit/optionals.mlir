// RUN: %cir-opt %s | %cir-opt | %FileCheck %s

// CHECK-LABEL: func.func @test_none
func.func @test_none() -> !cir.optional<i32> {
  // CHECK: cir.none : !cir.optional<i32>
  %n = cir.none : !cir.optional<i32>
  return %n : !cir.optional<i32>
}

// CHECK-LABEL: func.func @test_wrap_optional
func.func @test_wrap_optional(%val: i32) -> !cir.optional<i32> {
  // CHECK: cir.wrap_optional %{{.*}} : i32 to !cir.optional<i32>
  %w = cir.wrap_optional %val : i32 to !cir.optional<i32>
  return %w : !cir.optional<i32>
}

// CHECK-LABEL: func.func @test_is_non_null
func.func @test_is_non_null(%opt: !cir.optional<i32>) -> i1 {
  // CHECK: cir.is_non_null %{{.*}} : !cir.optional<i32> to i1
  %b = cir.is_non_null %opt : !cir.optional<i32> to i1
  return %b : i1
}

// CHECK-LABEL: func.func @test_optional_payload
func.func @test_optional_payload(%opt: !cir.optional<i32>) -> i32 {
  // CHECK: cir.optional_payload %{{.*}} : !cir.optional<i32> to i32
  %v = cir.optional_payload %opt : !cir.optional<i32> to i32
  return %v : i32
}

// CHECK-LABEL: func.func @test_optional_ptr_payload
func.func @test_optional_ptr_payload() -> !cir.optional<!cir.ptr> {
  // CHECK: cir.none : !cir.optional<!cir.ptr>
  %n = cir.none : !cir.optional<!cir.ptr>
  return %n : !cir.optional<!cir.ptr>
}
