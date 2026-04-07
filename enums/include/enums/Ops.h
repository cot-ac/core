//===- Ops.h - enums op declarations --------------------------*- C++ -*-===//
#ifndef ENUMS_OPS_H
#define ENUMS_OPS_H

#include "cot/CIR/CIRDialect.h"
#include "enums/Types.h"

#include "mlir/IR/BuiltinOps.h"
#include "mlir/IR/OpDefinition.h"
#include "mlir/IR/OpImplementation.h"
#include "mlir/Interfaces/InferTypeOpInterface.h"
#include "mlir/Interfaces/SideEffectInterfaces.h"

// Generated op classes
#define GET_OP_CLASSES
#include "enums/Ops.h.inc"

#endif // ENUMS_OPS_H
