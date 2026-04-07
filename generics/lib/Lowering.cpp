//===- Lowering.cpp - generics CIR -> LLVM patterns -----------*- C++ -*-===//
//
// GenericApplyOp should be resolved by GenericSpecializer before lowering.
// If it reaches lowering unresolved, emit a diagnostic error.
// TypeParamType falls back to !llvm.ptr as a safety net.
// Reference: Swift SIL — archetypes must be resolved before IRGen.
//
//===----------------------------------------------------------------------===//
#include "generics/Lowering.h"
#include "generics/Ops.h"
#include "generics/Types.h"

#include "mlir/Conversion/LLVMCommon/Pattern.h"
#include "mlir/Dialect/LLVMIR/LLVMDialect.h"
#include "mlir/Transforms/DialectConversion.h"

using namespace mlir;

namespace {

//===----------------------------------------------------------------------===//
// GenericApplyOp — must be resolved before lowering
//===----------------------------------------------------------------------===//

struct GenericApplyOpLowering
    : public OpConversionPattern<cir::GenericApplyOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::GenericApplyOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    return op.emitOpError(
        "must be resolved by GenericSpecializer before lowering");
  }
};

} // anonymous namespace

void cot::populateGenericsPatterns(RewritePatternSet &patterns,
                                   TypeConverter &typeConverter) {
  patterns.add<GenericApplyOpLowering>(typeConverter, patterns.getContext());
}
