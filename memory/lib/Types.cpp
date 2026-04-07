//===- Types.cpp - cot-memory type implementations -----------*- C++ -*-===//
#include "memory/Types.h"
#include "cot/CIR/CIRDialect.h"

#include "mlir/IR/Builders.h"
#include "mlir/IR/DialectImplementation.h"
#include "llvm/ADT/TypeSwitch.h"

using namespace mlir;

#define GET_TYPEDEF_CLASSES
#include "memory/Types.cpp.inc"

void cir::registerMemoryTypes(cir::CIRDialect *dialect) {
  dialect->registerConstructTypes<cir::PointerType, cir::RefType>();
}
