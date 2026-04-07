//===- Types.h - generics type declarations --------------------*- C++ -*-===//
#ifndef GENERICS_TYPES_H
#define GENERICS_TYPES_H

#include "mlir/IR/BuiltinTypes.h"
#include "mlir/IR/Types.h"

#define GET_TYPEDEF_CLASSES
#include "generics/Types.h.inc"

namespace cir {
class CIRDialect;

/// Register TypeParamType with the CIR dialect.
void registerGenericsTypes(CIRDialect *dialect);
} // namespace cir

#endif // GENERICS_TYPES_H
