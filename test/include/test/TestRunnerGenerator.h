//===- TestRunnerGenerator.h - test runner pass declaration ----*- C++ -*-===//
#ifndef COT_TEST_TESTRUNNER_H
#define COT_TEST_TESTRUNNER_H

#include "mlir/Pass/Pass.h"
#include <memory>

namespace cot {

/// Create the TestRunnerGenerator pass.
/// Discovers cir.test_case ops, extracts their regions into standalone
/// test functions, generates @main that calls each test sequentially.
std::unique_ptr<mlir::Pass> createTestRunnerGeneratorPass();

} // namespace cot

#endif // COT_TEST_TESTRUNNER_H
