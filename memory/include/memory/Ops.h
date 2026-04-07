//===- Ops.h - cot-memory op declarations ---------------------*- C++ -*-===//
#ifndef MEMORY_OPS_H
#define MEMORY_OPS_H

#include "cot/CIR/CIRDialect.h"
#include "memory/Types.h"

#include "mlir/IR/BuiltinOps.h"
#include "mlir/IR/OpDefinition.h"
#include "mlir/IR/OpImplementation.h"
#include "mlir/Interfaces/InferTypeOpInterface.h"
#include "mlir/Interfaces/SideEffectInterfaces.h"

// Generated op classes
#define GET_OP_CLASSES
#include "memory/Ops.h.inc"

#endif // MEMORY_OPS_H
