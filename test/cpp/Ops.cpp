//===- Ops.cpp - cot-test op implementations -----------------*- C++ -*-===//
#include "test/Ops.h"

#include "mlir/IR/Builders.h"
#include "mlir/IR/OpImplementation.h"

using namespace mlir;

#define GET_OP_CLASSES
#include "test/Ops.cpp.inc"
