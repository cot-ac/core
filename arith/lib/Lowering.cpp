//===- Lowering.cpp - cot-core CIR -> LLVM patterns -----------*- C++ -*-===//
//
// Reference: mlir/lib/Conversion/ArithToLLVM/ArithToLLVM.cpp
//
//===----------------------------------------------------------------------===//
#include "arith/Lowering.h"
#include "arith/Ops.h"

#include "mlir/Dialect/LLVMIR/LLVMDialect.h"
#include "mlir/Transforms/DialectConversion.h"

using namespace mlir;
using namespace cir;

//===----------------------------------------------------------------------===//
// Pattern: cir.constant -> llvm.mlir.constant
//===----------------------------------------------------------------------===//

struct ConstantOpLowering : public OpConversionPattern<ConstantOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      ConstantOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto resultType = getTypeConverter()->convertType(op.getType());
    rewriter.replaceOpWithNewOp<LLVM::ConstantOp>(
        op, resultType, op.getValue());
    return success();
  }
};

//===----------------------------------------------------------------------===//
// Arithmetic patterns
//===----------------------------------------------------------------------===//

struct AddOpLowering : public OpConversionPattern<AddOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      AddOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    rewriter.replaceOpWithNewOp<LLVM::AddOp>(
        op, adaptor.getLhs(), adaptor.getRhs());
    return success();
  }
};

struct SubOpLowering : public OpConversionPattern<SubOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      SubOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    rewriter.replaceOpWithNewOp<LLVM::SubOp>(
        op, adaptor.getLhs(), adaptor.getRhs());
    return success();
  }
};

struct MulOpLowering : public OpConversionPattern<MulOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      MulOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    rewriter.replaceOpWithNewOp<LLVM::MulOp>(
        op, adaptor.getLhs(), adaptor.getRhs());
    return success();
  }
};

struct DivSIOpLowering : public OpConversionPattern<DivSIOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      DivSIOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    rewriter.replaceOpWithNewOp<LLVM::SDivOp>(
        op, adaptor.getLhs(), adaptor.getRhs());
    return success();
  }
};

struct DivUIOpLowering : public OpConversionPattern<DivUIOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      DivUIOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    rewriter.replaceOpWithNewOp<LLVM::UDivOp>(
        op, adaptor.getLhs(), adaptor.getRhs());
    return success();
  }
};

struct DivFOpLowering : public OpConversionPattern<DivFOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      DivFOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    rewriter.replaceOpWithNewOp<LLVM::FDivOp>(
        op, adaptor.getLhs(), adaptor.getRhs());
    return success();
  }
};

struct RemSIOpLowering : public OpConversionPattern<RemSIOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      RemSIOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    rewriter.replaceOpWithNewOp<LLVM::SRemOp>(
        op, adaptor.getLhs(), adaptor.getRhs());
    return success();
  }
};

struct RemUIOpLowering : public OpConversionPattern<RemUIOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      RemUIOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    rewriter.replaceOpWithNewOp<LLVM::URemOp>(
        op, adaptor.getLhs(), adaptor.getRhs());
    return success();
  }
};

struct RemFOpLowering : public OpConversionPattern<RemFOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      RemFOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    rewriter.replaceOpWithNewOp<LLVM::FRemOp>(
        op, adaptor.getLhs(), adaptor.getRhs());
    return success();
  }
};

//===----------------------------------------------------------------------===//
// Negation patterns
//===----------------------------------------------------------------------===//

struct NegOpLowering : public OpConversionPattern<NegOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      NegOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto zero = rewriter.create<LLVM::ConstantOp>(
        op.getLoc(), adaptor.getOperand().getType(),
        rewriter.getIntegerAttr(adaptor.getOperand().getType(), 0));
    rewriter.replaceOpWithNewOp<LLVM::SubOp>(op, zero, adaptor.getOperand());
    return success();
  }
};

struct NegFOpLowering : public OpConversionPattern<NegFOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      NegFOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    rewriter.replaceOpWithNewOp<LLVM::FNegOp>(op, adaptor.getOperand());
    return success();
  }
};

//===----------------------------------------------------------------------===//
// Bitwise patterns
//===----------------------------------------------------------------------===//

struct BitAndOpLowering : public OpConversionPattern<BitAndOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      BitAndOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    rewriter.replaceOpWithNewOp<LLVM::AndOp>(
        op, adaptor.getLhs(), adaptor.getRhs());
    return success();
  }
};

struct BitOrOpLowering : public OpConversionPattern<BitOrOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      BitOrOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    rewriter.replaceOpWithNewOp<LLVM::OrOp>(
        op, adaptor.getLhs(), adaptor.getRhs());
    return success();
  }
};

struct BitXorOpLowering : public OpConversionPattern<BitXorOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      BitXorOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    rewriter.replaceOpWithNewOp<LLVM::XOrOp>(
        op, adaptor.getLhs(), adaptor.getRhs());
    return success();
  }
};

struct BitNotOpLowering : public OpConversionPattern<BitNotOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      BitNotOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto allOnes = rewriter.create<LLVM::ConstantOp>(
        op.getLoc(), adaptor.getOperand().getType(),
        rewriter.getIntegerAttr(adaptor.getOperand().getType(), -1));
    rewriter.replaceOpWithNewOp<LLVM::XOrOp>(
        op, adaptor.getOperand(), allOnes);
    return success();
  }
};

//===----------------------------------------------------------------------===//
// Shift patterns
//===----------------------------------------------------------------------===//

struct ShlOpLowering : public OpConversionPattern<ShlOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      ShlOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    rewriter.replaceOpWithNewOp<LLVM::ShlOp>(
        op, adaptor.getLhs(), adaptor.getRhs());
    return success();
  }
};

struct ShrOpLowering : public OpConversionPattern<ShrOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      ShrOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    rewriter.replaceOpWithNewOp<LLVM::LShrOp>(
        op, adaptor.getLhs(), adaptor.getRhs());
    return success();
  }
};

struct ShrSOpLowering : public OpConversionPattern<ShrSOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      ShrSOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    rewriter.replaceOpWithNewOp<LLVM::AShrOp>(
        op, adaptor.getLhs(), adaptor.getRhs());
    return success();
  }
};

//===----------------------------------------------------------------------===//
// Comparison patterns — E4: explicit predicate map, NOT static_cast
//===----------------------------------------------------------------------===//

struct CmpOpLowering : public OpConversionPattern<CmpOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      CmpOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    LLVM::ICmpPredicate llvmPred;
    switch (op.getPredicate()) {
    case CmpIPredicate::eq:  llvmPred = LLVM::ICmpPredicate::eq; break;
    case CmpIPredicate::ne:  llvmPred = LLVM::ICmpPredicate::ne; break;
    case CmpIPredicate::slt: llvmPred = LLVM::ICmpPredicate::slt; break;
    case CmpIPredicate::sle: llvmPred = LLVM::ICmpPredicate::sle; break;
    case CmpIPredicate::sgt: llvmPred = LLVM::ICmpPredicate::sgt; break;
    case CmpIPredicate::sge: llvmPred = LLVM::ICmpPredicate::sge; break;
    case CmpIPredicate::ult: llvmPred = LLVM::ICmpPredicate::ult; break;
    case CmpIPredicate::ule: llvmPred = LLVM::ICmpPredicate::ule; break;
    case CmpIPredicate::ugt: llvmPred = LLVM::ICmpPredicate::ugt; break;
    case CmpIPredicate::uge: llvmPred = LLVM::ICmpPredicate::uge; break;
    }
    rewriter.replaceOpWithNewOp<LLVM::ICmpOp>(
        op, llvmPred, adaptor.getLhs(), adaptor.getRhs());
    return success();
  }
};

struct CmpFOpLowering : public OpConversionPattern<CmpFOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      CmpFOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    LLVM::FCmpPredicate llvmPred;
    switch (op.getPredicate()) {
    case CmpFPredicate::oeq: llvmPred = LLVM::FCmpPredicate::oeq; break;
    case CmpFPredicate::ogt: llvmPred = LLVM::FCmpPredicate::ogt; break;
    case CmpFPredicate::oge: llvmPred = LLVM::FCmpPredicate::oge; break;
    case CmpFPredicate::olt: llvmPred = LLVM::FCmpPredicate::olt; break;
    case CmpFPredicate::ole: llvmPred = LLVM::FCmpPredicate::ole; break;
    case CmpFPredicate::one: llvmPred = LLVM::FCmpPredicate::one; break;
    case CmpFPredicate::ord: llvmPred = LLVM::FCmpPredicate::ord; break;
    case CmpFPredicate::ueq: llvmPred = LLVM::FCmpPredicate::ueq; break;
    case CmpFPredicate::ugt: llvmPred = LLVM::FCmpPredicate::ugt; break;
    case CmpFPredicate::uge: llvmPred = LLVM::FCmpPredicate::uge; break;
    case CmpFPredicate::ult: llvmPred = LLVM::FCmpPredicate::ult; break;
    case CmpFPredicate::ule: llvmPred = LLVM::FCmpPredicate::ule; break;
    case CmpFPredicate::une: llvmPred = LLVM::FCmpPredicate::une; break;
    case CmpFPredicate::uno: llvmPred = LLVM::FCmpPredicate::uno; break;
    }
    rewriter.replaceOpWithNewOp<LLVM::FCmpOp>(
        op, llvmPred, adaptor.getLhs(), adaptor.getRhs());
    return success();
  }
};

//===----------------------------------------------------------------------===//
// Select pattern
//===----------------------------------------------------------------------===//

struct SelectOpLowering : public OpConversionPattern<SelectOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      SelectOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    rewriter.replaceOpWithNewOp<LLVM::SelectOp>(
        op, adaptor.getCondition(),
        adaptor.getTrueValue(), adaptor.getFalseValue());
    return success();
  }
};

//===----------------------------------------------------------------------===//
// Cast patterns — each maps 1:1 to its LLVM equivalent
//===----------------------------------------------------------------------===//

struct ExtSIOpLowering : public OpConversionPattern<ExtSIOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      ExtSIOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto resultType = getTypeConverter()->convertType(op.getType());
    rewriter.replaceOpWithNewOp<LLVM::SExtOp>(
        op, resultType, adaptor.getInput());
    return success();
  }
};

struct ExtUIOpLowering : public OpConversionPattern<ExtUIOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      ExtUIOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto resultType = getTypeConverter()->convertType(op.getType());
    rewriter.replaceOpWithNewOp<LLVM::ZExtOp>(
        op, resultType, adaptor.getInput());
    return success();
  }
};

struct TruncIOpLowering : public OpConversionPattern<TruncIOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      TruncIOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto resultType = getTypeConverter()->convertType(op.getType());
    rewriter.replaceOpWithNewOp<LLVM::TruncOp>(
        op, resultType, adaptor.getInput());
    return success();
  }
};

struct SIToFPOpLowering : public OpConversionPattern<SIToFPOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      SIToFPOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto resultType = getTypeConverter()->convertType(op.getType());
    rewriter.replaceOpWithNewOp<LLVM::SIToFPOp>(
        op, resultType, adaptor.getInput());
    return success();
  }
};

struct FPToSIOpLowering : public OpConversionPattern<FPToSIOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      FPToSIOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto resultType = getTypeConverter()->convertType(op.getType());
    rewriter.replaceOpWithNewOp<LLVM::FPToSIOp>(
        op, resultType, adaptor.getInput());
    return success();
  }
};

struct ExtFOpLowering : public OpConversionPattern<ExtFOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      ExtFOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto resultType = getTypeConverter()->convertType(op.getType());
    rewriter.replaceOpWithNewOp<LLVM::FPExtOp>(
        op, resultType, adaptor.getInput());
    return success();
  }
};

struct TruncFOpLowering : public OpConversionPattern<TruncFOp> {
  using OpConversionPattern::OpConversionPattern;
  LogicalResult matchAndRewrite(
      TruncFOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto resultType = getTypeConverter()->convertType(op.getType());
    rewriter.replaceOpWithNewOp<LLVM::FPTruncOp>(
        op, resultType, adaptor.getInput());
    return success();
  }
};

//===----------------------------------------------------------------------===//
// Registration
//===----------------------------------------------------------------------===//

void cot::populateArithmeticPatterns(RewritePatternSet &patterns,
                                     TypeConverter &typeConverter) {
  patterns.add<
      ConstantOpLowering,
      AddOpLowering, SubOpLowering, MulOpLowering,
      DivSIOpLowering, DivUIOpLowering, DivFOpLowering,
      RemSIOpLowering, RemUIOpLowering, RemFOpLowering,
      NegOpLowering, NegFOpLowering,
      CmpOpLowering, CmpFOpLowering,
      SelectOpLowering,
      ExtSIOpLowering, ExtUIOpLowering, TruncIOpLowering,
      SIToFPOpLowering, FPToSIOpLowering,
      ExtFOpLowering, TruncFOpLowering,
      BitAndOpLowering, BitOrOpLowering, BitXorOpLowering,
      BitNotOpLowering, ShlOpLowering, ShrOpLowering, ShrSOpLowering
  >(typeConverter, patterns.getContext());
}
