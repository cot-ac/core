// RUN: %cir-opt %s | %FileCheck %s

%radius = cir.constant 3.14 : f64

// CHECK: cir.union_init "Circle"(%{{.*}}) : f64 to !cir.tagged_union<"Shape", "Circle": f64, "Rect": i64>
%0 = cir.union_init "Circle"(%radius) : f64 to !cir.tagged_union<"Shape", "Circle": f64, "Rect": i64>

// CHECK: cir.union_tag %{{.*}} : !cir.tagged_union<"Shape", "Circle": f64, "Rect": i64>
%1 = cir.union_tag %0 : !cir.tagged_union<"Shape", "Circle": f64, "Rect": i64>

// CHECK: cir.union_payload "Circle" %{{.*}} : !cir.tagged_union<"Shape", "Circle": f64, "Rect": i64> to f64
%2 = cir.union_payload "Circle" %0 : !cir.tagged_union<"Shape", "Circle": f64, "Rect": i64> to f64
