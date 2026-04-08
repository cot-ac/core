//===- ComptimeStep.cpp - Comptime evaluation CIRSema step ----*- C++ -*-===//
//
// Generic comptime evaluator. Layered on MLIR's existing fold() mechanism:
//
// 1. Track comptime-known values (constants) in sema.comptimeValues
// 2. For pure ops with fold(): if all inputs known, fold and replace
// 3. For side-effecting ops: query ComptimeEvaluable interface
// 4. For condbr with known condition: eliminate dead branch
//
// The step is ~50 lines of generic code. All op-specific knowledge
// lives in the ops' fold() methods and ComptimeEvaluable implementations.
//
// Reference: Zig Sema comptime, MLIR constant folding infrastructure.
//
//===----------------------------------------------------------------------===//
#include "cot/Pipeline/CIRSema.h"
#include "cot/Pipeline/SemaStep.h"
#include "cot/Construct/Construct.h"
#include "cot/CIR/CIROpInterfaces.h"

#include "mlir/Dialect/Func/IR/FuncOps.h"
#include "mlir/IR/Builders.h"
#include "mlir/IR/BuiltinOps.h"

using namespace mlir;

namespace {

class ComptimeStep : public cot::SemaStep {
public:
  llvm::StringRef getName() const override { return "comptime"; }
  Position getPosition() const override { return Comptime; }

  bool visitOp(Operation *op, cot::CIRSema &sema) override {
    // Track ConstantLike ops as comptime-known
    if (op->hasTrait<OpTrait::ConstantLike>() && op->getNumResults() == 1) {
      SmallVector<OpFoldResult> results;
      if (succeeded(op->fold(SmallVector<Attribute>{}, results)) &&
          results.size() == 1) {
        if (auto attr = llvm::dyn_cast_if_present<Attribute>(results[0]))
          sema.comptimeValues[op->getResult(0)] = attr;
      }
      return false; // don't modify — just track
    }

    // Simulated comptime memory (Reference: Zig ComptimeAlloc)
    auto opName = op->getName().getStringRef();

    // alloca: if the stored value will be comptime, track this alloca
    if (opName == "cir.alloca") {
      // Mark alloca as comptime-trackable (value TBD on first store)
      // We use a sentinel UnitAttr to mark it as "tracked but uninitialized"
      sema.comptimeAllocs[op->getResult(0)] = UnitAttr::get(sema.ctx);
      return false; // keep the op — it may be needed at runtime
    }

    // store: if addr is comptime alloca AND value is comptime-known, simulate
    if (opName == "cir.store" && op->getNumOperands() == 2) {
      auto value = op->getOperand(0);
      auto addr = op->getOperand(1);
      auto allocIt = sema.comptimeAllocs.find(addr);
      if (allocIt != sema.comptimeAllocs.end()) {
        auto valIt = sema.comptimeValues.find(value);
        if (valIt != sema.comptimeValues.end()) {
          // Store comptime value into simulated memory
          sema.comptimeAllocs[addr] = valIt->second;
          return false; // keep store — may be needed at runtime
        } else {
          // Runtime value stored → alloca is no longer comptime
          sema.comptimeAllocs.erase(allocIt);
        }
      }
      return false;
    }

    // load: if addr is comptime alloca with known value, propagate
    if (opName == "cir.load" && op->getNumOperands() == 1 && op->getNumResults() == 1) {
      auto addr = op->getOperand(0);
      auto allocIt = sema.comptimeAllocs.find(addr);
      if (allocIt != sema.comptimeAllocs.end() &&
          !isa<UnitAttr>(allocIt->second)) {
        // Propagate the stored value as comptime-known
        sema.comptimeValues[op->getResult(0)] = allocIt->second;
        return false; // keep load — but downstream ops see it as comptime
      }
      return false;
    }

    // Comptime function calls: if all args known, evaluate inline
    // Reference: Zig zirCall comptime path + memoization
    if (opName == "func.call" && op->getNumResults() == 1) {
      if (auto calleeAttr = op->getAttrOfType<FlatSymbolRefAttr>("callee")) {
        if (tryComptimeCall(op, calleeAttr.getValue(), sema))
          return true;
      }
    }

    // Terminators and no-result ops: try ComptimeEvaluable interface
    if (op->hasTrait<OpTrait::IsTerminator>() || op->getNumResults() == 0)
      return tryComptimeEvaluable(op, sema);

    // For ops with results: try fold if all operands comptime-known
    return tryFold(op, sema);
  }

private:
  /// Collect comptime-known Attributes for all operands. Returns false
  /// if any operand is not comptime-known.
  bool collectOperandAttrs(Operation *op, cot::CIRSema &sema,
                            SmallVectorImpl<Attribute> &attrs) {
    for (auto operand : op->getOperands()) {
      auto it = sema.comptimeValues.find(operand);
      if (it == sema.comptimeValues.end()) return false;
      attrs.push_back(it->second);
    }
    return true;
  }

  /// Try to constant-fold an op using MLIR's fold() infrastructure.
  bool tryFold(Operation *op, cot::CIRSema &sema) {
    SmallVector<Attribute> operandAttrs;
    if (!collectOperandAttrs(op, sema, operandAttrs))
      return false;

    SmallVector<OpFoldResult> foldResults;
    if (failed(op->fold(operandAttrs, foldResults)))
      return false;
    if (foldResults.size() != op->getNumResults())
      return false;

    // Process each fold result
    bool canErase = true;
    for (unsigned i = 0; i < op->getNumResults(); i++) {
      if (auto attr = llvm::dyn_cast_if_present<Attribute>(foldResults[i])) {
        // Fold produced a constant attribute — materialize as cir.constant
        sema.comptimeValues[op->getResult(i)] = attr;
        auto *dialect = sema.ctx->getLoadedDialect("cir");
        if (dialect) {
          OpBuilder builder(op);
          auto *constOp = dialect->materializeConstant(
              builder, attr, op->getResult(i).getType(), op->getLoc());
          if (constOp) {
            sema.comptimeValues[constOp->getResult(0)] = attr;
            op->getResult(i).replaceAllUsesWith(constOp->getResult(0));
            continue;
          }
        }
        canErase = false; // couldn't materialize — keep tracking
      } else if (auto val = llvm::dyn_cast_if_present<Value>(foldResults[i])) {
        // Fold produced an existing value — propagate and replace
        auto it = sema.comptimeValues.find(val);
        if (it != sema.comptimeValues.end())
          sema.comptimeValues[op->getResult(i)] = it->second;
        op->getResult(i).replaceAllUsesWith(val);
      } else {
        canErase = false;
      }
    }

    if (canErase) {
      op->erase();
      return true;
    }
    return false;
  }

  /// Try to evaluate a func.call at compile time.
  /// Reference: Zig zirCall comptime path (lines 7223-7494).
  bool tryComptimeCall(Operation *op, StringRef calleeName,
                        cot::CIRSema &sema) {
    // Check all args are comptime-known
    SmallVector<Attribute> argAttrs;
    if (!collectOperandAttrs(op, sema, argAttrs))
      return false;

    // Check memoization cache
    std::string memoKey = calleeName.str();
    for (auto &attr : argAttrs) {
      memoKey += ":";
      llvm::raw_string_ostream os(memoKey);
      attr.print(os);
    }
    auto memoIt = sema.memoizedCalls.find(memoKey);
    if (memoIt != sema.memoizedCalls.end()) {
      // Cached result — materialize and replace
      sema.comptimeValues[op->getResult(0)] = memoIt->second;
      auto *dialect = sema.ctx->getLoadedDialect("cir");
      if (dialect) {
        OpBuilder builder(op);
        auto *constOp = dialect->materializeConstant(
            builder, memoIt->second, op->getResult(0).getType(), op->getLoc());
        if (constOp) {
          sema.comptimeValues[constOp->getResult(0)] = memoIt->second;
          op->getResult(0).replaceAllUsesWith(constOp->getResult(0));
          op->erase();
          return true;
        }
      }
      return false;
    }

    // Look up callee function
    auto calleeFunc = sema.symbolTable->lookup<func::FuncOp>(calleeName);
    if (!calleeFunc || calleeFunc.isExternal())
      return false;
    if (calleeFunc.getBody().getBlocks().size() != 1)
      return false; // only handle single-block functions for now

    // Branch quota check
    sema.branchCount++;
    if (sema.branchCount > sema.branchQuota)
      return false;

    // Simulate: bind args to params, walk body, extract return
    auto &entryBlock = calleeFunc.getBody().front();
    llvm::DenseMap<Value, Attribute> localComptime;

    // Bind block arguments (function params) to comptime arg values
    for (unsigned i = 0; i < entryBlock.getNumArguments() && i < argAttrs.size(); i++)
      localComptime[entryBlock.getArgument(i)] = argAttrs[i];

    // Walk ops in the function body, evaluating comptimeially
    Attribute returnAttr;
    for (auto &bodyOp : entryBlock) {
      // Check for return
      if (bodyOp.getName().getStringRef() == "func.return") {
        if (bodyOp.getNumOperands() > 0) {
          auto retVal = bodyOp.getOperand(0);
          auto it = localComptime.find(retVal);
          if (it != localComptime.end())
            returnAttr = it->second;
        }
        break;
      }

      // Try to evaluate this op with local comptime values
      if (bodyOp.hasTrait<OpTrait::ConstantLike>() && bodyOp.getNumResults() == 1) {
        SmallVector<OpFoldResult> results;
        if (succeeded(bodyOp.fold(SmallVector<Attribute>{}, results)) &&
            results.size() == 1) {
          if (auto attr = llvm::dyn_cast_if_present<Attribute>(results[0]))
            localComptime[bodyOp.getResult(0)] = attr;
        }
        continue;
      }

      // Collect local comptime operands
      SmallVector<Attribute> opAttrs;
      bool allKnown = true;
      for (auto operand : bodyOp.getOperands()) {
        auto it = localComptime.find(operand);
        if (it != localComptime.end()) {
          opAttrs.push_back(it->second);
        } else {
          allKnown = false;
          break;
        }
      }
      if (!allKnown) continue;

      // Try fold
      if (bodyOp.getNumResults() > 0) {
        SmallVector<OpFoldResult> foldResults;
        if (succeeded(bodyOp.fold(opAttrs, foldResults)) &&
            foldResults.size() == bodyOp.getNumResults()) {
          for (unsigned i = 0; i < bodyOp.getNumResults(); i++) {
            if (auto attr = llvm::dyn_cast_if_present<Attribute>(foldResults[i]))
              localComptime[bodyOp.getResult(i)] = attr;
          }
        }
      }
    }

    if (!returnAttr) return false;

    // Cache the result
    sema.memoizedCalls[memoKey] = returnAttr;

    // Materialize and replace
    sema.comptimeValues[op->getResult(0)] = returnAttr;
    auto *dialect = sema.ctx->getLoadedDialect("cir");
    if (dialect) {
      OpBuilder builder(op);
      auto *constOp = dialect->materializeConstant(
          builder, returnAttr, op->getResult(0).getType(), op->getLoc());
      if (constOp) {
        sema.comptimeValues[constOp->getResult(0)] = returnAttr;
        op->getResult(0).replaceAllUsesWith(constOp->getResult(0));
        op->erase();
        return true;
      }
    }
    return false;
  }

  /// Query ComptimeEvaluable interface for ops that need special handling.
  bool tryComptimeEvaluable(Operation *op, cot::CIRSema &sema) {
    auto evaluable = dyn_cast<cir::ComptimeEvaluable>(op);
    if (!evaluable) return false;

    SmallVector<Attribute> operandAttrs;
    if (!collectOperandAttrs(op, sema, operandAttrs))
      return false;

    auto result = evaluable.comptimeEval(operandAttrs);
    if (!result) return false;

    // If the op has results, track the comptime value
    if (op->getNumResults() > 0)
      sema.comptimeValues[op->getResult(0)] = result;

    return true; // handled
  }
};

//===----------------------------------------------------------------------===//
// ComptimeConstruct
//===----------------------------------------------------------------------===//

class ComptimeConstruct : public cot::Construct {
public:
  llvm::StringRef getName() const override { return "comptime"; }

  void registerSemaSteps(cot::CIRSema &sema) override {
    sema.addStep(std::make_unique<ComptimeStep>());
  }
};

} // namespace

COT_REGISTER_CONSTRUCT(ComptimeConstruct)
