//===- Types.cpp - cot-optionals type implementations ---------*- C++ -*-===//
#include "optionals/Types.h"
#include "cot/CIR/CIRDialect.h"
#include "cot/CIR/CIRInterfaces.h"

#include "mlir/IR/Builders.h"
#include "mlir/IR/DialectImplementation.h"
#include "llvm/ADT/TypeSwitch.h"

using namespace mlir;

#define GET_TYPEDEF_CLASSES
#include "optionals/Types.cpp.inc"

/// Returns true if the payload type implements PointerLikeTypeInterface.
/// Pointer-like optionals use null-pointer optimization (the optional
/// IS the pointer, null = none). Value optionals use struct<(T, i1)>.
bool cir::OptionalType::isPointerLike() const {
  return mlir::isa<cir::PointerLikeTypeInterface>(getPayloadType());
}

void cir::registerOptionalsTypes(cir::CIRDialect *dialect) {
  dialect->registerConstructTypes<cir::OptionalType>();
}
