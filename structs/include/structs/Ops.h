//===- Ops.h - cot-structs op declarations --------------------*- C++ -*-===//
#ifndef STRUCTS_OPS_H
#define STRUCTS_OPS_H

#include "cot/CIR/CIRDialect.h"
#include "structs/Types.h"

#include "mlir/IR/BuiltinOps.h"
#include "mlir/IR/OpDefinition.h"
#include "mlir/IR/OpImplementation.h"
#include "mlir/Interfaces/InferTypeOpInterface.h"
#include "mlir/Interfaces/SideEffectInterfaces.h"

// Generated op classes
#define GET_OP_CLASSES
#include "structs/Ops.h.inc"

#endif // STRUCTS_OPS_H
