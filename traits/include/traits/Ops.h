//===- Ops.h - traits op declarations -------------------------*- C++ -*-===//
#ifndef TRAITS_OPS_H
#define TRAITS_OPS_H

#include "cot/CIR/CIRDialect.h"
#include "traits/Types.h"

#include "mlir/IR/BuiltinOps.h"
#include "mlir/IR/OpDefinition.h"
#include "mlir/IR/OpImplementation.h"
#include "mlir/IR/SymbolTable.h"
#include "mlir/Interfaces/SideEffectInterfaces.h"

#define GET_OP_CLASSES
#include "traits/Ops.h.inc"

#endif // TRAITS_OPS_H
