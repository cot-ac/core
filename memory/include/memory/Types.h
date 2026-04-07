//===- Types.h - cot-memory type declarations -----------------*- C++ -*-===//
#ifndef MEMORY_TYPES_H
#define MEMORY_TYPES_H

#include "cot/CIR/CIRInterfaces.h"
#include "mlir/IR/BuiltinTypes.h"
#include "mlir/IR/Types.h"

#define GET_TYPEDEF_CLASSES
#include "memory/Types.h.inc"

namespace cir {
class CIRDialect;

/// Register PointerType and RefType with the CIR dialect.
/// Must be called from Types.cpp where the storage classes are complete.
void registerMemoryTypes(CIRDialect *dialect);
} // namespace cir

#endif // MEMORY_TYPES_H
