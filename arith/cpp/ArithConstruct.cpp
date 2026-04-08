//===- CoreConstruct.cpp - cot-core construct registration ----*- C++ -*-===//
//
// Registers cot-core's ops, transforms, and lowering patterns with the
// COT framework.
//
//===----------------------------------------------------------------------===//
#include "cot/Construct/Construct.h"
#include "cot/CIR/CIRDialect.h"
#include "arith/Ops.h"
#include "arith/Transform.h"
#include "arith/Lowering.h"

#include "mlir/Dialect/Func/IR/FuncOps.h"
#include "mlir/Pass/Pass.h"

using namespace mlir;

namespace {

class CoreConstruct : public cot::Construct {
public:
  llvm::StringRef getName() const override { return "arith"; }

  void registerOpsAndTypes(MLIRContext &ctx) override {
    // Get the CIR dialect and register cot-core's ops.
    auto *dialect = ctx.getOrLoadDialect<cir::CIRDialect>();
    dialect->registerConstructOps<
        cir::ConstantOp,
        cir::AddOp, cir::SubOp, cir::MulOp,
        cir::DivSIOp, cir::DivUIOp, cir::DivFOp,
        cir::RemSIOp, cir::RemUIOp, cir::RemFOp,
        cir::NegOp, cir::NegFOp,
        cir::BitAndOp, cir::BitOrOp, cir::BitXorOp, cir::BitNotOp,
        cir::ShlOp, cir::ShrOp, cir::ShrSOp,
        cir::CmpOp, cir::CmpFOp,
        cir::SelectOp,
        cir::ExtSIOp, cir::ExtUIOp, cir::TruncIOp,
        cir::SIToFPOp, cir::FPToSIOp,
        cir::ExtFOp, cir::TruncFOp
    >();
  }

  void addTransformers(PassManager &preSemaPM,
                       PassManager &postSemaPM) override {
    preSemaPM.addNestedPass<func::FuncOp>(
        cot::createSemanticAnalysisPass());
  }

  void populateLoweringPatterns(
      RewritePatternSet &patterns,
      TypeConverter &typeConverter) override {
    cot::populateArithmeticPatterns(patterns, typeConverter);
  }
};

} // namespace

COT_REGISTER_CONSTRUCT(CoreConstruct)
