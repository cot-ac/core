//===- Lowering.h - cot-structs lowering declarations ---------*- C++ -*-===//
#ifndef STRUCTS_LOWERING_H
#define STRUCTS_LOWERING_H

namespace mlir {
class RewritePatternSet;
class TypeConverter;
} // namespace mlir

namespace cot {

/// Add cot-structs CIR->LLVM conversion patterns.
void populateStructsPatterns(mlir::RewritePatternSet &patterns,
                             mlir::TypeConverter &typeConverter);

} // namespace cot

#endif // STRUCTS_LOWERING_H
