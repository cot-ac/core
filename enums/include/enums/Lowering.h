//===- Lowering.h - enums lowering declarations ---------------*- C++ -*-===//
#ifndef ENUMS_LOWERING_H
#define ENUMS_LOWERING_H

namespace mlir {
class RewritePatternSet;
class TypeConverter;
} // namespace mlir

namespace cot {

/// Add enums CIR->LLVM conversion patterns.
void populateEnumsPatterns(mlir::RewritePatternSet &patterns,
                           mlir::TypeConverter &typeConverter);

} // namespace cot

#endif // ENUMS_LOWERING_H
