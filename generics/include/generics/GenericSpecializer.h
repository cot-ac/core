//===- GenericSpecializer.h - generic specialization pass ------*- C++ -*-===//
#ifndef GENERICS_GENERIC_SPECIALIZER_H
#define GENERICS_GENERIC_SPECIALIZER_H

#include <memory>

namespace mlir {
class Pass;
} // namespace mlir

namespace cot {

/// Monomorphization pass: clones generic functions for each concrete
/// type instantiation. Resolves cir.generic_apply → func.call to the
/// specialized clone. Must run BEFORE SemanticAnalysis.
/// Reference: Rust rustc_monomorphize, Swift GenericSpecializer.
std::unique_ptr<mlir::Pass> createGenericSpecializerPass();

} // namespace cot

#endif // GENERICS_GENERIC_SPECIALIZER_H
