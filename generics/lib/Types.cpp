//===- Types.cpp - generics type implementations --------------*- C++ -*-===//
#include "generics/Types.h"
#include "cot/CIR/CIRDialect.h"

#include "mlir/IR/Builders.h"
#include "mlir/IR/DialectImplementation.h"
#include "llvm/ADT/TypeSwitch.h"

using namespace mlir;

#define GET_TYPEDEF_CLASSES
#include "generics/Types.cpp.inc"

void cir::registerGenericsTypes(cir::CIRDialect *dialect) {
  dialect->registerConstructTypes<cir::TypeParamType>();
}
