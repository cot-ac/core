//===- Types.cpp - cot-slices type implementations -----------*- C++ -*-===//
#include "slices/Types.h"
#include "cot/CIR/CIRDialect.h"

#include "mlir/IR/Builders.h"
#include "mlir/IR/DialectImplementation.h"
#include "llvm/ADT/TypeSwitch.h"

using namespace mlir;

#define GET_TYPEDEF_CLASSES
#include "slices/Types.cpp.inc"

void cir::registerSlicesTypes(cir::CIRDialect *dialect) {
  dialect->registerConstructTypes<cir::SliceType>();
}
