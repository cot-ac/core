//===- Lowering.h - cot-core lowering declarations ------------*- C++ -*-===//
#ifndef ARITH_LOWERING_H
#define ARITH_LOWERING_H

namespace mlir {
class RewritePatternSet;
class TypeConverter;
} // namespace mlir

namespace cot {

/// Add cot-core CIR->LLVM conversion patterns.
void populateArithmeticPatterns(mlir::RewritePatternSet &patterns,
                                mlir::TypeConverter &typeConverter);

} // namespace cot

#endif // ARITH_LOWERING_H
