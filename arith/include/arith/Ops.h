//===- Ops.h - cot-core op declarations -----------------------*- C++ -*-===//
#ifndef ARITH_OPS_H
#define ARITH_OPS_H

#include "cot/CIR/CIRDialect.h"
#include "cot/CIR/CIRTypes.h"

#include "mlir/IR/BuiltinAttributeInterfaces.h"
#include "mlir/IR/BuiltinOps.h"
#include "mlir/IR/OpDefinition.h"
#include "mlir/IR/OpImplementation.h"
#include "mlir/IR/PatternMatch.h"
#include "mlir/Interfaces/CastInterfaces.h"
#include "mlir/Interfaces/InferTypeOpInterface.h"
#include "mlir/Interfaces/SideEffectInterfaces.h"

// Generated enum declarations (CmpIPredicate, CmpFPredicate)
#include "arith/Enums.h.inc"

// Generated op classes
#define GET_OP_CLASSES
#include "arith/Ops.h.inc"

#endif // ARITH_OPS_H
