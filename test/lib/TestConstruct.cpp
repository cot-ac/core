//===- TestConstruct.cpp - cot-test construct registration ----*- C++ -*-===//
//
// Registers cot-test's ops and lowering patterns with the COT framework.
// Also registers the TestRunnerGenerator pass.
//
//===----------------------------------------------------------------------===//
#include "cot/Construct/Construct.h"
#include "cot/CIR/CIRDialect.h"
#include "test/Ops.h"
#include "test/Lowering.h"
#include "test/TestRunnerGenerator.h"

#include "mlir/Dialect/LLVMIR/LLVMDialect.h"
#include "mlir/Pass/PassRegistry.h"

using namespace mlir;

namespace {

class TestConstruct : public cot::Construct {
public:
  llvm::StringRef getName() const override { return "test"; }

  llvm::SmallVector<llvm::StringRef> getRequiredConstructs() const override {
    return {"flow"};
  }

  void registerOpsAndTypes(MLIRContext &ctx) override {
    auto *dialect = ctx.getOrLoadDialect<cir::CIRDialect>();
    dialect->registerConstructOps<
        cir::AssertOp,
        cir::TestCaseOp
    >();
  }

  void populateLoweringPatterns(
      RewritePatternSet &patterns,
      TypeConverter &typeConverter) override {
    cot::populateTestPatterns(patterns, typeConverter);
  }

  void addTransformers(PassManager &preSemaPM,
                       PassManager &postSemaPM) override {
    postSemaPM.addPass(cot::createTestRunnerGeneratorPass());
  }
};

} // namespace

COT_REGISTER_CONSTRUCT(TestConstruct)
