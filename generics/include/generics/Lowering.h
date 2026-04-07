//===- Lowering.h - generics lowering declarations ------------*- C++ -*-===//
#ifndef GENERICS_LOWERING_H
#define GENERICS_LOWERING_H

namespace mlir {
class RewritePatternSet;
class TypeConverter;
} // namespace mlir

namespace cot {

/// Add generics CIR->LLVM conversion patterns.
void populateGenericsPatterns(mlir::RewritePatternSet &patterns,
                              mlir::TypeConverter &typeConverter);

} // namespace cot

#endif // GENERICS_LOWERING_H
