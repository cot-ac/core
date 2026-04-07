//===- Lowering.h - traits lowering declarations --------------*- C++ -*-===//
#ifndef TRAITS_LOWERING_H
#define TRAITS_LOWERING_H

namespace mlir {
class RewritePatternSet;
class TypeConverter;
} // namespace mlir

namespace cot {
void populateTraitsPatterns(mlir::RewritePatternSet &patterns,
                            mlir::TypeConverter &typeConverter);
} // namespace cot

#endif // TRAITS_LOWERING_H
