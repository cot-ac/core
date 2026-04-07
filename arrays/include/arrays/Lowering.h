//===- Lowering.h - cot-arrays lowering declarations ----------*- C++ -*-===//
#ifndef ARRAYS_LOWERING_H
#define ARRAYS_LOWERING_H

namespace mlir {
class RewritePatternSet;
class TypeConverter;
} // namespace mlir

namespace cot {

/// Add cot-arrays CIR->LLVM conversion patterns.
void populateArraysPatterns(mlir::RewritePatternSet &patterns,
                            mlir::TypeConverter &typeConverter);

} // namespace cot

#endif // ARRAYS_LOWERING_H
