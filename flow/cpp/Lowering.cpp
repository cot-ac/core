//===- Lowering.cpp - cot-flow CIR → LLVM patterns -----------*- C++ -*-===//
//
// Four lowering patterns: br, condbr, switch, trap.
// Reference: MLIR ConversionPatterns, LLVM IR branch lowering.
//
//===----------------------------------------------------------------------===//
#include "flow/Lowering.h"
#include "flow/Ops.h"

#include "mlir/Conversion/LLVMCommon/Pattern.h"
#include "mlir/Dialect/LLVMIR/LLVMDialect.h"
#include "mlir/Transforms/DialectConversion.h"

using namespace mlir;

namespace {

//===----------------------------------------------------------------------===//
// BrOp → llvm.br
//===----------------------------------------------------------------------===//

struct BrOpLowering : public OpConversionPattern<cir::BrOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::BrOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    rewriter.replaceOpWithNewOp<LLVM::BrOp>(
        op, adaptor.getDestOperands(), op.getDest());
    return success();
  }
};

//===----------------------------------------------------------------------===//
// CondBrOp → llvm.cond_br
//===----------------------------------------------------------------------===//

struct CondBrOpLowering : public OpConversionPattern<cir::CondBrOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::CondBrOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    rewriter.replaceOpWithNewOp<LLVM::CondBrOp>(
        op, adaptor.getCondition(),
        op.getTrueDest(), adaptor.getTrueDestOperands(),
        op.getFalseDest(), adaptor.getFalseDestOperands());
    return success();
  }
};

//===----------------------------------------------------------------------===//
// SwitchOp → llvm.switch
//===----------------------------------------------------------------------===//

struct SwitchOpLowering : public OpConversionPattern<cir::SwitchOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::SwitchOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto caseVals = op.getCaseValues();
    auto caseDests = op.getCaseDests();

    // Build DenseIntElementsAttr from the i64 case values
    auto valueType = op.getValue().getType();
    SmallVector<APInt> caseAPInts;
    unsigned bitWidth = valueType.getIntOrFloatBitWidth();
    for (int64_t val : caseVals)
      caseAPInts.push_back(APInt(bitWidth, val, /*isSigned=*/true));

    DenseIntElementsAttr caseValuesAttr;
    if (!caseAPInts.empty()) {
      auto attrType =
          RankedTensorType::get({(int64_t)caseAPInts.size()}, valueType);
      caseValuesAttr = DenseIntElementsAttr::get(attrType, caseAPInts);
    }

    rewriter.replaceOpWithNewOp<LLVM::SwitchOp>(
        op, adaptor.getValue(), op.getDefaultDest(),
        ValueRange{}, caseValuesAttr,
        op.getCaseDests(), SmallVector<ValueRange>(caseDests.size()));
    return success();
  }
};

//===----------------------------------------------------------------------===//
// TrapOp → llvm.intr.trap + llvm.unreachable
//===----------------------------------------------------------------------===//

struct TrapOpLowering : public OpConversionPattern<cir::TrapOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::TrapOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto loc = op.getLoc();
    rewriter.create<LLVM::Trap>(loc);
    rewriter.replaceOpWithNewOp<LLVM::UnreachableOp>(op);
    return success();
  }
};

} // anonymous namespace

void cot::populateFlowPatterns(RewritePatternSet &patterns,
                               TypeConverter &typeConverter) {
  patterns.add<
      BrOpLowering,
      CondBrOpLowering,
      SwitchOpLowering,
      TrapOpLowering>(typeConverter, patterns.getContext());
}
