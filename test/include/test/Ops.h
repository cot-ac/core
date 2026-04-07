//===- Ops.h - cot-test op declarations -----------------------*- C++ -*-===//
#ifndef TEST_OPS_H
#define TEST_OPS_H

#include "cot/CIR/CIRDialect.h"

#include "mlir/IR/BuiltinOps.h"
#include "mlir/IR/OpDefinition.h"
#include "mlir/IR/OpImplementation.h"
#include "mlir/IR/RegionKindInterface.h"
#include "mlir/Interfaces/SideEffectInterfaces.h"

// Generated op classes
#define GET_OP_CLASSES
#include "test/Ops.h.inc"

#endif // TEST_OPS_H
