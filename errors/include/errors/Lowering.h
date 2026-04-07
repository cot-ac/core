//===- Lowering.h - cot-errors lowering declarations ----------*- C++ -*-===//
#ifndef ERRORS_LOWERING_H
#define ERRORS_LOWERING_H

namespace mlir {
class RewritePatternSet;
class TypeConverter;
} // namespace mlir

namespace cot {

/// Add cot-errors CIR->LLVM conversion patterns.
void populateErrorsPatterns(mlir::RewritePatternSet &patterns,
                            mlir::TypeConverter &typeConverter);

} // namespace cot

#endif // ERRORS_LOWERING_H
