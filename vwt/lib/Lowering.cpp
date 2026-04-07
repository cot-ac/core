//===- Lowering.cpp - VWT CIR -> LLVM patterns ----------------*- C++ -*-===//
//
// VWT ops lower to GEP + load (data) or GEP + load + indirect call (functions).
// The VWT is treated as an array of i64 (8-byte words).
//
// VWT layout indices:
//   [0] initBufferWithCopy  fn ptr
//   [1] destroy             fn ptr
//   [2] initializeWithCopy  fn ptr
//   [3] assignWithCopy      fn ptr
//   [4] initializeWithTake  fn ptr
//   [5] assignWithTake      fn ptr
//   [6] getEnumTagSingle    fn ptr  (unused here)
//   [7] storeEnumTagSingle  fn ptr  (unused here)
//   [8] size                i64
//   [9] stride              i64
//   [10] flags              i32 (alignment mask in low 8 bits)
//   [11] extraInhabitants   i32
//
// Reference: Swift include/swift/ABI/ValueWitnessTable.h
//
//===----------------------------------------------------------------------===//
#include "vwt/Lowering.h"
#include "vwt/Ops.h"

#include "mlir/Conversion/LLVMCommon/Pattern.h"
#include "mlir/Dialect/LLVMIR/LLVMDialect.h"
#include "mlir/Transforms/DialectConversion.h"

using namespace mlir;

namespace {

/// Helper: GEP into VWT (treated as i64 array) at the given index, then load.
static Value vwtLoadField(ConversionPatternRewriter &rewriter, Location loc,
                          Value vwt, int64_t index, Type resultType) {
  auto ctx = rewriter.getContext();
  auto i64Ty = rewriter.getI64Type();
  auto ptrTy = LLVM::LLVMPointerType::get(ctx);

  // GEP: &vwt[index] (vwt is ptr to array of i64-sized words)
  auto indexVal = rewriter.create<LLVM::ConstantOp>(
      loc, i64Ty, rewriter.getI64IntegerAttr(index));
  auto elemPtr = rewriter.create<LLVM::GEPOp>(
      loc, ptrTy, i64Ty, vwt, ValueRange{indexVal});

  // Load the field
  return rewriter.create<LLVM::LoadOp>(loc, resultType, elemPtr);
}

//===----------------------------------------------------------------------===//
// VWTSizeOp → GEP VWT[8] + load i64
//===----------------------------------------------------------------------===//

struct VWTSizeOpLowering : public OpConversionPattern<cir::VWTSizeOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      cir::VWTSizeOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto result = vwtLoadField(rewriter, op.getLoc(), adaptor.getVwt(),
                                8, rewriter.getI64Type());
    rewriter.replaceOp(op, result);
    return success();
  }
};

//===----------------------------------------------------------------------===//
// VWTStrideOp → GEP VWT[9] + load i64
//===----------------------------------------------------------------------===//

struct VWTStrideOpLowering : public OpConversionPattern<cir::VWTStrideOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      cir::VWTStrideOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto result = vwtLoadField(rewriter, op.getLoc(), adaptor.getVwt(),
                                9, rewriter.getI64Type());
    rewriter.replaceOp(op, result);
    return success();
  }
};

//===----------------------------------------------------------------------===//
// VWTAlignOp → GEP VWT[10] + load i32 + mask + add 1
//===----------------------------------------------------------------------===//

struct VWTAlignOpLowering : public OpConversionPattern<cir::VWTAlignOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      cir::VWTAlignOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto loc = op.getLoc();
    auto i64Ty = rewriter.getI64Type();
    auto i32Ty = rewriter.getI32Type();

    // Load flags field (i32 at index 10)
    auto flags = vwtLoadField(rewriter, loc, adaptor.getVwt(), 10, i32Ty);

    // Mask low 8 bits (alignment mask)
    auto mask = rewriter.create<LLVM::ConstantOp>(
        loc, i32Ty, rewriter.getI32IntegerAttr(0xFF));
    auto alignMask = rewriter.create<LLVM::AndOp>(loc, flags, mask);

    // Zero-extend to i64
    auto alignMask64 = rewriter.create<LLVM::ZExtOp>(loc, i64Ty, alignMask);

    // Add 1: alignment = alignMask + 1
    auto one = rewriter.create<LLVM::ConstantOp>(
        loc, i64Ty, rewriter.getI64IntegerAttr(1));
    auto alignment = rewriter.create<LLVM::AddOp>(loc, alignMask64, one);

    rewriter.replaceOp(op, alignment.getResult());
    return success();
  }
};

//===----------------------------------------------------------------------===//
// VWTDestroyOp → load fn ptr from VWT[1], call indirectly
//===----------------------------------------------------------------------===//

struct VWTDestroyOpLowering : public OpConversionPattern<cir::VWTDestroyOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      cir::VWTDestroyOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto loc = op.getLoc();
    auto ctx = rewriter.getContext();
    auto ptrTy = LLVM::LLVMPointerType::get(ctx);

    // Load destroy fn ptr from VWT[1]
    auto fnPtr = vwtLoadField(rewriter, loc, adaptor.getVwt(), 1, ptrTy);

    // Indirect call: destroy(ptr, metadata) → void
    // First operand is the callee (fn ptr), rest are args
    auto voidTy = LLVM::LLVMVoidType::get(ctx);
    auto fnTy = LLVM::LLVMFunctionType::get(voidTy, {ptrTy, ptrTy});
    SmallVector<Value> operands = {fnPtr, adaptor.getPtr(), adaptor.getVwt()};
    rewriter.create<LLVM::CallOp>(loc, fnTy, operands);
    rewriter.eraseOp(op);
    return success();
  }
};

//===----------------------------------------------------------------------===//
// VWTCopyOp → load fn ptr from VWT[2], call indirectly
//===----------------------------------------------------------------------===//

struct VWTCopyOpLowering : public OpConversionPattern<cir::VWTCopyOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      cir::VWTCopyOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto loc = op.getLoc();
    auto ctx = rewriter.getContext();
    auto ptrTy = LLVM::LLVMPointerType::get(ctx);

    auto fnPtr = vwtLoadField(rewriter, loc, adaptor.getVwt(), 2, ptrTy);
    auto fnTy = LLVM::LLVMFunctionType::get(ptrTy, {ptrTy, ptrTy, ptrTy});
    SmallVector<Value> operands = {
        fnPtr, adaptor.getDst(), adaptor.getSrc(), adaptor.getVwt()};
    rewriter.create<LLVM::CallOp>(loc, fnTy, operands);
    rewriter.eraseOp(op);
    return success();
  }
};

struct VWTMoveOpLowering : public OpConversionPattern<cir::VWTMoveOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      cir::VWTMoveOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto loc = op.getLoc();
    auto ctx = rewriter.getContext();
    auto ptrTy = LLVM::LLVMPointerType::get(ctx);

    auto fnPtr = vwtLoadField(rewriter, loc, adaptor.getVwt(), 4, ptrTy);
    auto fnTy = LLVM::LLVMFunctionType::get(ptrTy, {ptrTy, ptrTy, ptrTy});
    SmallVector<Value> operands = {
        fnPtr, adaptor.getDst(), adaptor.getSrc(), adaptor.getVwt()};
    rewriter.create<LLVM::CallOp>(loc, fnTy, operands);
    rewriter.eraseOp(op);
    return success();
  }
};

struct VWTInitBufferOpLowering
    : public OpConversionPattern<cir::VWTInitBufferOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      cir::VWTInitBufferOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto loc = op.getLoc();
    auto ctx = rewriter.getContext();
    auto ptrTy = LLVM::LLVMPointerType::get(ctx);

    auto fnPtr = vwtLoadField(rewriter, loc, adaptor.getVwt(), 0, ptrTy);
    auto fnTy = LLVM::LLVMFunctionType::get(ptrTy, {ptrTy, ptrTy, ptrTy});
    SmallVector<Value> operands = {
        fnPtr, adaptor.getDst(), adaptor.getSrc(), adaptor.getVwt()};
    auto call = rewriter.create<LLVM::CallOp>(loc, fnTy, operands);
    rewriter.replaceOp(op, call.getResult());
    return success();
  }
};

} // anonymous namespace

void cot::populateVWTPatterns(RewritePatternSet &patterns,
                              TypeConverter &typeConverter) {
  patterns.add<
      VWTSizeOpLowering,
      VWTStrideOpLowering,
      VWTAlignOpLowering,
      VWTDestroyOpLowering,
      VWTCopyOpLowering,
      VWTMoveOpLowering,
      VWTInitBufferOpLowering>(typeConverter, patterns.getContext());
}
