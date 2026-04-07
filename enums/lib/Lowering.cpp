//===- Lowering.cpp - enums CIR -> LLVM patterns --------------*- C++ -*-===//
//
// Two lowering patterns: enum_constant, enum_value.
// Enum IS the tag integer — lowering is trivial.
// Reference: Rust C-like enum lowering to integer constants.
//
//===----------------------------------------------------------------------===//
#include "enums/Lowering.h"
#include "enums/Ops.h"
#include "enums/Types.h"

#include "mlir/Conversion/LLVMCommon/Pattern.h"
#include "mlir/Dialect/LLVMIR/LLVMDialect.h"
#include "mlir/Transforms/DialectConversion.h"

using namespace mlir;

namespace {

//===----------------------------------------------------------------------===//
// EnumConstantOp -> llvm.mlir.constant (variant index)
//===----------------------------------------------------------------------===//

struct EnumConstantOpLowering
    : public OpConversionPattern<cir::EnumConstantOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::EnumConstantOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto enumType = mlir::cast<cir::EnumType>(op.getType());
    auto tagType = getTypeConverter()->convertType(enumType.getTagType());
    auto variant = op.getVariant();

    // Find variant index
    int64_t index = -1;
    auto variants = enumType.getVariants();
    for (unsigned i = 0; i < variants.size(); i++) {
      if (variants[i].getValue() == variant) {
        index = i;
        break;
      }
    }

    auto attr = rewriter.getIntegerAttr(tagType, index);
    rewriter.replaceOpWithNewOp<LLVM::ConstantOp>(op, tagType, attr);
    return success();
  }
};

//===----------------------------------------------------------------------===//
// EnumValueOp -> identity (enum IS the integer after type conversion)
//===----------------------------------------------------------------------===//

struct EnumValueOpLowering : public OpConversionPattern<cir::EnumValueOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::EnumValueOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    // After type conversion, the enum is already the tag integer.
    // Just forward the converted operand.
    rewriter.replaceOp(op, adaptor.getInput());
    return success();
  }
};

} // anonymous namespace

void cot::populateEnumsPatterns(RewritePatternSet &patterns,
                                TypeConverter &typeConverter) {
  patterns.add<
      EnumConstantOpLowering,
      EnumValueOpLowering>(typeConverter, patterns.getContext());
}
