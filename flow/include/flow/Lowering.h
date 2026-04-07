//===- Lowering.h - cot-flow lowering declarations ------------*- C++ -*-===//
#ifndef FLOW_LOWERING_H
#define FLOW_LOWERING_H

namespace mlir {
class RewritePatternSet;
class TypeConverter;
} // namespace mlir

namespace cot {

/// Add cot-flow CIR->LLVM conversion patterns.
void populateFlowPatterns(mlir::RewritePatternSet &patterns,
                          mlir::TypeConverter &typeConverter);

} // namespace cot

#endif // FLOW_LOWERING_H
