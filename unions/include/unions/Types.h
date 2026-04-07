//===- Types.h - unions type declarations ----------------------*- C++ -*-===//
#ifndef UNIONS_TYPES_H
#define UNIONS_TYPES_H

#include "mlir/IR/BuiltinTypes.h"
#include "mlir/IR/Types.h"

#define GET_TYPEDEF_CLASSES
#include "unions/Types.h.inc"

namespace cir {
class CIRDialect;

/// Register TaggedUnionType with the CIR dialect.
void registerUnionsTypes(CIRDialect *dialect);
} // namespace cir

#endif // UNIONS_TYPES_H
