//===- Types.h - enums type declarations ----------------------*- C++ -*-===//
#ifndef ENUMS_TYPES_H
#define ENUMS_TYPES_H

#include "mlir/IR/BuiltinTypes.h"
#include "mlir/IR/Types.h"

#define GET_TYPEDEF_CLASSES
#include "enums/Types.h.inc"

namespace cir {
class CIRDialect;

/// Register EnumType with the CIR dialect.
void registerEnumsTypes(CIRDialect *dialect);
} // namespace cir

#endif // ENUMS_TYPES_H
