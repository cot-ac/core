//===- TestRunnerGenerator.cpp - test runner pass --------------*- C++ -*-===//
//
// Discovers cir.test_case ops, extracts their regions into standalone
// test functions, generates @main that calls each sequentially.
// Reference: Zig test runner.
//
//===----------------------------------------------------------------------===//
#include "test/TestRunnerGenerator.h"
#include "test/Ops.h"

#include "mlir/Dialect/Func/IR/FuncOps.h"
#include "mlir/Dialect/LLVMIR/LLVMDialect.h"
#include "mlir/IR/Builders.h"
#include "mlir/IR/BuiltinOps.h"
#include "mlir/Pass/Pass.h"

using namespace mlir;

namespace {

struct TestRunnerGeneratorPass
    : public PassWrapper<TestRunnerGeneratorPass, OperationPass<ModuleOp>> {
  MLIR_DEFINE_EXPLICIT_INTERNAL_INLINE_TYPE_ID(TestRunnerGeneratorPass)

  StringRef getArgument() const override { return "cir-test-runner"; }
  StringRef getDescription() const override {
    return "Generate test runner main from cir.test_case ops";
  }

  void runOnOperation() override {
    auto module = getOperation();
    auto ctx = module.getContext();
    OpBuilder builder(ctx);

    // Collect all test_case ops
    SmallVector<cir::TestCaseOp> testCases;
    module.walk([&](cir::TestCaseOp op) { testCases.push_back(op); });

    // No tests → no-op (regular programs unaffected)
    if (testCases.empty())
      return;

    // Save test names before erasing ops
    SmallVector<std::string> testNames;
    for (auto &tc : testCases)
      testNames.push_back(tc.getName().str());

    // For each test_case, extract the body into a standalone function
    SmallVector<func::FuncOp> testFns;
    for (unsigned i = 0; i < testCases.size(); i++) {
      auto testCase = testCases[i];
      auto loc = testCase.getLoc();
      auto name = std::string("__test_") + std::to_string(i);

      // Create void() function
      builder.setInsertionPoint(testCase);
      auto fnType = builder.getFunctionType({}, {});
      auto fn = builder.create<func::FuncOp>(loc, name, fnType);

      // Move the test_case's region body into the function
      fn.getBody().takeBody(testCase.getBody());

      // Ensure the function body has a terminator
      auto &block = fn.getBody().back();
      if (block.empty() || !block.back().hasTrait<OpTrait::IsTerminator>()) {
        builder.setInsertionPointToEnd(&block);
        builder.create<func::ReturnOp>(loc, ValueRange{});
      }

      testFns.push_back(fn);

      // Erase the test_case op (region already moved)
      testCase.erase();
    }

    // Generate @main that calls each test function
    auto loc = builder.getUnknownLoc();
    builder.setInsertionPointToEnd(module.getBody());

    auto mainType = builder.getFunctionType({}, {builder.getI32Type()});
    auto mainFn = builder.create<func::FuncOp>(loc, "main", mainType);
    auto *entry = mainFn.addEntryBlock();
    builder.setInsertionPointToStart(entry);

    // Declare @cir_test_pass and @cir_test_summary
    auto ptrType = LLVM::LLVMPointerType::get(ctx);
    auto i64Type = builder.getI64Type();
    auto i32Type = builder.getI32Type();
    auto voidType = LLVM::LLVMVoidType::get(ctx);

    auto passFnType = LLVM::LLVMFunctionType::get(voidType, {ptrType, i64Type});
    auto passFn = module.lookupSymbol<LLVM::LLVMFuncOp>("cir_test_pass");
    if (!passFn) {
      OpBuilder::InsertionGuard guard(builder);
      builder.setInsertionPointToStart(module.getBody());
      passFn = builder.create<LLVM::LLVMFuncOp>(loc, "cir_test_pass", passFnType);
    }

    auto summaryFnType = LLVM::LLVMFunctionType::get(voidType, {i32Type});
    auto summaryFn = module.lookupSymbol<LLVM::LLVMFuncOp>("cir_test_summary");
    if (!summaryFn) {
      OpBuilder::InsertionGuard guard(builder);
      builder.setInsertionPointToStart(module.getBody());
      summaryFn = builder.create<LLVM::LLVMFuncOp>(loc, "cir_test_summary", summaryFnType);
    }

    // Call each test function, emit pass message after each
    for (unsigned i = 0; i < testFns.size(); i++) {
      builder.create<func::CallOp>(loc, testFns[i], ValueRange{});

      // Create global for pass message
      auto testName = testNames[i];
      auto i8Type = builder.getI8Type();
      auto arrayType = LLVM::LLVMArrayType::get(i8Type, testName.size());
      std::string globalName = "__test_name_" + std::to_string(i);

      {
        OpBuilder::InsertionGuard guard(builder);
        builder.setInsertionPointToStart(module.getBody());
        builder.create<LLVM::GlobalOp>(
            loc, arrayType, /*isConstant=*/true, LLVM::Linkage::Internal,
            globalName, builder.getStringAttr(testName));
      }

      auto namePtr = builder.create<LLVM::AddressOfOp>(loc, ptrType, globalName);
      auto nameLen = builder.create<LLVM::ConstantOp>(
          loc, i64Type, builder.getI64IntegerAttr(testName.size()));
      builder.create<LLVM::CallOp>(loc, passFn, ValueRange{namePtr, nameLen});
    }

    // Summary and return 0
    auto count = builder.create<LLVM::ConstantOp>(
        loc, i32Type, builder.getI32IntegerAttr(testFns.size()));
    builder.create<LLVM::CallOp>(loc, summaryFn, ValueRange{count});
    auto zero = builder.create<LLVM::ConstantOp>(
        loc, i32Type, builder.getI32IntegerAttr(0));
    builder.create<func::ReturnOp>(loc, ValueRange{zero});
  }
};

} // anonymous namespace

std::unique_ptr<Pass> cot::createTestRunnerGeneratorPass() {
  return std::make_unique<TestRunnerGeneratorPass>();
}
