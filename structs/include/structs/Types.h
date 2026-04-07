//===- Types.h - cot-structs type declarations ----------------*- C++ -*-===//
#ifndef STRUCTS_TYPES_H
#define STRUCTS_TYPES_H

#include "mlir/IR/BuiltinTypes.h"
#include "mlir/IR/Types.h"

#define GET_TYPEDEF_CLASSES
#include "structs/Types.h.inc"

namespace cir {
class CIRDialect;

/// Register StructType with the CIR dialect.
/// Must be called from Types.cpp where the storage classes are complete.
void registerStructsTypes(CIRDialect *dialect);
} // namespace cir

#endif // STRUCTS_TYPES_H
