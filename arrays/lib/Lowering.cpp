//===- Lowering.cpp - cot-arrays CIR → LLVM patterns ----------*- C++ -*-===//
//
// Three lowering patterns: array_init, elem_val, elem_ptr.
// Reference: MLIR ConversionPatterns, LLVM insertvalue/extractvalue/GEP.
//
//===----------------------------------------------------------------------===//
#include "arrays/Lowering.h"
#include "arrays/Ops.h"
#include "arrays/Types.h"

#include "mlir/Conversion/LLVMCommon/Pattern.h"
#include "mlir/Dialect/LLVMIR/LLVMDialect.h"
#include "mlir/Transforms/DialectConversion.h"

using namespace mlir;

namespace {

//===----------------------------------------------------------------------===//
// ArrayInitOp → undef + insertvalue chain
//===----------------------------------------------------------------------===//

struct ArrayInitOpLowering : public OpConversionPattern<cir::ArrayInitOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::ArrayInitOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto loc = op.getLoc();
    auto resultType = getTypeConverter()->convertType(op.getType());

    // Start with undef
    Value result = rewriter.create<LLVM::UndefOp>(loc, resultType);

    // Insert each element
    auto elems = adaptor.getElements();
    for (unsigned i = 0; i < elems.size(); i++) {
      result = rewriter.create<LLVM::InsertValueOp>(
          loc, result, elems[i], i);
    }

    rewriter.replaceOp(op, result);
    return success();
  }
};

//===----------------------------------------------------------------------===//
// ElemValOp → llvm.extractvalue
//===----------------------------------------------------------------------===//

struct ElemValOpLowering : public OpConversionPattern<cir::ElemValOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::ElemValOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto idx = op.getIndex();
    rewriter.replaceOpWithNewOp<LLVM::ExtractValueOp>(
        op, adaptor.getInput(), idx);
    return success();
  }
};

//===----------------------------------------------------------------------===//
// ElemPtrOp → llvm.getelementptr [0, idx]
//===----------------------------------------------------------------------===//

struct ElemPtrOpLowering : public OpConversionPattern<cir::ElemPtrOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::ElemPtrOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto arrayType = getTypeConverter()->convertType(op.getArrayType());
    auto resultType = LLVM::LLVMPointerType::get(getContext());

    // GEP: base[0][idx] — first index steps through the pointer,
    // second indexes into the array
    rewriter.replaceOpWithNewOp<LLVM::GEPOp>(
        op, resultType, arrayType, adaptor.getBase(),
        ArrayRef<LLVM::GEPArg>{0, adaptor.getIdx()});
    return success();
  }
};

} // anonymous namespace

void cot::populateArraysPatterns(RewritePatternSet &patterns,
                                 TypeConverter &typeConverter) {
  patterns.add<
      ArrayInitOpLowering,
      ElemValOpLowering,
      ElemPtrOpLowering>(typeConverter, patterns.getContext());
}
