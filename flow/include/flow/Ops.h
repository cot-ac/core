//===- Ops.h - cot-flow op declarations -----------------------*- C++ -*-===//
#ifndef FLOW_OPS_H
#define FLOW_OPS_H

#include "cot/CIR/CIRDialect.h"
#include "cot/CIR/CIROpInterfaces.h"

#include "mlir/IR/BuiltinOps.h"
#include "mlir/IR/OpDefinition.h"
#include "mlir/IR/OpImplementation.h"
#include "mlir/Interfaces/ControlFlowInterfaces.h"
#include "mlir/Interfaces/SideEffectInterfaces.h"

// Generated op classes
#define GET_OP_CLASSES
#include "flow/Ops.h.inc"

#endif // FLOW_OPS_H
