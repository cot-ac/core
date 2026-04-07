//===- Lowering.h - cot-test lowering declarations ------------*- C++ -*-===//
#ifndef TEST_LOWERING_H
#define TEST_LOWERING_H

namespace mlir {
class RewritePatternSet;
class TypeConverter;
} // namespace mlir

namespace cot {

/// Add cot-test CIR->LLVM conversion patterns.
void populateTestPatterns(mlir::RewritePatternSet &patterns,
                          mlir::TypeConverter &typeConverter);

} // namespace cot

#endif // TEST_LOWERING_H
