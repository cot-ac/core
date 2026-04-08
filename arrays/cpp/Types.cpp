//===- Types.cpp - cot-arrays type implementations -----------*- C++ -*-===//
#include "arrays/Types.h"
#include "cot/CIR/CIRDialect.h"

#include "mlir/IR/Builders.h"
#include "mlir/IR/DialectImplementation.h"
#include "llvm/ADT/TypeSwitch.h"

using namespace mlir;

#define GET_TYPEDEF_CLASSES
#include "arrays/Types.cpp.inc"

void cir::registerArraysTypes(cir::CIRDialect *dialect) {
  dialect->registerConstructTypes<cir::ArrayType>();
}
