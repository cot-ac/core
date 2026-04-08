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

    // Skip terminators and ops with no results (handled by ComptimeEvaluable)
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
    bool allResolved = true;
    for (unsigned i = 0; i < op->getNumResults(); i++) {
      if (auto attr = llvm::dyn_cast_if_present<Attribute>(foldResults[i])) {
        // Fold produced a constant — track it
        sema.comptimeValues[op->getResult(i)] = attr;
      } else if (auto val = llvm::dyn_cast_if_present<Value>(foldResults[i])) {
        // Fold produced an existing value — propagate comptime status
        auto it = sema.comptimeValues.find(val);
        if (it != sema.comptimeValues.end())
          sema.comptimeValues[op->getResult(i)] = it->second;
        op->getResult(i).replaceAllUsesWith(val);
      } else {
        allResolved = false;
      }
    }

    // If all results are constant Attributes, we can try to materialize
    // and erase. But materialization requires dialect support (cir.constant).
    // For now, just track the values — downstream steps see them as known.
    // Full materialization happens when we implement dialect materializeConstant.
    return false; // don't erase — tracking is sufficient for comptime propagation
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
