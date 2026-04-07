//===- Ops.h - VWT op declarations ----------------------------*- C++ -*-===//
#ifndef VWT_OPS_H
#define VWT_OPS_H

#include "cot/CIR/CIRDialect.h"

#include "mlir/IR/BuiltinOps.h"
#include "mlir/IR/OpDefinition.h"
#include "mlir/IR/OpImplementation.h"
#include "mlir/Interfaces/SideEffectInterfaces.h"

#define GET_OP_CLASSES
#include "vwt/Ops.h.inc"

#endif // VWT_OPS_H
