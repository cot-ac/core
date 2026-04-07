//===- Ops.cpp - VWT op implementations -----------------------*- C++ -*-===//
#include "vwt/Ops.h"

#include "mlir/IR/Builders.h"
#include "mlir/IR/OpImplementation.h"

using namespace mlir;

// All VWT ops use declarative assemblyFormat — no custom parse/print needed.

#define GET_OP_CLASSES
#include "vwt/Ops.cpp.inc"
