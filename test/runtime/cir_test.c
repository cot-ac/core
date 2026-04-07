//===- cir_test.c - cot-test runtime functions ----------------*- C -*-===//
//
// Runtime support for cir.assert and TestRunnerGenerator.
// Linked into test binaries built with `cot test`.
//
//===----------------------------------------------------------------------===//
#include <stdio.h>
#include <stdint.h>

void cir_test_fail(const char *msg, int64_t len) {
  fprintf(stderr, "FAIL: %.*s\n", (int)len, msg);
}

void cir_test_pass(const char *msg, int64_t len) {
  fprintf(stderr, "PASS: %.*s\n", (int)len, msg);
}

void cir_test_summary(int32_t count) {
  fprintf(stderr, "\n%d tests passed\n", count);
}
