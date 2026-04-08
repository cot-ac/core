//===- Types.cpp - cot-errors type implementations -----------*- C++ -*-===//
#include "errors/Types.h"
#include "cot/CIR/CIRDialect.h"

#include "mlir/IR/Builders.h"
#include "mlir/IR/DialectImplementation.h"
#include "llvm/ADT/TypeSwitch.h"

using namespace mlir;

#define GET_TYPEDEF_CLASSES
#include "errors/Types.cpp.inc"

void cir::registerErrorsTypes(cir::CIRDialect *dialect) {
  dialect->registerConstructTypes<cir::ErrorUnionType>();
}
