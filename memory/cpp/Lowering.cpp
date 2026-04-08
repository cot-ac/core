//===- Lowering.cpp - cot-memory CIR → LLVM patterns ----------*- C++ -*-===//
//
// Five lowering patterns: alloca, store, load, addr_of, deref.
// Reference: MLIR ConversionPatterns, Flang FIR→LLVM lowering.
//
//===----------------------------------------------------------------------===//
#include "memory/Lowering.h"
#include "memory/Ops.h"
#include "memory/Types.h"

#include "mlir/Conversion/LLVMCommon/Pattern.h"
#include "mlir/Dialect/LLVMIR/LLVMDialect.h"
#include "mlir/Transforms/DialectConversion.h"

using namespace mlir;

namespace {

//===----------------------------------------------------------------------===//
// AllocaOp → llvm.alloca (count=1)
//===----------------------------------------------------------------------===//

struct AllocaOpLowering : public OpConversionPattern<cir::AllocaOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::AllocaOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto loc = op.getLoc();
    auto elemType = getTypeConverter()->convertType(op.getElemType());

    // LLVM alloca needs a count (always 1 for CIR alloca)
    auto one = rewriter.create<LLVM::ConstantOp>(
        loc, rewriter.getI64Type(), rewriter.getI64IntegerAttr(1));

    rewriter.replaceOpWithNewOp<LLVM::AllocaOp>(
        op, LLVM::LLVMPointerType::get(getContext()), elemType, one);
    return success();
  }
};

//===----------------------------------------------------------------------===//
// StoreOp → llvm.store
//===----------------------------------------------------------------------===//

struct StoreOpLowering : public OpConversionPattern<cir::StoreOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::StoreOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    rewriter.replaceOpWithNewOp<LLVM::StoreOp>(
        op, adaptor.getValue(), adaptor.getAddr());
    return success();
  }
};

//===----------------------------------------------------------------------===//
// LoadOp → llvm.load
//===----------------------------------------------------------------------===//

struct LoadOpLowering : public OpConversionPattern<cir::LoadOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::LoadOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto resultType = getTypeConverter()->convertType(op.getType());
    rewriter.replaceOpWithNewOp<LLVM::LoadOp>(
        op, resultType, adaptor.getAddr());
    return success();
  }
};

//===----------------------------------------------------------------------===//
// AddrOfOp → identity (both !cir.ptr and !cir.ref<T> lower to !llvm.ptr)
//===----------------------------------------------------------------------===//

struct AddrOfOpLowering : public OpConversionPattern<cir::AddrOfOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::AddrOfOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    rewriter.replaceOp(op, adaptor.getAddr());
    return success();
  }
};

//===----------------------------------------------------------------------===//
// DerefOp → llvm.load (errata E6: NOT Pure — performs a memory read)
//===----------------------------------------------------------------------===//

struct DerefOpLowering : public OpConversionPattern<cir::DerefOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::DerefOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto resultType = getTypeConverter()->convertType(op.getType());
    rewriter.replaceOpWithNewOp<LLVM::LoadOp>(
        op, resultType, adaptor.getRef());
    return success();
  }
};

} // anonymous namespace

void cot::populateMemoryPatterns(RewritePatternSet &patterns,
                                 TypeConverter &typeConverter) {
  patterns.add<
      AllocaOpLowering,
      StoreOpLowering,
      LoadOpLowering,
      AddrOfOpLowering,
      DerefOpLowering>(typeConverter, patterns.getContext());
}
