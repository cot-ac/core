//===- Types.h - cot-optionals type declarations --------------*- C++ -*-===//
#ifndef OPTIONALS_TYPES_H
#define OPTIONALS_TYPES_H

#include "cot/CIR/CIRInterfaces.h"
#include "mlir/IR/BuiltinTypes.h"
#include "mlir/IR/Types.h"

#define GET_TYPEDEF_CLASSES
#include "optionals/Types.h.inc"

namespace cir {
class CIRDialect;

/// Register OptionalType with the CIR dialect.
/// Must be called from Types.cpp where the storage classes are complete.
void registerOptionalsTypes(CIRDialect *dialect);
} // namespace cir

#endif // OPTIONALS_TYPES_H
