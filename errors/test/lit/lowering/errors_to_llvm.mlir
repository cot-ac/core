// RUN: %cir-opt --cir-to-llvm %s | %FileCheck %s

// CHECK-LABEL: llvm.func @test_wrap_result
func.func @test_wrap_result(%val: i32) -> !cir.error_union<i32> {
  // CHECK: %[[U:.*]] = llvm.mlir.undef : !llvm.struct<(i32, i16)>
  // CHECK: %[[S1:.*]] = llvm.insertvalue %{{.*}}, %[[U]][0]
  // CHECK: %[[Z:.*]] = llvm.mlir.constant(0 : i16) : i16
  // CHECK: %[[S2:.*]] = llvm.insertvalue %[[Z]], %[[S1]][1]
  %r = cir.wrap_result %val : i32 to !cir.error_union<i32>
  // CHECK: llvm.return %[[S2]]
  return %r : !cir.error_union<i32>
}

// CHECK-LABEL: llvm.func @test_wrap_error
func.func @test_wrap_error(%code: i16) -> !cir.error_union<i32> {
  // CHECK: %[[U:.*]] = llvm.mlir.undef : !llvm.struct<(i32, i16)>
  // CHECK: %[[S:.*]] = llvm.insertvalue %{{.*}}, %[[U]][1]
  %e = cir.wrap_error %code : !cir.error_union<i32>
  // CHECK: llvm.return %[[S]]
  return %e : !cir.error_union<i32>
}

// CHECK-LABEL: llvm.func @test_is_error
func.func @test_is_error(%eu: !cir.error_union<i32>) -> i1 {
  // CHECK: %[[CODE:.*]] = llvm.extractvalue %{{.*}}[1] : !llvm.struct<(i32, i16)>
  // CHECK: %[[ZERO:.*]] = llvm.mlir.constant(0 : i16) : i16
  // CHECK: llvm.icmp "ne" %[[CODE]], %[[ZERO]]
  %b = cir.is_error %eu : !cir.error_union<i32> to i1
  return %b : i1
}

// CHECK-LABEL: llvm.func @test_error_payload
func.func @test_error_payload(%eu: !cir.error_union<i32>) -> i32 {
  // CHECK: llvm.extractvalue %{{.*}}[0] : !llvm.struct<(i32, i16)>
  %v = cir.error_payload %eu : !cir.error_union<i32> to i32
  return %v : i32
}

// CHECK-LABEL: llvm.func @test_error_code
func.func @test_error_code(%eu: !cir.error_union<i32>) -> i16 {
  // CHECK: llvm.extractvalue %{{.*}}[1] : !llvm.struct<(i32, i16)>
  %c = cir.error_code %eu : !cir.error_union<i32> to i16
  return %c : i16
}
