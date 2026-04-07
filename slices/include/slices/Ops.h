//===- Ops.h - cot-slices op declarations ---------------------*- C++ -*-===//
#ifndef SLICES_OPS_H
#define SLICES_OPS_H

#include "cot/CIR/CIRDialect.h"
#include "slices/Types.h"

#include "mlir/IR/BuiltinOps.h"
#include "mlir/IR/OpDefinition.h"
#include "mlir/IR/OpImplementation.h"
#include "mlir/Interfaces/InferTypeOpInterface.h"
#include "mlir/Interfaces/SideEffectInterfaces.h"

#define GET_OP_CLASSES
#include "slices/Ops.h.inc"

#endif // SLICES_OPS_H
