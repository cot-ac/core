//===- Types.h - cot-errors type declarations -----------------*- C++ -*-===//
#ifndef ERRORS_TYPES_H
#define ERRORS_TYPES_H

#include "mlir/IR/BuiltinTypes.h"
#include "mlir/IR/Types.h"

#define GET_TYPEDEF_CLASSES
#include "errors/Types.h.inc"

namespace cir {
class CIRDialect;

/// Register ErrorUnionType with the CIR dialect.
/// Must be called from Types.cpp where the storage classes are complete.
void registerErrorsTypes(CIRDialect *dialect);
} // namespace cir

#endif // ERRORS_TYPES_H
