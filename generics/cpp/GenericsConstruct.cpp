//===- GenericsConstruct.cpp - generics construct registration --*- C++ -*-===//
//
// Registers type_param type and generic_apply op with the COT framework.
//
//===----------------------------------------------------------------------===//
#include "cot/Construct/Construct.h"
#include "cot/Pipeline/CIRSema.h"
#include "cot/Pipeline/SemaStep.h"
#include "cot/CIR/CIRDialect.h"
#include "generics/Types.h"
#include "generics/Ops.h"
#include "generics/Lowering.h"
#include "generics/GenericSpecializer.h"

#include "mlir/Dialect/Func/IR/FuncOps.h"
#include "mlir/IR/Builders.h"
#include "mlir/IR/BuiltinOps.h"
#include "mlir/IR/IRMapping.h"
#include "mlir/IR/SymbolTable.h"
#include "mlir/Pass/Pass.h"
#include "mlir/IR/DialectImplementation.h"
#include "mlir/Dialect/LLVMIR/LLVMDialect.h"
#include "mlir/Pass/PassManager.h"

using namespace mlir;

namespace {

//===----------------------------------------------------------------------===//
// GenericSpecializerStep — CIRSema step for generic instantiation
//===----------------------------------------------------------------------===//

class GenericSpecializerStep : public cot::SemaStep {
  llvm::StringMap<func::FuncOp> specializations_;

public:
  llvm::StringRef getName() const override { return "generic-specializer"; }
  Position getPosition() const override { return Generics; }

  bool visitOp(Operation *op, cot::CIRSema &sema) override {
    auto applyOp = dyn_cast<cir::GenericApplyOp>(op);
    if (!applyOp) return false;

    // Extract substitution info
    auto callee = applyOp.getCallee();
    auto keys = applyOp.getSubKeys();
    auto types = applyOp.getSubTypes();
    SmallVector<StringRef> subKeys;
    SmallVector<Type> subTypes;
    for (unsigned i = 0; i < keys.size(); i++) {
      subKeys.push_back(mlir::cast<StringAttr>(keys[i]).getValue());
      subTypes.push_back(mlir::cast<TypeAttr>(types[i]).getValue());
    }

    // Build mangled name
    std::string mangledName = callee.str();
    for (auto ty : subTypes) {
      mangledName += "__";
      llvm::raw_string_ostream os(mangledName);
      ty.print(os);
    }

    // Look up or create specialization
    func::FuncOp specialized;
    auto it = specializations_.find(mangledName);
    if (it != specializations_.end()) {
      specialized = it->second;
    } else {
      auto calleeFunc = sema.symbolTable->lookup<func::FuncOp>(callee);
      if (!calleeFunc) {
        applyOp.emitError("callee '") << callee << "' not found";
        return true;
      }
      specialized = specializeFunction(
          sema.module, calleeFunc, subKeys, subTypes, mangledName, sema.ctx);
      specializations_[mangledName] = specialized;
    }

    // Rewrite: generic_apply → func.call
    OpBuilder builder(applyOp);
    SmallVector<Value> args(applyOp.getArgs());
    SmallVector<Type> resultTypes;
    for (auto result : applyOp.getResults())
      resultTypes.push_back(result.getType());
    auto callOp = builder.create<func::CallOp>(
        applyOp.getLoc(), specialized.getSymName(), resultTypes, args);
    for (unsigned i = 0; i < applyOp.getNumResults(); i++)
      applyOp.getResult(i).replaceAllUsesWith(callOp.getResult(i));
    applyOp.erase();
    return true;
  }

private:
  Type substituteType(Type ty, const llvm::StringMap<Type> &subs) {
    if (auto tp = dyn_cast<cir::TypeParamType>(ty)) {
      auto it = subs.find(tp.getName());
      if (it != subs.end()) return it->second;
    }
    return ty;
  }

  func::FuncOp specializeFunction(
      ModuleOp module, func::FuncOp genericFunc,
      ArrayRef<StringRef> subKeys, ArrayRef<Type> subTypes,
      StringRef mangledName, MLIRContext *ctx) {
    llvm::StringMap<Type> subs;
    for (unsigned i = 0; i < subKeys.size(); i++)
      subs[subKeys[i]] = subTypes[i];

    auto genericFuncType = genericFunc.getFunctionType();
    SmallVector<Type> newInputs, newResults;
    for (auto t : genericFuncType.getInputs())
      newInputs.push_back(substituteType(t, subs));
    for (auto t : genericFuncType.getResults())
      newResults.push_back(substituteType(t, subs));

    OpBuilder builder(ctx);
    builder.setInsertionPoint(genericFunc);
    auto specialized = builder.create<func::FuncOp>(
        genericFunc.getLoc(), mangledName,
        FunctionType::get(ctx, newInputs, newResults));

    IRMapping mapping;
    for (auto &block : genericFunc.getBody()) {
      auto *newBlock = new Block();
      specialized.getBody().push_back(newBlock);
      for (auto arg : block.getArguments()) {
        auto newArg = newBlock->addArgument(
            substituteType(arg.getType(), subs), arg.getLoc());
        mapping.map(arg, newArg);
      }
      builder.setInsertionPointToEnd(newBlock);
      for (auto &op : block) {
        auto *newOp = builder.clone(op, mapping);
        for (unsigned i = 0; i < newOp->getNumResults(); i++) {
          auto newType = substituteType(newOp->getResult(i).getType(), subs);
          if (newOp->getResult(i).getType() != newType)
            newOp->getResult(i).setType(newType);
        }
      }
    }
    return specialized;
  }
};

//===----------------------------------------------------------------------===//
// GenericsConstruct
//===----------------------------------------------------------------------===//

class GenericsConstruct : public cot::Construct {
public:
  llvm::StringRef getName() const override { return "generics"; }

  void registerOpsAndTypes(MLIRContext &ctx) override {
    auto *dialect = ctx.getOrLoadDialect<cir::CIRDialect>();
    cir::registerGenericsTypes(dialect);
    dialect->registerConstructOps<
        cir::GenericApplyOp
    >();
  }

  void registerSemaSteps(cot::CIRSema &sema) override {
    sema.addStep(std::make_unique<GenericSpecializerStep>());
  }

  // Keep old pass for backward compatibility (--cir-specialize).
  // Will be removed in Task 49.
  void addTransformers(PassManager &preSemaPM,
                       PassManager &postSemaPM) override {
  }

  void populateLoweringPatterns(
      RewritePatternSet &patterns,
      TypeConverter &typeConverter) override {
    cot::populateGenericsPatterns(patterns, typeConverter);
  }

  void addTypeConversions(TypeConverter &typeConverter) override {
    // !cir.type_param<"T"> → !llvm.ptr (fallback — should be resolved first)
    typeConverter.addConversion(
        [&](cir::TypeParamType type) -> Type {
          return LLVM::LLVMPointerType::get(type.getContext());
        });
  }

  OptionalParseResult parseType(
      llvm::StringRef keyword, DialectAsmParser &parser,
      Type &result) const override {
    if (keyword == "type_param") {
      auto parsed = cir::TypeParamType::parse(parser);
      if (!parsed)
        return failure();
      result = parsed;
      return success();
    }
    return {};
  }

  LogicalResult printType(
      Type type, DialectAsmPrinter &printer) const override {
    if (auto tp = mlir::dyn_cast<cir::TypeParamType>(type)) {
      printer << "type_param<\"" << tp.getName() << "\">";
      return success();
    }
    return failure();
  }
};

} // namespace

COT_REGISTER_CONSTRUCT(GenericsConstruct)
