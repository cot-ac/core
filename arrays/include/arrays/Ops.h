//===- Ops.h - cot-arrays op declarations ---------------------*- C++ -*-===//
#ifndef ARRAYS_OPS_H
#define ARRAYS_OPS_H

#include "cot/CIR/CIRDialect.h"
#include "arrays/Types.h"

#include "mlir/IR/BuiltinOps.h"
#include "mlir/IR/OpDefinition.h"
#include "mlir/IR/OpImplementation.h"
#include "mlir/Interfaces/InferTypeOpInterface.h"
#include "mlir/Interfaces/SideEffectInterfaces.h"

#define GET_OP_CLASSES
#include "arrays/Ops.h.inc"

#endif // ARRAYS_OPS_H
