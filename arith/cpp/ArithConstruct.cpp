//===- CoreConstruct.cpp - cot-core construct registration ----*- C++ -*-===//
//
// Registers cot-core's ops, transforms, and lowering patterns with the
// COT framework.
//
//===----------------------------------------------------------------------===//
#include "cot/Construct/Construct.h"
#include "cot/Pipeline/CIRSema.h"
#include "cot/Pipeline/SemaStep.h"
#include "cot/CIR/CIRDialect.h"
#include "arith/Ops.h"
#include "arith/Transform.h"
#include "arith/Lowering.h"

#include "mlir/Dialect/Func/IR/FuncOps.h"
#include "mlir/IR/Builders.h"
#include "mlir/IR/BuiltinOps.h"
#include "mlir/IR/SymbolTable.h"
#include "mlir/Pass/Pass.h"

using namespace mlir;

namespace {

//===----------------------------------------------------------------------===//
// TypeCheckingStep — CIRSema step for type checking + cast insertion
//===----------------------------------------------------------------------===//

class TypeCheckingStep : public cot::SemaStep {
public:
  llvm::StringRef getName() const override { return "type-checking"; }
  Position getPosition() const override { return Types; }

  bool visitOp(Operation *op, cot::CIRSema &sema) override {
    auto callOp = dyn_cast<func::CallOp>(op);
    if (!callOp) return false;

    auto *calleeSym = sema.symbolTable->lookup(callOp.getCallee());
    if (!calleeSym) return false;
    auto calleeFunc = dyn_cast<func::FuncOp>(calleeSym);
    if (!calleeFunc) return false;

    auto calleeType = calleeFunc.getFunctionType();
    auto args = callOp.getOperands();
    bool modified = false;

    for (unsigned i = 0;
         i < args.size() && i < calleeType.getNumInputs(); ++i) {
      Type argType = args[i].getType();
      Type paramType = calleeType.getInput(i);
      if (argType == paramType) continue;

      OpBuilder builder(callOp);
      Value castResult = insertCast(builder, callOp.getLoc(),
                                     args[i], argType, paramType);
      if (castResult) {
        callOp.setOperand(i, castResult);
        modified = true;
      }
    }
    return modified;
  }

private:
  Value insertCast(OpBuilder &builder, Location loc,
                   Value input, Type srcType, Type dstType) {
    if (auto srcInt = dyn_cast<IntegerType>(srcType)) {
      if (auto dstInt = dyn_cast<IntegerType>(dstType)) {
        if (srcInt.getWidth() < dstInt.getWidth())
          return builder.create<cir::ExtSIOp>(loc, dstType, input);
        if (srcInt.getWidth() > dstInt.getWidth())
          return builder.create<cir::TruncIOp>(loc, dstType, input);
      }
      if (isa<FloatType>(dstType))
        return builder.create<cir::SIToFPOp>(loc, dstType, input);
    }
    if (isa<FloatType>(srcType)) {
      if (isa<IntegerType>(dstType))
        return builder.create<cir::FPToSIOp>(loc, dstType, input);
      if (auto srcFloat = dyn_cast<FloatType>(srcType)) {
        if (auto dstFloat = dyn_cast<FloatType>(dstType)) {
          if (srcFloat.getWidth() < dstFloat.getWidth())
            return builder.create<cir::ExtFOp>(loc, dstType, input);
          if (srcFloat.getWidth() > dstFloat.getWidth())
            return builder.create<cir::TruncFOp>(loc, dstType, input);
        }
      }
    }
    return nullptr;
  }
};

//===----------------------------------------------------------------------===//
// CoreConstruct
//===----------------------------------------------------------------------===//

class CoreConstruct : public cot::Construct {
public:
  llvm::StringRef getName() const override { return "arith"; }

  void registerOpsAndTypes(MLIRContext &ctx) override {
    // Get the CIR dialect and register cot-core's ops.
    auto *dialect = ctx.getOrLoadDialect<cir::CIRDialect>();
    dialect->registerConstructOps<
        cir::ConstantOp,
        cir::AddOp, cir::SubOp, cir::MulOp,
        cir::DivSIOp, cir::DivUIOp, cir::DivFOp,
        cir::RemSIOp, cir::RemUIOp, cir::RemFOp,
        cir::NegOp, cir::NegFOp,
        cir::BitAndOp, cir::BitOrOp, cir::BitXorOp, cir::BitNotOp,
        cir::ShlOp, cir::ShrOp, cir::ShrSOp,
        cir::CmpOp, cir::CmpFOp,
        cir::SelectOp,
        cir::ExtSIOp, cir::ExtUIOp, cir::TruncIOp,
        cir::SIToFPOp, cir::FPToSIOp,
        cir::ExtFOp, cir::TruncFOp
    >();
  }

  void registerSemaSteps(cot::CIRSema &sema) override {
    sema.addStep(std::make_unique<TypeCheckingStep>());
  }

  // addTransformers() intentionally empty — type checking now in CIRSema.
  void addTransformers(PassManager &preSemaPM,
                       PassManager &postSemaPM) override {}

  void populateLoweringPatterns(
      RewritePatternSet &patterns,
      TypeConverter &typeConverter) override {
    cot::populateArithmeticPatterns(patterns, typeConverter);
  }
};

} // namespace

COT_REGISTER_CONSTRUCT(CoreConstruct)
