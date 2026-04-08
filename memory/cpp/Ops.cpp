//===- Ops.cpp - cot-memory op implementations ---------------*- C++ -*-===//
#include "memory/Ops.h"

#include "mlir/IR/Builders.h"
#include "mlir/IR/OpImplementation.h"

using namespace mlir;

#define GET_OP_CLASSES
#include "memory/Ops.cpp.inc"
