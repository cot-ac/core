//===- Ops.h - cot-errors op declarations ---------------------*- C++ -*-===//
#ifndef ERRORS_OPS_H
#define ERRORS_OPS_H

#include "cot/CIR/CIRDialect.h"
#include "errors/Types.h"

#include "mlir/IR/BuiltinOps.h"
#include "mlir/IR/OpDefinition.h"
#include "mlir/IR/OpImplementation.h"
#include "mlir/Interfaces/InferTypeOpInterface.h"
#include "mlir/Interfaces/SideEffectInterfaces.h"

// Generated op classes
#define GET_OP_CLASSES
#include "errors/Ops.h.inc"

#endif // ERRORS_OPS_H
