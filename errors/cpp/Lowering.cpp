//===- Lowering.cpp - cot-errors CIR -> LLVM patterns ---------*- C++ -*-===//
//
// Five lowering patterns: wrap_result, wrap_error, is_error, error_payload,
// error_code. All use struct<(T, i16)> representation.
// Reference: Zig error union lowering, MLIR ConversionPatterns.
//
//===----------------------------------------------------------------------===//
#include "errors/Lowering.h"
#include "errors/Ops.h"
#include "errors/Types.h"

#include "mlir/Conversion/LLVMCommon/Pattern.h"
#include "mlir/Dialect/LLVMIR/LLVMDialect.h"
#include "mlir/Transforms/DialectConversion.h"

using namespace mlir;

namespace {

//===----------------------------------------------------------------------===//
// WrapResultOp -> {val, i16 0}
//===----------------------------------------------------------------------===//

struct WrapResultOpLowering : public OpConversionPattern<cir::WrapResultOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::WrapResultOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto loc = op.getLoc();
    auto resultType = getTypeConverter()->convertType(op.getType());

    // {val, i16 0}
    Value result = rewriter.create<LLVM::UndefOp>(loc, resultType);
    result = rewriter.create<LLVM::InsertValueOp>(
        loc, result, adaptor.getInput(), 0);
    auto zero = rewriter.create<LLVM::ConstantOp>(
        loc, rewriter.getI16Type(), rewriter.getI16IntegerAttr(0));
    result = rewriter.create<LLVM::InsertValueOp>(loc, result, zero, 1);
    rewriter.replaceOp(op, result);
    return success();
  }
};

//===----------------------------------------------------------------------===//
// WrapErrorOp -> {undef, code}
//===----------------------------------------------------------------------===//

struct WrapErrorOpLowering : public OpConversionPattern<cir::WrapErrorOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::WrapErrorOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto loc = op.getLoc();
    auto resultType = getTypeConverter()->convertType(op.getType());

    // {undef, code}
    Value result = rewriter.create<LLVM::UndefOp>(loc, resultType);
    result = rewriter.create<LLVM::InsertValueOp>(
        loc, result, adaptor.getCode(), 1);
    rewriter.replaceOp(op, result);
    return success();
  }
};

//===----------------------------------------------------------------------===//
// IsErrorOp -> extractvalue [1] + icmp ne 0
//===----------------------------------------------------------------------===//

struct IsErrorOpLowering : public OpConversionPattern<cir::IsErrorOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::IsErrorOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto loc = op.getLoc();

    // extractvalue [1] (the error code)
    auto code = rewriter.create<LLVM::ExtractValueOp>(
        loc, adaptor.getInput(), 1);
    // icmp ne 0
    auto zero = rewriter.create<LLVM::ConstantOp>(
        loc, rewriter.getI16Type(), rewriter.getI16IntegerAttr(0));
    rewriter.replaceOpWithNewOp<LLVM::ICmpOp>(
        op, LLVM::ICmpPredicate::ne, code, zero);
    return success();
  }
};

//===----------------------------------------------------------------------===//
// ErrorPayloadOp -> extractvalue [0]
//===----------------------------------------------------------------------===//

struct ErrorPayloadOpLowering
    : public OpConversionPattern<cir::ErrorPayloadOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::ErrorPayloadOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    rewriter.replaceOpWithNewOp<LLVM::ExtractValueOp>(
        op, adaptor.getInput(), 0);
    return success();
  }
};

//===----------------------------------------------------------------------===//
// ErrorCodeOp -> extractvalue [1]
//===----------------------------------------------------------------------===//

struct ErrorCodeOpLowering : public OpConversionPattern<cir::ErrorCodeOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::ErrorCodeOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    rewriter.replaceOpWithNewOp<LLVM::ExtractValueOp>(
        op, adaptor.getInput(), 1);
    return success();
  }
};

} // anonymous namespace

void cot::populateErrorsPatterns(RewritePatternSet &patterns,
                                  TypeConverter &typeConverter) {
  patterns.add<
      WrapResultOpLowering,
      WrapErrorOpLowering,
      IsErrorOpLowering,
      ErrorPayloadOpLowering,
      ErrorCodeOpLowering>(typeConverter, patterns.getContext());
}
