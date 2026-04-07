//===- Types.h - cot-arrays type declarations -----------------*- C++ -*-===//
#ifndef ARRAYS_TYPES_H
#define ARRAYS_TYPES_H

#include "mlir/IR/BuiltinTypes.h"
#include "mlir/IR/Types.h"

#define GET_TYPEDEF_CLASSES
#include "arrays/Types.h.inc"

namespace cir {
class CIRDialect;

/// Register ArrayType with the CIR dialect.
void registerArraysTypes(CIRDialect *dialect);
} // namespace cir

#endif // ARRAYS_TYPES_H
