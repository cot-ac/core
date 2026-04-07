//===- Transform.h - cot-core pass declarations ---------------*- C++ -*-===//
#ifndef ARITH_TRANSFORM_H
#define ARITH_TRANSFORM_H

#include <memory>

namespace mlir {
class Pass;
} // namespace mlir

namespace cot {

/// Type checking and cast insertion for CIR.
std::unique_ptr<mlir::Pass> createSemanticAnalysisPass();

} // namespace cot

#endif // ARITH_TRANSFORM_H
