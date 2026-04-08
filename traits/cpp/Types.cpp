//===- Types.cpp - traits type implementations ----------------*- C++ -*-===//
#include "traits/Types.h"
#include "cot/CIR/CIRDialect.h"

#include "mlir/IR/Builders.h"
#include "mlir/IR/DialectImplementation.h"
#include "llvm/ADT/TypeSwitch.h"

using namespace mlir;

#define GET_TYPEDEF_CLASSES
#include "traits/Types.cpp.inc"

void cir::registerTraitsTypes(cir::CIRDialect *dialect) {
  dialect->registerConstructTypes<cir::ExistentialType>();
}
