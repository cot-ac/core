//===- Ops.h - unions op declarations -------------------------*- C++ -*-===//
#ifndef UNIONS_OPS_H
#define UNIONS_OPS_H

#include "cot/CIR/CIRDialect.h"
#include "unions/Types.h"

#include "mlir/IR/BuiltinOps.h"
#include "mlir/IR/OpDefinition.h"
#include "mlir/IR/OpImplementation.h"
#include "mlir/Interfaces/InferTypeOpInterface.h"
#include "mlir/Interfaces/SideEffectInterfaces.h"

// Generated op classes
#define GET_OP_CLASSES
#include "unions/Ops.h.inc"

#endif // UNIONS_OPS_H
