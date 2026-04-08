//===- Lowering.cpp - traits CIR -> LLVM patterns -------------*- C++ -*-===//
//
// Existential container lowered layout: struct<([24 x i8], ptr, ptr)>
//   field 0: [24 x i8]  — inline value buffer (3 words on 64-bit)
//   field 1: ptr         — Value Witness Table pointer
//   field 2: ptr         — Protocol Witness Table pointer
//
// Reference: Swift lib/IRGen/GenProto.cpp, GenExistential.cpp
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
// WitnessTableOp → LLVM global constant array of function pointers
//===----------------------------------------------------------------------===//
//
// cir.witness_table @Summable_Point ["sum" = @Point_sum, "zero" = @Point_zero]
// →
// llvm.mlir.global constant @Summable_Point() : !llvm.array<2 x ptr> {
//   %0 = llvm.mlir.addressof @Point_sum : !llvm.ptr
//   %1 = llvm.mlir.undef : !llvm.array<2 x ptr>
//   %2 = llvm.insertvalue %0, %1[0] : !llvm.array<2 x ptr>
//   %3 = llvm.mlir.addressof @Point_zero : !llvm.ptr
//   %4 = llvm.insertvalue %3, %2[1] : !llvm.array<2 x ptr>
//   llvm.return %4 : !llvm.array<2 x ptr>
// }

struct WitnessTableOpLowering
    : public OpConversionPattern<cir::WitnessTableOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::WitnessTableOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto loc = op.getLoc();
    auto ctx = rewriter.getContext();
    auto ptrTy = LLVM::LLVMPointerType::get(ctx);

    auto impls = op.getMethodImpls();
    auto numMethods = impls.size();

    // Create global constant: array of function pointers
    auto arrayTy = LLVM::LLVMArrayType::get(ptrTy, numMethods);
    auto global = rewriter.create<LLVM::GlobalOp>(
        loc, arrayTy, /*isConstant=*/true, LLVM::Linkage::Internal,
        op.getSymName(), Attribute{});

    // Build initializer region
    auto &initRegion = global.getInitializerRegion();
    auto *initBlock = rewriter.createBlock(&initRegion);
    rewriter.setInsertionPointToStart(initBlock);

    // Start with undef array
    Value array = rewriter.create<LLVM::UndefOp>(loc, arrayTy);

    // Insert each method's address
    for (unsigned i = 0; i < numMethods; i++) {
      auto implRef = mlir::cast<FlatSymbolRefAttr>(impls[i]);
      auto addr = rewriter.create<LLVM::AddressOfOp>(loc, ptrTy,
                                                       implRef.getValue());
      array = rewriter.create<LLVM::InsertValueOp>(loc, array, addr, i);
    }

    rewriter.create<LLVM::ReturnOp>(loc, array);

    // Erase the CIR witness_table op
    rewriter.eraseOp(op);
    return success();
  }
};

//===----------------------------------------------------------------------===//
// InitExistentialOp → store value + VWT + PWT into container struct
//===----------------------------------------------------------------------===//
//
// Container is struct<([24 x i8], ptr, ptr)>:
//   GEP field 0 → store value (bitcast to byte buffer)
//   GEP field 1 → store VWT pointer
//   GEP field 2 → store PWT pointer
//
// Reference: Swift GenExistential.cpp emitOpaqueExistentialContainerInit

struct InitExistentialOpLowering
    : public OpConversionPattern<cir::InitExistentialOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::InitExistentialOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto loc = op.getLoc();
    auto ctx = rewriter.getContext();
    auto ptrTy = LLVM::LLVMPointerType::get(ctx);
    auto i32Ty = rewriter.getI32Type();

    auto container = adaptor.getContainer();

    // GEP field 0 (buffer) → store value
    auto zero = rewriter.create<LLVM::ConstantOp>(
        loc, i32Ty, rewriter.getI32IntegerAttr(0));
    auto bufIdx = rewriter.create<LLVM::ConstantOp>(
        loc, i32Ty, rewriter.getI32IntegerAttr(0));
    auto bufferStructTy = getTypeConverter()->convertType(
        op.getContainer().getType());
    auto bufPtr = rewriter.create<LLVM::GEPOp>(
        loc, ptrTy, bufferStructTy, container,
        ValueRange{zero, bufIdx});
    rewriter.create<LLVM::StoreOp>(loc, adaptor.getValue(), bufPtr);

    // GEP field 1 (VWT) → store VWT pointer
    auto vwtIdx = rewriter.create<LLVM::ConstantOp>(
        loc, i32Ty, rewriter.getI32IntegerAttr(1));
    auto vwtPtr = rewriter.create<LLVM::GEPOp>(
        loc, ptrTy, bufferStructTy, container,
        ValueRange{zero, vwtIdx});
    rewriter.create<LLVM::StoreOp>(loc, adaptor.getVwt(), vwtPtr);

    // GEP field 2 (PWT) → store PWT pointer
    auto pwtIdx = rewriter.create<LLVM::ConstantOp>(
        loc, i32Ty, rewriter.getI32IntegerAttr(2));
    auto pwtPtr = rewriter.create<LLVM::GEPOp>(
        loc, ptrTy, bufferStructTy, container,
        ValueRange{zero, pwtIdx});
    rewriter.create<LLVM::StoreOp>(loc, adaptor.getPwt(), pwtPtr);

    rewriter.eraseOp(op);
    return success();
  }
};

//===----------------------------------------------------------------------===//
// OpenExistentialOp → GEP + load buffer ptr, VWT ptr, PWT ptr
//===----------------------------------------------------------------------===//
//
// Extract three fields from the container struct.
// Returns: (buffer_ptr, vwt_ptr, pwt_ptr)

struct OpenExistentialOpLowering
    : public OpConversionPattern<cir::OpenExistentialOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::OpenExistentialOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto loc = op.getLoc();
    auto ctx = rewriter.getContext();
    auto ptrTy = LLVM::LLVMPointerType::get(ctx);
    auto i32Ty = rewriter.getI32Type();

    auto container = adaptor.getContainer();
    auto containerTy = getTypeConverter()->convertType(
        op.getContainer().getType());

    auto zero = rewriter.create<LLVM::ConstantOp>(
        loc, i32Ty, rewriter.getI32IntegerAttr(0));

    // GEP field 0 → buffer address (pointer to the [24 x i8] array)
    auto bufIdx = rewriter.create<LLVM::ConstantOp>(
        loc, i32Ty, rewriter.getI32IntegerAttr(0));
    auto bufPtr = rewriter.create<LLVM::GEPOp>(
        loc, ptrTy, containerTy, container,
        ValueRange{zero, bufIdx});

    // GEP field 1 → load VWT pointer
    auto vwtIdx = rewriter.create<LLVM::ConstantOp>(
        loc, i32Ty, rewriter.getI32IntegerAttr(1));
    auto vwtSlot = rewriter.create<LLVM::GEPOp>(
        loc, ptrTy, containerTy, container,
        ValueRange{zero, vwtIdx});
    auto vwt = rewriter.create<LLVM::LoadOp>(loc, ptrTy, vwtSlot);

    // GEP field 2 → load PWT pointer
    auto pwtIdx = rewriter.create<LLVM::ConstantOp>(
        loc, i32Ty, rewriter.getI32IntegerAttr(2));
    auto pwtSlot = rewriter.create<LLVM::GEPOp>(
        loc, ptrTy, containerTy, container,
        ValueRange{zero, pwtIdx});
    auto pwt = rewriter.create<LLVM::LoadOp>(loc, ptrTy, pwtSlot);

    rewriter.replaceOp(op, {bufPtr, vwt, pwt});
    return success();
  }
};

//===----------------------------------------------------------------------===//
// WitnessMethodOp → GEP into PWT at method_index + load fn ptr
//===----------------------------------------------------------------------===//
//
// PWT is an array of function pointers. method_index gives the slot.
// Reference: Swift IRGen — witness table is indexed by requirement offset.

struct WitnessMethodOpLowering
    : public OpConversionPattern<cir::WitnessMethodOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::WitnessMethodOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto loc = op.getLoc();
    auto ctx = rewriter.getContext();
    auto ptrTy = LLVM::LLVMPointerType::get(ctx);
    auto i64Ty = rewriter.getI64Type();

    auto methodIdx = op.getMethodIndex();
    if (!methodIdx) {
      return op.emitOpError("method_index must be set before lowering "
                             "(run a resolution pass first)");
    }

    // GEP into PWT (array of ptrs) at method_index
    auto idx = rewriter.create<LLVM::ConstantOp>(
        loc, i64Ty, rewriter.getI64IntegerAttr(*methodIdx));
    auto fnSlot = rewriter.create<LLVM::GEPOp>(
        loc, ptrTy, ptrTy, adaptor.getPwt(), ValueRange{idx});

    // Load the function pointer
    auto fnPtr = rewriter.create<LLVM::LoadOp>(loc, ptrTy, fnSlot);

    rewriter.replaceOp(op, fnPtr.getResult());
    return success();
  }
};

//===----------------------------------------------------------------------===//
// DeinitExistentialOp → no-op (metadata cleanup deferred to VWT.destroy)
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
      InitExistentialOpLowering,
      OpenExistentialOpLowering,
      WitnessMethodOpLowering,
      DeinitExistentialOpLowering>(typeConverter, patterns.getContext());
}
