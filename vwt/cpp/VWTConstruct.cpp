//===- VWTConstruct.cpp - VWT construct registration -----------*- C++ -*-===//
#include "cot/Construct/Construct.h"
#include "cot/CIR/CIRDialect.h"
#include "vwt/Ops.h"
#include "vwt/Lowering.h"

#include "mlir/IR/DialectImplementation.h"

using namespace mlir;

namespace {

class VWTConstruct : public cot::Construct {
public:
  llvm::StringRef getName() const override { return "vwt"; }

  void registerOpsAndTypes(MLIRContext &ctx) override {
    auto *dialect = ctx.getOrLoadDialect<cir::CIRDialect>();
    dialect->registerConstructOps<
        cir::VWTSizeOp,
        cir::VWTStrideOp,
        cir::VWTAlignOp,
        cir::VWTCopyOp,
        cir::VWTDestroyOp,
        cir::VWTMoveOp,
        cir::VWTInitBufferOp
    >();
  }

  void populateLoweringPatterns(
      RewritePatternSet &patterns,
      TypeConverter &typeConverter) override {
    cot::populateVWTPatterns(patterns, typeConverter);
  }
};

} // namespace

COT_REGISTER_CONSTRUCT(VWTConstruct)
