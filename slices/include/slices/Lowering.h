//===- Lowering.h - cot-slices lowering declarations ----------*- C++ -*-===//
#ifndef SLICES_LOWERING_H
#define SLICES_LOWERING_H

namespace mlir {
class RewritePatternSet;
class TypeConverter;
} // namespace mlir

namespace cot {

/// Add cot-slices CIR->LLVM conversion patterns.
void populateSlicesPatterns(mlir::RewritePatternSet &patterns,
                            mlir::TypeConverter &typeConverter);

} // namespace cot

#endif // SLICES_LOWERING_H
