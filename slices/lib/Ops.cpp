//===- Ops.cpp - cot-slices op implementations ---------------*- C++ -*-===//
#include "slices/Ops.h"

#include "mlir/IR/Builders.h"
#include "mlir/IR/OpImplementation.h"

using namespace mlir;

#define GET_OP_CLASSES
#include "slices/Ops.cpp.inc"
