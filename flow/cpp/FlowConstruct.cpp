//===- FlowConstruct.cpp - cot-flow construct registration ----*- C++ -*-===//
//
// Registers cot-flow's ops and lowering patterns with the COT framework.
//
//===----------------------------------------------------------------------===//
#include "cot/Construct/Construct.h"
#include "cot/CIR/CIRDialect.h"
#include "flow/Ops.h"
#include "flow/Lowering.h"

using namespace mlir;

namespace {

class FlowConstruct : public cot::Construct {
public:
  llvm::StringRef getName() const override { return "flow"; }

  void registerOpsAndTypes(MLIRContext &ctx) override {
    auto *dialect = ctx.getOrLoadDialect<cir::CIRDialect>();
    dialect->registerConstructOps<
        cir::BrOp,
        cir::CondBrOp,
        cir::SwitchOp,
        cir::TrapOp
    >();
  }

  void populateLoweringPatterns(
      RewritePatternSet &patterns,
      TypeConverter &typeConverter) override {
    cot::populateFlowPatterns(patterns, typeConverter);
  }
};

} // namespace

COT_REGISTER_CONSTRUCT(FlowConstruct)
