//===- Lowering.cpp - cot-test CIR -> LLVM patterns -----------*- C++ -*-===//
//
// Lowering for cir.assert. cir.test_case is erased by TestRunnerGenerator
// before lowering runs — it should never reach this point.
// Reference: Zig test runner, MLIR ConversionPatterns.
//
//===----------------------------------------------------------------------===//
#include "test/Lowering.h"
#include "test/Ops.h"

#include "mlir/Conversion/LLVMCommon/Pattern.h"
#include "mlir/Dialect/LLVMIR/LLVMDialect.h"
#include "mlir/Transforms/DialectConversion.h"

using namespace mlir;

namespace {

//===----------------------------------------------------------------------===//
// AssertOp -> condbr true:continue, false:(call @cir_test_fail + trap)
//===----------------------------------------------------------------------===//

struct AssertOpLowering : public OpConversionPattern<cir::AssertOp> {
  using OpConversionPattern::OpConversionPattern;

  LogicalResult matchAndRewrite(
      cir::AssertOp op, OpAdaptor adaptor,
      ConversionPatternRewriter &rewriter) const override {
    auto loc = op.getLoc();
    auto moduleOp = op->getParentOfType<ModuleOp>();
    auto *parentRegion = op->getBlock()->getParent();

    auto msg = op.getMessage();
    auto msgStr = msg.str();
    auto i8Type = rewriter.getI8Type();
    auto i64Type = rewriter.getI64Type();
    auto i32Type = rewriter.getI32Type();
    auto ptrType = LLVM::LLVMPointerType::get(getContext());
    auto voidType = LLVM::LLVMVoidType::get(getContext());

    // Create global string constant for the message
    auto arrayType = LLVM::LLVMArrayType::get(i8Type, msgStr.size());
    static int assertCounter = 0;
    std::string globalName =
        std::string("__assert_msg_") + std::to_string(assertCounter++);
    {
      OpBuilder::InsertionGuard guard(rewriter);
      rewriter.setInsertionPointToStart(moduleOp.getBody());
      if (!moduleOp.lookupSymbol<LLVM::GlobalOp>(globalName)) {
        rewriter.create<LLVM::GlobalOp>(
            loc, arrayType, /*isConstant=*/true, LLVM::Linkage::Internal,
            globalName, rewriter.getStringAttr(msgStr));
      }
    }

    // Declare @cir_test_fail if not already declared
    if (!moduleOp.lookupSymbol<LLVM::LLVMFuncOp>("cir_test_fail")) {
      OpBuilder::InsertionGuard guard(rewriter);
      rewriter.setInsertionPointToStart(moduleOp.getBody());
      auto fnType = LLVM::LLVMFunctionType::get(voidType, {ptrType, i64Type});
      rewriter.create<LLVM::LLVMFuncOp>(loc, "cir_test_fail", fnType);
    }

    // Split: everything after the assert goes to the continue block
    auto *currentBlock = rewriter.getInsertionBlock();
    auto *continueBlock = rewriter.splitBlock(
        currentBlock, std::next(op->getIterator()));

    // Create fail block
    auto *failBlock = rewriter.createBlock(
        parentRegion, parentRegion->end());

    // In fail block: call @cir_test_fail, then trap + unreachable
    rewriter.setInsertionPointToStart(failBlock);
    auto msgPtr = rewriter.create<LLVM::AddressOfOp>(loc, ptrType, globalName);
    auto msgLen = rewriter.create<LLVM::ConstantOp>(
        loc, i64Type, rewriter.getI64IntegerAttr(msgStr.size()));
    auto failFn = moduleOp.lookupSymbol<LLVM::LLVMFuncOp>("cir_test_fail");
    rewriter.create<LLVM::CallOp>(loc, failFn, ValueRange{msgPtr, msgLen});
    rewriter.create<LLVM::Trap>(loc);
    rewriter.create<LLVM::UnreachableOp>(loc);

    // Replace assert with condbr: true→continue, false→fail
    rewriter.setInsertionPointToEnd(currentBlock);
    rewriter.create<LLVM::CondBrOp>(
        loc, adaptor.getCondition(), continueBlock, failBlock);

    rewriter.eraseOp(op);
    return success();
  }
};

} // anonymous namespace

void cot::populateTestPatterns(RewritePatternSet &patterns,
                                TypeConverter &typeConverter) {
  patterns.add<AssertOpLowering>(typeConverter, patterns.getContext());
}
