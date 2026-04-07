//===- Lowering.cpp - cot-structs CIR → LLVM patterns ---------*- C++ -*-===//
//
// Three lowering patterns: struct_init, field_val, field_ptr.
// Reference: MLIR ConversionPatterns, LLVM insertvalue/extractvalue/GEP.
//
//===----------------------------------------------------------------------===//
#include "structs/Lowering.h"
#include "structs/Ops.h"
#include "structs/Types.h"

#include "mlir/Conversion/LLVMCommon/Pattern.h"
#include "mlir/Dialect/LLVMIR/LLVMDialect.h"
#include "mlir/Transforms/DialectConversion.h"

using namespace mlir;

namespace {

//===----------------------------------------------------------------------===//
// StructInitOp → undef + insertvalue chain
//===----------------------------------------------------------------------===//

struct StructInitOpLowering : public OpConversionPattern<cir::StructInitOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::StructInitOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto loc = op.getLoc();
    auto resultType = getTypeConverter()->convertType(op.getType());

    // Start with undef
    Value result = rewriter.create<LLVM::UndefOp>(loc, resultType);

    // Insert each field
    auto fields = adaptor.getFields();
    for (unsigned i = 0; i < fields.size(); i++) {
      result = rewriter.create<LLVM::InsertValueOp>(
          loc, result, fields[i], i);
    }

    rewriter.replaceOp(op, result);
    return success();
  }
};

//===----------------------------------------------------------------------===//
// FieldValOp → llvm.extractvalue
//===----------------------------------------------------------------------===//

struct FieldValOpLowering : public OpConversionPattern<cir::FieldValOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::FieldValOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto idx = op.getIndex();
    rewriter.replaceOpWithNewOp<LLVM::ExtractValueOp>(
        op, adaptor.getInput(), idx);
    return success();
  }
};

//===----------------------------------------------------------------------===//
// FieldPtrOp → llvm.getelementptr [0, idx]
//===----------------------------------------------------------------------===//

struct FieldPtrOpLowering : public OpConversionPattern<cir::FieldPtrOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::FieldPtrOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto idx = op.getIndex();

    // Convert the struct type to get the LLVM struct layout for GEP
    auto structType = getTypeConverter()->convertType(op.getStructType());
    auto resultType = LLVM::LLVMPointerType::get(getContext());

    rewriter.replaceOpWithNewOp<LLVM::GEPOp>(
        op, resultType, structType, adaptor.getBase(),
        ArrayRef<LLVM::GEPArg>{0, static_cast<int32_t>(idx)});
    return success();
  }
};

} // anonymous namespace

void cot::populateStructsPatterns(RewritePatternSet &patterns,
                                  TypeConverter &typeConverter) {
  patterns.add<
      StructInitOpLowering,
      FieldValOpLowering,
      FieldPtrOpLowering>(typeConverter, patterns.getContext());
}
