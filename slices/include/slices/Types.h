//===- Types.h - cot-slices type declarations -----------------*- C++ -*-===//
#ifndef SLICES_TYPES_H
#define SLICES_TYPES_H

#include "mlir/IR/BuiltinTypes.h"
#include "mlir/IR/Types.h"

#define GET_TYPEDEF_CLASSES
#include "slices/Types.h.inc"

namespace cir {
class CIRDialect;

/// Register SliceType with the CIR dialect.
void registerSlicesTypes(CIRDialect *dialect);
} // namespace cir

#endif // SLICES_TYPES_H
