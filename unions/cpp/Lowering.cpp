//===- Lowering.cpp - unions CIR -> LLVM patterns -------------*- C++ -*-===//
//
// Three lowering patterns: union_init, union_tag, union_payload.
// Tagged union layout: struct<(i8 tag, [max_bytes x i8] payload)>.
// Reference: Rust enum lowering, MLIR ConversionPatterns.
//
//===----------------------------------------------------------------------===//
#include "unions/Lowering.h"
#include "unions/Ops.h"
#include "unions/Types.h"

#include "mlir/Conversion/LLVMCommon/Pattern.h"
#include "mlir/Dialect/LLVMIR/LLVMDialect.h"
#include "mlir/Transforms/DialectConversion.h"

using namespace mlir;

namespace {

//===----------------------------------------------------------------------===//
// UnionInitOp -> undef + insertvalue tag + store/load payload bytes
//===----------------------------------------------------------------------===//

struct UnionInitOpLowering : public OpConversionPattern<cir::UnionInitOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::UnionInitOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto loc = op.getLoc();
    auto unionType = mlir::cast<cir::TaggedUnionType>(op.getType());
    auto resultType = getTypeConverter()->convertType(op.getType());

    // Find variant index
    auto idx = unionType.getVariantIndex(op.getVariant());

    // Create undef struct
    Value result = rewriter.create<LLVM::UndefOp>(loc, resultType);

    // Insert tag (i8)
    auto tagVal = rewriter.create<LLVM::ConstantOp>(
        loc, rewriter.getI8Type(), rewriter.getI8IntegerAttr(idx));
    result = rewriter.create<LLVM::InsertValueOp>(loc, result, tagVal, 0);

    // Insert payload if present — alloca-store-load bitcast pattern
    if (adaptor.getPayload()) {
      auto payload = adaptor.getPayload();
      auto payloadType = payload.getType();

      // Alloca for the payload, store into it
      auto one = rewriter.create<LLVM::ConstantOp>(
          loc, rewriter.getI64Type(), rewriter.getI64IntegerAttr(1));
      auto payloadPtr = rewriter.create<LLVM::AllocaOp>(
          loc, LLVM::LLVMPointerType::get(rewriter.getContext()),
          payloadType, one);
      rewriter.create<LLVM::StoreOp>(loc, payload, payloadPtr);

      // Load as byte array (reinterpret cast via pointer)
      auto structType = mlir::cast<LLVM::LLVMStructType>(resultType);
      auto byteArrayType = structType.getBody()[1];
      auto loaded = rewriter.create<LLVM::LoadOp>(
          loc, byteArrayType, payloadPtr);
      result = rewriter.create<LLVM::InsertValueOp>(loc, result, loaded, 1);
    }

    rewriter.replaceOp(op, result);
    return success();
  }
};

//===----------------------------------------------------------------------===//
// UnionTagOp -> extractvalue [0]
//===----------------------------------------------------------------------===//

struct UnionTagOpLowering : public OpConversionPattern<cir::UnionTagOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::UnionTagOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    rewriter.replaceOpWithNewOp<LLVM::ExtractValueOp>(
        op, adaptor.getInput(), 0);
    return success();
  }
};

//===----------------------------------------------------------------------===//
// UnionPayloadOp -> extractvalue [1] (byte array) + load as target type
//===----------------------------------------------------------------------===//

struct UnionPayloadOpLowering
    : public OpConversionPattern<cir::UnionPayloadOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::UnionPayloadOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto loc = op.getLoc();
    auto resultType = getTypeConverter()->convertType(op.getType());

    // Extract byte array
    auto byteArray = rewriter.create<LLVM::ExtractValueOp>(
        loc, adaptor.getInput(), 1);

    // Alloca-store-load bitcast: store byte array, load as target type
    auto byteArrayType = byteArray.getType();
    auto one = rewriter.create<LLVM::ConstantOp>(
        loc, rewriter.getI64Type(), rewriter.getI64IntegerAttr(1));
    auto ptr = rewriter.create<LLVM::AllocaOp>(
        loc, LLVM::LLVMPointerType::get(rewriter.getContext()),
        byteArrayType, one);
    rewriter.create<LLVM::StoreOp>(loc, byteArray, ptr);
    auto loaded = rewriter.create<LLVM::LoadOp>(loc, resultType, ptr);

    rewriter.replaceOp(op, loaded);
    return success();
  }
};

} // anonymous namespace

void cot::populateUnionsPatterns(RewritePatternSet &patterns,
                                  TypeConverter &typeConverter) {
  patterns.add<
      UnionInitOpLowering,
      UnionTagOpLowering,
      UnionPayloadOpLowering>(typeConverter, patterns.getContext());
}
