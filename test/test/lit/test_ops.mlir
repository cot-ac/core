// RUN: %cir-opt %s | %cir-opt | %FileCheck %s

// CHECK-LABEL: func.func @test_assert
func.func @test_assert(%cond: i1) {
  // CHECK: cir.assert %{{.*}}, "condition must be true"
  cir.assert %cond, "condition must be true"
  return
}

// CHECK-LABEL: cir.test_case "addition works"
cir.test_case "addition works" {
  %a = cir.constant 2 : i32
  %b = cir.constant 3 : i32
  %c = cir.add %a, %b : i32
  %expected = cir.constant 5 : i32
  %eq = cir.cmp eq, %c, %expected : i32
  // CHECK: cir.assert
  cir.assert %eq, "2 + 3 should equal 5"
}
