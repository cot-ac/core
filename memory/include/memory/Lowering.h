//===- Lowering.h - cot-memory lowering declarations ----------*- C++ -*-===//
#ifndef MEMORY_LOWERING_H
#define MEMORY_LOWERING_H

namespace mlir {
class RewritePatternSet;
class TypeConverter;
} // namespace mlir

namespace cot {

/// Add cot-memory CIR->LLVM conversion patterns.
void populateMemoryPatterns(mlir::RewritePatternSet &patterns,
                            mlir::TypeConverter &typeConverter);

} // namespace cot

#endif // MEMORY_LOWERING_H
