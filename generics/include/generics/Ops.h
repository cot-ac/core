//===- Ops.h - generics op declarations -----------------------*- C++ -*-===//
#ifndef GENERICS_OPS_H
#define GENERICS_OPS_H

#include "cot/CIR/CIRDialect.h"
#include "generics/Types.h"

#include "mlir/IR/BuiltinOps.h"
#include "mlir/IR/OpDefinition.h"
#include "mlir/IR/OpImplementation.h"
#include "mlir/IR/SymbolTable.h"
#include "mlir/Interfaces/SideEffectInterfaces.h"

// Generated op classes
#define GET_OP_CLASSES
#include "generics/Ops.h.inc"

#endif // GENERICS_OPS_H
