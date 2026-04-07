//===- Types.h - traits type declarations ---------------------*- C++ -*-===//
#ifndef TRAITS_TYPES_H
#define TRAITS_TYPES_H

#include "mlir/IR/BuiltinTypes.h"
#include "mlir/IR/Types.h"

#define GET_TYPEDEF_CLASSES
#include "traits/Types.h.inc"

namespace cir {
class CIRDialect;
void registerTraitsTypes(CIRDialect *dialect);
} // namespace cir

#endif // TRAITS_TYPES_H
