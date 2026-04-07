//===- GenericSpecializer.cpp - Monomorphization pass ----------*- C++ -*-===//
//
// Clones generic functions for each concrete type instantiation.
// Resolves cir.generic_apply → func.call to specialized versions.
//
// Algorithm:
//   1. Walk module, collect all cir.generic_apply call sites
//   2. Group by (callee, substitution map) → deduplicate
//   3. For each unique instantiation: clone callee, substitute types
//   4. Rewrite call sites to func.call @specialized_fn
//
// Reference: Rust rustc_monomorphize/src/collector.rs (two-phase collect),
//            Swift lib/SILOptimizer/Transforms/GenericSpecializer.cpp
//
//===----------------------------------------------------------------------===//
#include "generics/GenericSpecializer.h"
#include "generics/Ops.h"
#include "generics/Types.h"

#include "mlir/Dialect/Func/IR/FuncOps.h"
#include "mlir/IR/Builders.h"
#include "mlir/IR/BuiltinOps.h"
#include "mlir/IR/IRMapping.h"
#include "mlir/IR/SymbolTable.h"
#include "mlir/Pass/Pass.h"

using namespace mlir;

namespace {

//===----------------------------------------------------------------------===//
// Substitution key: (callee name, concrete type list) for deduplication
//===----------------------------------------------------------------------===//

struct SpecializationKey {
  StringRef callee;
  SmallVector<Type> concreteTypes;

  bool operator==(const SpecializationKey &other) const {
    if (callee != other.callee) return false;
    if (concreteTypes.size() != other.concreteTypes.size()) return false;
    for (unsigned i = 0; i < concreteTypes.size(); i++)
      if (concreteTypes[i] != other.concreteTypes[i]) return false;
    return true;
  }
};

//===----------------------------------------------------------------------===//
// Collected apply site: a cir.generic_apply to specialize
//===----------------------------------------------------------------------===//

struct ApplySite {
  cir::GenericApplyOp op;
  StringRef callee;
  SmallVector<StringRef> subKeys;
  SmallVector<Type> subTypes;
};

//===----------------------------------------------------------------------===//
// The pass
//===----------------------------------------------------------------------===//

struct GenericSpecializerPass
    : public PassWrapper<GenericSpecializerPass, OperationPass<ModuleOp>> {
  MLIR_DEFINE_EXPLICIT_INTERNAL_INLINE_TYPE_ID(GenericSpecializerPass)

  StringRef getArgument() const override { return "cir-specialize"; }
  StringRef getDescription() const override {
    return "Monomorphize generic functions (resolve cir.generic_apply)";
  }

  void getDependentDialects(DialectRegistry &registry) const override {
    registry.insert<func::FuncDialect>();
  }

  void runOnOperation() override {
    ModuleOp module = getOperation();
    MLIRContext *ctx = &getContext();
    SymbolTable symbolTable(module);

    // Phase 1: Collect all generic_apply sites
    SmallVector<ApplySite> applySites;
    module.walk([&](cir::GenericApplyOp applyOp) {
      ApplySite site;
      site.op = applyOp;
      site.callee = applyOp.getCallee();

      auto keys = applyOp.getSubKeys();
      auto types = applyOp.getSubTypes();
      for (unsigned i = 0; i < keys.size(); i++) {
        site.subKeys.push_back(
            mlir::cast<StringAttr>(keys[i]).getValue());
        site.subTypes.push_back(
            mlir::cast<TypeAttr>(types[i]).getValue());
      }
      applySites.push_back(std::move(site));
    });

    if (applySites.empty())
      return;

    // Phase 2: Deduplicate — group by (callee, types)
    // Map: mangled name → specialized function (already created?)
    llvm::StringMap<func::FuncOp> specializations;

    for (auto &site : applySites) {
      // Build mangled name: callee__type1_type2
      std::string mangledName = mangleName(site.callee, site.subTypes);

      // Check if we already specialized this
      if (specializations.count(mangledName)) {
        // Phase 4: Rewrite call site
        rewriteCallSite(site, specializations[mangledName]);
        continue;
      }

      // Phase 3: Clone and specialize
      auto callee = symbolTable.lookup<func::FuncOp>(site.callee);
      if (!callee) {
        site.op.emitError("callee '") << site.callee << "' not found";
        return signalPassFailure();
      }

      auto specialized = specializeFunction(
          module, callee, site.subKeys, site.subTypes, mangledName, ctx);
      if (!specialized) {
        site.op.emitError("failed to specialize '") << site.callee << "'";
        return signalPassFailure();
      }

      specializations[mangledName] = specialized;

      // Phase 4: Rewrite call site
      rewriteCallSite(site, specialized);
    }
  }

private:
  /// Build a mangled name: identity__i32, max__f64, etc.
  std::string mangleName(StringRef callee, ArrayRef<Type> types) {
    std::string name = callee.str();
    for (auto ty : types) {
      name += "__";
      llvm::raw_string_ostream os(name);
      ty.print(os);
    }
    return name;
  }

  /// Clone a generic function, substituting type parameters with concrete types.
  func::FuncOp specializeFunction(
      ModuleOp module, func::FuncOp genericFunc,
      ArrayRef<StringRef> subKeys, ArrayRef<Type> subTypes,
      StringRef mangledName, MLIRContext *ctx) {

    // Build substitution map: param name → concrete type
    llvm::StringMap<Type> substitutionMap;
    for (unsigned i = 0; i < subKeys.size(); i++)
      substitutionMap[subKeys[i]] = subTypes[i];

    // Compute the specialized function type by substituting type_param types
    auto genericFuncType = genericFunc.getFunctionType();
    SmallVector<Type> newInputTypes;
    for (auto inputTy : genericFuncType.getInputs())
      newInputTypes.push_back(substituteType(inputTy, substitutionMap));

    SmallVector<Type> newResultTypes;
    for (auto resultTy : genericFuncType.getResults())
      newResultTypes.push_back(substituteType(resultTy, substitutionMap));

    auto specializedFuncType = FunctionType::get(ctx, newInputTypes,
                                                  newResultTypes);

    // Create the new function
    OpBuilder builder(ctx);
    builder.setInsertionPoint(genericFunc);
    auto specializedFunc = builder.create<func::FuncOp>(
        genericFunc.getLoc(), mangledName, specializedFuncType);

    // Clone the function body using IRMapping for value remapping
    IRMapping mapping;

    // Create entry block with specialized argument types
    auto &genericRegion = genericFunc.getBody();
    auto &specializedRegion = specializedFunc.getBody();

    for (auto &block : genericRegion) {
      auto *newBlock = new Block();
      specializedRegion.push_back(newBlock);

      // Map block arguments with substituted types
      for (auto arg : block.getArguments()) {
        auto newType = substituteType(arg.getType(), substitutionMap);
        auto newArg = newBlock->addArgument(newType, arg.getLoc());
        mapping.map(arg, newArg);
      }

      // Clone operations with type substitution
      builder.setInsertionPointToEnd(newBlock);
      for (auto &op : block) {
        auto *newOp = builder.clone(op, mapping);
        // Substitute types in results
        for (unsigned i = 0; i < newOp->getNumResults(); i++) {
          auto oldType = newOp->getResult(i).getType();
          auto newType = substituteType(oldType, substitutionMap);
          if (oldType != newType)
            newOp->getResult(i).setType(newType);
        }
      }
    }

    return specializedFunc;
  }

  /// Substitute !cir.type_param<"T"> with concrete types from the map.
  Type substituteType(Type ty, const llvm::StringMap<Type> &subs) {
    if (auto tp = mlir::dyn_cast<cir::TypeParamType>(ty)) {
      auto it = subs.find(tp.getName());
      if (it != subs.end())
        return it->second;
    }
    return ty;
  }

  /// Rewrite a cir.generic_apply to func.call @specialized_fn.
  void rewriteCallSite(ApplySite &site, func::FuncOp specialized) {
    auto applyOp = site.op;
    OpBuilder builder(applyOp);

    // Build func.call with the same arguments
    SmallVector<Value> args(applyOp.getArgs());
    SmallVector<Type> resultTypes;
    for (auto result : applyOp.getResults())
      resultTypes.push_back(result.getType());

    auto callOp = builder.create<func::CallOp>(
        applyOp.getLoc(), specialized.getSymName(), resultTypes, args);

    // Replace uses and erase
    for (unsigned i = 0; i < applyOp.getNumResults(); i++)
      applyOp.getResult(i).replaceAllUsesWith(callOp.getResult(i));
    applyOp.erase();
  }
};

} // anonymous namespace

std::unique_ptr<Pass> cot::createGenericSpecializerPass() {
  return std::make_unique<GenericSpecializerPass>();
}
