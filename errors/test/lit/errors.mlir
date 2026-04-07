// RUN: %cir-opt %s | %cir-opt | %FileCheck %s

// CHECK-LABEL: func.func @test_wrap_result
func.func @test_wrap_result(%val: i32) -> !cir.error_union<i32> {
  // CHECK: cir.wrap_result %{{.*}} : i32 to !cir.error_union<i32>
  %r = cir.wrap_result %val : i32 to !cir.error_union<i32>
  return %r : !cir.error_union<i32>
}

// CHECK-LABEL: func.func @test_wrap_error
func.func @test_wrap_error(%code: i16) -> !cir.error_union<i32> {
  // CHECK: cir.wrap_error %{{.*}} : !cir.error_union<i32>
  %e = cir.wrap_error %code : !cir.error_union<i32>
  return %e : !cir.error_union<i32>
}

// CHECK-LABEL: func.func @test_is_error
func.func @test_is_error(%eu: !cir.error_union<i32>) -> i1 {
  // CHECK: cir.is_error %{{.*}} : !cir.error_union<i32> to i1
  %b = cir.is_error %eu : !cir.error_union<i32> to i1
  return %b : i1
}

// CHECK-LABEL: func.func @test_error_payload
func.func @test_error_payload(%eu: !cir.error_union<i32>) -> i32 {
  // CHECK: cir.error_payload %{{.*}} : !cir.error_union<i32> to i32
  %v = cir.error_payload %eu : !cir.error_union<i32> to i32
  return %v : i32
}

// CHECK-LABEL: func.func @test_error_code
func.func @test_error_code(%eu: !cir.error_union<i32>) -> i16 {
  // CHECK: cir.error_code %{{.*}} : !cir.error_union<i32> to i16
  %c = cir.error_code %eu : !cir.error_union<i32> to i16
  return %c : i16
}

// CHECK-LABEL: func.func @test_error_union_f64
func.func @test_error_union_f64(%val: f64) -> !cir.error_union<f64> {
  // CHECK: cir.wrap_result %{{.*}} : f64 to !cir.error_union<f64>
  %r = cir.wrap_result %val : f64 to !cir.error_union<f64>
  return %r : !cir.error_union<f64>
}
