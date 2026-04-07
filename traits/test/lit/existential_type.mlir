// RUN: %cir-opt %s | %FileCheck %s

// Test: existential type roundtrip.

// CHECK: func.func @take_summable(%arg0: !cir.existential<"Summable">)
func.func @take_summable(%arg0: !cir.existential<"Summable">) {
  return
}
