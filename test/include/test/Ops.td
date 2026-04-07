//===- Ops.td - cot-test operation definitions ---------------*- tablegen -*-===//
//
// cot-test: assert, test_case.
// Reference: Zig test blocks, Rust #[test] attribute.
//
//===----------------------------------------------------------------------===//

#ifndef TEST_OPS
#define TEST_OPS

include "cot/CIR/CIROps.td"

//===----------------------------------------------------------------------===//
// Assert — conditional trap with diagnostic message
//===----------------------------------------------------------------------===//

def CIR_AssertOp : CIR_Op<"assert", []> {
  let summary = "assert condition is true, trap with message if false";
  let description = [{
    Runtime assertion. If the condition is false, prints the message to
    stderr via @cir_test_fail and traps. Carries source location for
    diagnostics. Unlike raw cir.trap, this provides meaningful output.
    Reference: Zig @panic, Rust assert!().
  }];
  let arguments = (ins I1:$condition, StrAttr:$message);
  let assemblyFormat = "$condition `,` $message attr-dict";
}

//===----------------------------------------------------------------------===//
// TestCase — region op wrapping a test body
//===----------------------------------------------------------------------===//

def CIR_TestCaseOp : CIR_Op<"test_case", [NoTerminator]> {
  let summary = "a named test case containing assertions";
  let description = [{
    Wraps a test body in a named region. The TestRunnerGenerator pass
    extracts each test_case's region into a standalone @__test_N function
    and generates a @main that calls them all sequentially.
    Reference: Zig test "name" { }, Rust #[test].
  }];
  let arguments = (ins StrAttr:$name);
  let regions = (region SizedRegion<1>:$body);
  let assemblyFormat = "$name $body attr-dict";
}

#endif // TEST_OPS
