//===- Lowering.cpp - cot-slices CIR → LLVM patterns ----------*- C++ -*-===//
//
// Five lowering patterns: string_constant, slice_ptr, slice_len,
// slice_elem, array_to_slice.
// Reference: MLIR ConversionPatterns, Rust slice lowering.
//
//===----------------------------------------------------------------------===//
#include "slices/Lowering.h"
#include "slices/Ops.h"
#include "slices/Types.h"

#include "mlir/Conversion/LLVMCommon/Pattern.h"
#include "mlir/Dialect/LLVMIR/LLVMDialect.h"
#include "mlir/Transforms/DialectConversion.h"

using namespace mlir;

namespace {

/// Get the LLVM type for a slice: {!llvm.ptr, i64}
static LLVM::LLVMStructType getSliceLLVMType(MLIRContext *ctx) {
  auto ptr = LLVM::LLVMPointerType::get(ctx);
  auto i64 = IntegerType::get(ctx, 64);
  return LLVM::LLVMStructType::getLiteral(ctx, {ptr, i64});
}

//===----------------------------------------------------------------------===//
// StringConstantOp → global + addressof + struct{ptr, len}
//===----------------------------------------------------------------------===//

struct StringConstantOpLowering
    : public OpConversionPattern<cir::StringConstantOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::StringConstantOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto loc = op.getLoc();
    auto ctx = getContext();
    auto strValue = op.getValue();

    // Find or create the parent module
    auto moduleOp = op->getParentOfType<ModuleOp>();

    // Create a unique global name
    static int counter = 0;
    std::string globalName = ".str." + std::to_string(counter++);

    // Create LLVM global with null-terminated string
    auto i8Type = IntegerType::get(ctx, 8);
    auto strLen = strValue.size();
    auto arrayType = LLVM::LLVMArrayType::get(i8Type, strLen + 1);

    // Insert global at module level (guard restores insertion point)
    {
      OpBuilder::InsertionGuard guard(rewriter);
      rewriter.setInsertionPointToStart(moduleOp.getBody());
      rewriter.create<LLVM::GlobalOp>(
          loc, arrayType, /*isConstant=*/true,
          LLVM::Linkage::Internal, globalName,
          rewriter.getStringAttr(std::string(strValue) + '\0'));
    }

    // Get address of the global
    auto ptrType = LLVM::LLVMPointerType::get(ctx);
    auto addr = rewriter.create<LLVM::AddressOfOp>(
        loc, ptrType, globalName);

    // Build slice struct: {ptr, len}
    auto sliceType = getSliceLLVMType(ctx);
    auto lenVal = rewriter.create<LLVM::ConstantOp>(
        loc, rewriter.getI64Type(),
        rewriter.getI64IntegerAttr(strLen));

    Value result = rewriter.create<LLVM::UndefOp>(loc, sliceType);
    result = rewriter.create<LLVM::InsertValueOp>(loc, result, addr, 0);
    result = rewriter.create<LLVM::InsertValueOp>(loc, result, lenVal, 1);

    rewriter.replaceOp(op, result);
    return success();
  }
};

//===----------------------------------------------------------------------===//
// SlicePtrOp → extractvalue [0]
//===----------------------------------------------------------------------===//

struct SlicePtrOpLowering : public OpConversionPattern<cir::SlicePtrOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::SlicePtrOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    rewriter.replaceOpWithNewOp<LLVM::ExtractValueOp>(
        op, adaptor.getInput(), 0);
    return success();
  }
};

//===----------------------------------------------------------------------===//
// SliceLenOp → extractvalue [1]
//===----------------------------------------------------------------------===//

struct SliceLenOpLowering : public OpConversionPattern<cir::SliceLenOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::SliceLenOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    rewriter.replaceOpWithNewOp<LLVM::ExtractValueOp>(
        op, adaptor.getInput(), 1);
    return success();
  }
};

//===----------------------------------------------------------------------===//
// SliceElemOp → extract ptr + GEP + load (Errata E6: has MemRead)
//===----------------------------------------------------------------------===//

struct SliceElemOpLowering : public OpConversionPattern<cir::SliceElemOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::SliceElemOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto loc = op.getLoc();
    auto ctx = getContext();

    // Extract pointer from slice
    auto ptr = rewriter.create<LLVM::ExtractValueOp>(
        loc, adaptor.getInput(), 0);

    // GEP to the element
    auto elemType = getTypeConverter()->convertType(op.getType());
    auto ptrType = LLVM::LLVMPointerType::get(ctx);
    auto gep = rewriter.create<LLVM::GEPOp>(
        loc, ptrType, elemType, ptr,
        ArrayRef<LLVM::GEPArg>{adaptor.getIdx()});

    // Load the element
    rewriter.replaceOpWithNewOp<LLVM::LoadOp>(op, elemType, gep);
    return success();
  }
};

//===----------------------------------------------------------------------===//
// ArrayToSliceOp → GEP(base, start) + (end - start) → struct{ptr, len}
//===----------------------------------------------------------------------===//

struct ArrayToSliceOpLowering
    : public OpConversionPattern<cir::ArrayToSliceOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::ArrayToSliceOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto loc = op.getLoc();
    auto ctx = getContext();

    auto sliceType = mlir::cast<cir::SliceType>(op.getType());
    auto elemType = getTypeConverter()->convertType(
        sliceType.getElementType());
    auto ptrType = LLVM::LLVMPointerType::get(ctx);

    // GEP: base + start
    auto ptr = rewriter.create<LLVM::GEPOp>(
        loc, ptrType, elemType, adaptor.getBase(),
        ArrayRef<LLVM::GEPArg>{adaptor.getStart()});

    // Length: end - start
    auto len = rewriter.create<LLVM::SubOp>(
        loc, adaptor.getEnd(), adaptor.getStart());

    // Build slice struct
    auto llvmSliceType = getSliceLLVMType(ctx);
    Value result = rewriter.create<LLVM::UndefOp>(loc, llvmSliceType);
    result = rewriter.create<LLVM::InsertValueOp>(loc, result, ptr, 0);
    result = rewriter.create<LLVM::InsertValueOp>(loc, result, len, 1);

    rewriter.replaceOp(op, result);
    return success();
  }
};

} // anonymous namespace

void cot::populateSlicesPatterns(RewritePatternSet &patterns,
                                 TypeConverter &typeConverter) {
  patterns.add<
      StringConstantOpLowering,
      SlicePtrOpLowering,
      SliceLenOpLowering,
      SliceElemOpLowering,
      ArrayToSliceOpLowering>(typeConverter, patterns.getContext());
}
