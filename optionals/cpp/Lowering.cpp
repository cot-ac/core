//===- Lowering.cpp - cot-optionals CIR -> LLVM patterns ------*- C++ -*-===//
//
// Four lowering patterns: none, wrap_optional, is_non_null, optional_payload.
// Dual lowering: pointer-like payloads use null-pointer optimization,
// value payloads use struct<(T, i1)>.
// Reference: Zig optional lowering, MLIR ConversionPatterns.
//
//===----------------------------------------------------------------------===//
#include "optionals/Lowering.h"
#include "optionals/Ops.h"
#include "optionals/Types.h"

#include "mlir/Conversion/LLVMCommon/Pattern.h"
#include "mlir/Dialect/LLVMIR/LLVMDialect.h"
#include "mlir/Transforms/DialectConversion.h"

using namespace mlir;

namespace {

/// Get the CIR optional type from a CIR op's result or input.
/// Uses OptionalType::isPointerLike() (backed by PointerLikeTypeInterface)
/// to decide between null-pointer and value-based lowering.
static cir::OptionalType getOptionalType(Type cirType) {
  return mlir::cast<cir::OptionalType>(cirType);
}

//===----------------------------------------------------------------------===//
// NoneOp -> null ptr or {undef, i1 0}
//===----------------------------------------------------------------------===//

struct NoneOpLowering : public OpConversionPattern<cir::NoneOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::NoneOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto loc = op.getLoc();
    auto optType = getOptionalType(op.getType());
    auto resultType = getTypeConverter()->convertType(op.getType());

    if (optType.isPointerLike()) {
      // Null-pointer optimization: none is just a null pointer
      rewriter.replaceOpWithNewOp<LLVM::ZeroOp>(op, resultType);
    } else {
      // Value-based: {undef, i1 0}
      Value result = rewriter.create<LLVM::UndefOp>(loc, resultType);
      auto zero = rewriter.create<LLVM::ConstantOp>(
          loc, rewriter.getI1Type(), rewriter.getBoolAttr(false));
      result = rewriter.create<LLVM::InsertValueOp>(loc, result, zero, 1);
      rewriter.replaceOp(op, result);
    }
    return success();
  }
};

//===----------------------------------------------------------------------===//
// WrapOptionalOp -> identity (ptr-like) or {val, i1 1}
//===----------------------------------------------------------------------===//

struct WrapOptionalOpLowering
    : public OpConversionPattern<cir::WrapOptionalOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::WrapOptionalOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto loc = op.getLoc();
    auto optType = getOptionalType(op.getType());
    auto resultType = getTypeConverter()->convertType(op.getType());

    if (optType.isPointerLike()) {
      // Pointer payload: the pointer IS the optional
      rewriter.replaceOp(op, adaptor.getInput());
    } else {
      // Value-based: {val, i1 1}
      Value result = rewriter.create<LLVM::UndefOp>(loc, resultType);
      result = rewriter.create<LLVM::InsertValueOp>(
          loc, result, adaptor.getInput(), 0);
      auto one = rewriter.create<LLVM::ConstantOp>(
          loc, rewriter.getI1Type(), rewriter.getBoolAttr(true));
      result = rewriter.create<LLVM::InsertValueOp>(loc, result, one, 1);
      rewriter.replaceOp(op, result);
    }
    return success();
  }
};

//===----------------------------------------------------------------------===//
// IsNonNullOp -> icmp ne null (ptr-like) or extractvalue [1]
//===----------------------------------------------------------------------===//

struct IsNonNullOpLowering : public OpConversionPattern<cir::IsNonNullOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::IsNonNullOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto loc = op.getLoc();
    auto optType = getOptionalType(op.getInput().getType());

    if (optType.isPointerLike()) {
      // Pointer: icmp ne null
      auto inputType = adaptor.getInput().getType();
      auto null = rewriter.create<LLVM::ZeroOp>(loc, inputType);
      rewriter.replaceOpWithNewOp<LLVM::ICmpOp>(
          op, LLVM::ICmpPredicate::ne, adaptor.getInput(), null);
    } else {
      // Value-based: extractvalue [1] (the has_value flag)
      rewriter.replaceOpWithNewOp<LLVM::ExtractValueOp>(
          op, adaptor.getInput(), 1);
    }
    return success();
  }
};

//===----------------------------------------------------------------------===//
// OptionalPayloadOp -> identity (ptr-like) or extractvalue [0]
//===----------------------------------------------------------------------===//

struct OptionalPayloadOpLowering
    : public OpConversionPattern<cir::OptionalPayloadOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::OptionalPayloadOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto optType = getOptionalType(op.getInput().getType());

    if (optType.isPointerLike()) {
      // Pointer: the pointer IS the payload
      rewriter.replaceOp(op, adaptor.getInput());
    } else {
      // Value-based: extractvalue [0]
      rewriter.replaceOpWithNewOp<LLVM::ExtractValueOp>(
          op, adaptor.getInput(), 0);
    }
    return success();
  }
};

} // anonymous namespace

void cot::populateOptionalsPatterns(RewritePatternSet &patterns,
                                    TypeConverter &typeConverter) {
  patterns.add<
      NoneOpLowering,
      WrapOptionalOpLowering,
      IsNonNullOpLowering,
      OptionalPayloadOpLowering>(typeConverter, patterns.getContext());
}
