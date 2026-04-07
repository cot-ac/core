//===- Lowering.cpp - traits CIR -> LLVM patterns -------------*- C++ -*-===//
//
// witness_table → LLVM global constant struct (array of fn ptrs).
// trait_call → must be resolved by GenericSpecializer (error if reaches lowering).
// witness_method → GEP into PWT + load function pointer.
// init/open/deinit_existential → struct stores/loads on the 3-word container.
// Reference: Swift lib/IRGen/GenProto.cpp.
//
//===----------------------------------------------------------------------===//
#include "traits/Lowering.h"
#include "traits/Ops.h"
#include "traits/Types.h"

#include "mlir/Conversion/LLVMCommon/Pattern.h"
#include "mlir/Dialect/LLVMIR/LLVMDialect.h"
#include "mlir/Transforms/DialectConversion.h"

using namespace mlir;

namespace {

//===----------------------------------------------------------------------===//
// TraitCallOp — must be resolved before lowering
//===----------------------------------------------------------------------===//

struct TraitCallOpLowering : public OpConversionPattern<cir::TraitCallOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::TraitCallOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    return op.emitOpError(
        "must be resolved by GenericSpecializer before lowering");
  }
};

//===----------------------------------------------------------------------===//
// WitnessTableOp → LLVM global constant (array of ptr)
//===----------------------------------------------------------------------===//

struct WitnessTableOpLowering
    : public OpConversionPattern<cir::WitnessTableOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::WitnessTableOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    // Witness tables are declarations consumed by the specializer
    // and WitnessTableGenerator. By lowering time, they can be erased.
    rewriter.eraseOp(op);
    return success();
  }
};

//===----------------------------------------------------------------------===//
// DeinitExistentialOp → no-op (metadata cleanup handled by VWT.destroy)
//===----------------------------------------------------------------------===//

struct DeinitExistentialOpLowering
    : public OpConversionPattern<cir::DeinitExistentialOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::DeinitExistentialOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    rewriter.eraseOp(op);
    return success();
  }
};

} // anonymous namespace

void cot::populateTraitsPatterns(RewritePatternSet &patterns,
                                 TypeConverter &typeConverter) {
  patterns.add<
      TraitCallOpLowering,
      WitnessTableOpLowering,
      DeinitExistentialOpLowering>(typeConverter, patterns.getContext());
}
