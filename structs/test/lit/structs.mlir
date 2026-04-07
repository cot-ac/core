// RUN: %cir-opt %s | %cir-opt | %FileCheck %s

// CHECK-LABEL: func.func @test_struct_init
func.func @test_struct_init() -> !cir.struct<"Point", "x": i32, "y": i32> {
  // CHECK: cir.constant 1 : i32
  %x = cir.constant 1 : i32
  // CHECK: cir.constant 2 : i32
  %y = cir.constant 2 : i32
  // CHECK: cir.struct_init(%{{.*}}, %{{.*}}) : !cir.struct<"Point", "x": i32, "y": i32>
  %s = cir.struct_init(%x, %y) : !cir.struct<"Point", "x": i32, "y": i32>
  return %s : !cir.struct<"Point", "x": i32, "y": i32>
}

// CHECK-LABEL: func.func @test_field_val
func.func @test_field_val(%s: !cir.struct<"Point", "x": i32, "y": i32>) -> i32 {
  // CHECK: cir.field_val %{{.*}}, 0 : <"Point", "x": i32, "y": i32> to i32
  %v = cir.field_val %s, 0 : !cir.struct<"Point", "x": i32, "y": i32> to i32
  return %v : i32
}

// CHECK-LABEL: func.func @test_field_ptr
func.func @test_field_ptr(%base: !cir.ptr) -> !cir.ptr {
  // CHECK: cir.field_ptr %{{.*}}, 1, !cir.struct<"Point", "x": i32, "y": i32> : !cir.ptr to !cir.ptr
  %p = cir.field_ptr %base, 1, !cir.struct<"Point", "x": i32, "y": i32> : !cir.ptr to !cir.ptr
  return %p : !cir.ptr
}

// CHECK-LABEL: func.func @test_struct_float_fields
func.func @test_struct_float_fields() -> !cir.struct<"Vec2", "x": f64, "y": f64> {
  %x = cir.constant 1.0 : f64
  %y = cir.constant 2.0 : f64
  // CHECK: cir.struct_init(%{{.*}}, %{{.*}}) : !cir.struct<"Vec2", "x": f64, "y": f64>
  %s = cir.struct_init(%x, %y) : !cir.struct<"Vec2", "x": f64, "y": f64>
  return %s : !cir.struct<"Vec2", "x": f64, "y": f64>
}
