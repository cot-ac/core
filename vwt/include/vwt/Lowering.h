//===- Lowering.h - VWT lowering declarations -----------------*- C++ -*-===//
#ifndef VWT_LOWERING_H
#define VWT_LOWERING_H

namespace mlir {
class RewritePatternSet;
class TypeConverter;
} // namespace mlir

namespace cot {
void populateVWTPatterns(mlir::RewritePatternSet &patterns,
                         mlir::TypeConverter &typeConverter);
} // namespace cot

#endif // VWT_LOWERING_H
