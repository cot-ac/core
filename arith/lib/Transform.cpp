//===- Transform.cpp - SemanticAnalysis pass ------------------*- C++ -*-===//
//
// Type checking and cast insertion for CIR.
// Reference: Zig Sema (~/references/zig/src/Sema.zig) — resolves
//            untyped ZIR to typed AIR via sequential dispatch.
//
//===----------------------------------------------------------------------===//
#include "arith/Transform.h"
#include "arith/Ops.h"

#include "mlir/Dialect/Func/IR/FuncOps.h"
#include "mlir/IR/BuiltinOps.h"
#include "mlir/Pass/Pass.h"

using namespace mlir;
using namespace cir;

namespace {

/// Semantic analysis pass. Runs per-function.
///
/// What it does:
/// 1. Looks up callee signatures via SymbolTable (E8)
/// 2. Validates call argument types match callee parameter types
/// 3. Inserts cast ops (extsi, trunci, sitofp, etc.) at type boundaries
struct SemanticAnalysisPass
    : public PassWrapper<SemanticAnalysisPass, OperationPass<func::FuncOp>> {
  MLIR_DEFINE_EXPLICIT_INTERNAL_INLINE_TYPE_ID(SemanticAnalysisPass)

  StringRef getArgument() const final { return "cir-semantic-analysis"; }
  StringRef getDescription() const final {
    return "Type checking and cast insertion for CIR";
  }

  void runOnOperation() override {
    auto funcOp = getOperation();

    // Build symbol table from module (E8: use SymbolTable)
    auto moduleOp = funcOp->getParentOfType<ModuleOp>();
    SymbolTable symbolTable(moduleOp);

    // Walk all operations in the function
    funcOp.walk([&](Operation *op) {
      if (auto callOp = dyn_cast<func::CallOp>(op)) {
        auto *calleeSym = symbolTable.lookup(callOp.getCallee());
        if (!calleeSym)
          return;  // external function — skip

        auto calleeFunc = dyn_cast<func::FuncOp>(calleeSym);
        if (!calleeFunc)
          return;

        auto calleeType = calleeFunc.getFunctionType();
        auto args = callOp.getOperands();

        for (unsigned i = 0;
             i < args.size() && i < calleeType.getNumInputs(); ++i) {
          Type argType = args[i].getType();
          Type paramType = calleeType.getInput(i);

          if (argType == paramType)
            continue;

          OpBuilder builder(callOp);
          Value castResult = insertCast(builder, callOp.getLoc(),
                                        args[i], argType, paramType);
          if (castResult)
            callOp.setOperand(i, castResult);
        }
      }
    });
  }

private:
  /// Insert the correct cast op for a type pair.
  Value insertCast(OpBuilder &builder, Location loc,
                   Value input, Type srcType, Type dstType) {
    if (auto srcInt = dyn_cast<IntegerType>(srcType)) {
      if (auto dstInt = dyn_cast<IntegerType>(dstType)) {
        if (srcInt.getWidth() < dstInt.getWidth())
          return builder.create<ExtSIOp>(loc, dstType, input);
        if (srcInt.getWidth() > dstInt.getWidth())
          return builder.create<TruncIOp>(loc, dstType, input);
      }
      if (isa<FloatType>(dstType))
        return builder.create<SIToFPOp>(loc, dstType, input);
    }

    if (isa<FloatType>(srcType)) {
      if (isa<IntegerType>(dstType))
        return builder.create<FPToSIOp>(loc, dstType, input);
      if (auto srcFloat = dyn_cast<FloatType>(srcType)) {
        if (auto dstFloat = dyn_cast<FloatType>(dstType)) {
          if (srcFloat.getWidth() < dstFloat.getWidth())
            return builder.create<ExtFOp>(loc, dstType, input);
          if (srcFloat.getWidth() > dstFloat.getWidth())
            return builder.create<TruncFOp>(loc, dstType, input);
        }
      }
    }

    return nullptr;
  }
};

} // namespace

std::unique_ptr<Pass> cot::createSemanticAnalysisPass() {
  return std::make_unique<SemanticAnalysisPass>();
}
