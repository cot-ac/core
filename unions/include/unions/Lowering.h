//===- Lowering.h - unions lowering declarations --------------*- C++ -*-===//
#ifndef UNIONS_LOWERING_H
#define UNIONS_LOWERING_H

namespace mlir {
class RewritePatternSet;
class TypeConverter;
} // namespace mlir

namespace cot {

/// Add unions CIR->LLVM conversion patterns.
void populateUnionsPatterns(mlir::RewritePatternSet &patterns,
                            mlir::TypeConverter &typeConverter);

} // namespace cot

#endif // UNIONS_LOWERING_H
