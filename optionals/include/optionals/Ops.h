//===- Ops.h - cot-optionals op declarations ------------------*- C++ -*-===//
#ifndef OPTIONALS_OPS_H
#define OPTIONALS_OPS_H

#include "cot/CIR/CIRDialect.h"
#include "optionals/Types.h"

#include "mlir/IR/BuiltinOps.h"
#include "mlir/IR/OpDefinition.h"
#include "mlir/IR/OpImplementation.h"
#include "mlir/Interfaces/InferTypeOpInterface.h"
#include "mlir/Interfaces/SideEffectInterfaces.h"

// Generated op classes
#define GET_OP_CLASSES
#include "optionals/Ops.h.inc"

#endif // OPTIONALS_OPS_H
