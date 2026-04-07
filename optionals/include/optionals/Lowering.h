//===- Lowering.h - cot-optionals lowering declarations -------*- C++ -*-===//
#ifndef OPTIONALS_LOWERING_H
#define OPTIONALS_LOWERING_H

namespace mlir {
class RewritePatternSet;
class TypeConverter;
} // namespace mlir

namespace cot {

/// Add cot-optionals CIR->LLVM conversion patterns.
void populateOptionalsPatterns(mlir::RewritePatternSet &patterns,
                               mlir::TypeConverter &typeConverter);

} // namespace cot

#endif // OPTIONALS_LOWERING_H
