//===- Ops.cpp - cot-core op implementations ------------------*- C++ -*-===//
//
// Verifiers, folders, canonicalizers, and custom parse/print for cot-core ops.
// Reference: MLIR Arith dialect (ArithOps.cpp)
//
//===----------------------------------------------------------------------===//
#include "arith/Ops.h"

#include "mlir/IR/Builders.h"
#include "mlir/IR/BuiltinTypes.h"
#include "mlir/IR/OpImplementation.h"
#include "mlir/IR/PatternMatch.h"

using namespace mlir;
using namespace cir;

// TableGen-generated enum implementations
#include "arith/Enums.cpp.inc"

// TableGen-generated op implementations
#define GET_OP_CLASSES
#include "arith/Ops.cpp.inc"

//===----------------------------------------------------------------------===//
// ConstantOp
//===----------------------------------------------------------------------===//

/// Verify that the value attribute type matches the result type.
LogicalResult ConstantOp::verify() {
  auto valType = getValue().getType();
  auto resType = getType();
  if (valType != resType)
    return emitOpError("value type ")
           << valType << " must match result type " << resType;
  return success();
}

/// Custom parser: `cir.constant 42 : i32`
/// The attribute `42 : i32` is a typed attribute — it already carries the type.
ParseResult ConstantOp::parse(OpAsmParser &parser, OperationState &result) {
  Attribute valueAttr;

  if (parser.parseAttribute(valueAttr))
    return failure();

  // The typed attribute carries the type — use it for the result.
  auto typedAttr = dyn_cast<TypedAttr>(valueAttr);
  if (!typedAttr)
    return parser.emitError(parser.getNameLoc(),
                            "expected typed attribute for constant");

  result.addAttribute("value", valueAttr);
  result.addTypes(typedAttr.getType());
  return success();
}

/// Custom printer: `cir.constant 42 : i32`
void ConstantOp::print(OpAsmPrinter &printer) {
  printer << " " << getValue();
}

/// Fold: constant always folds to its value attribute.
OpFoldResult ConstantOp::fold(FoldAdaptor adaptor) {
  return getValue();
}

//===----------------------------------------------------------------------===//
// AddOp
//===----------------------------------------------------------------------===//

/// Fold: constant folding (a + b → result), identity (x + 0 → x).
OpFoldResult AddOp::fold(FoldAdaptor adaptor) {
  // x + 0 → x (integer)
  if (auto rhs = dyn_cast_or_null<IntegerAttr>(adaptor.getRhs()))
    if (rhs.getValue().isZero())
      return getLhs();
  if (auto lhs = dyn_cast_or_null<IntegerAttr>(adaptor.getLhs()))
    if (lhs.getValue().isZero())
      return getRhs();

  // Constant fold: int + int
  if (auto lhs = dyn_cast_or_null<IntegerAttr>(adaptor.getLhs()))
    if (auto rhs = dyn_cast_or_null<IntegerAttr>(adaptor.getRhs()))
      return IntegerAttr::get(getType(), lhs.getValue() + rhs.getValue());

  return {};
}

/// Canonicalize: additional algebraic simplifications.
LogicalResult AddOp::canonicalize(AddOp op, PatternRewriter &rewriter) {
  // Fold handled by fold(). No additional canonicalizations yet.
  return failure();
}

//===----------------------------------------------------------------------===//
// SubOp
//===----------------------------------------------------------------------===//

/// Fold: x - 0 → x, x - x → 0, constant fold.
OpFoldResult SubOp::fold(FoldAdaptor adaptor) {
  // x - 0 → x
  if (auto rhs = dyn_cast_or_null<IntegerAttr>(adaptor.getRhs()))
    if (rhs.getValue().isZero())
      return getLhs();

  // x - x → 0
  if (getLhs() == getRhs())
    return IntegerAttr::get(getType(), 0);

  // Constant fold: int - int
  if (auto lhs = dyn_cast_or_null<IntegerAttr>(adaptor.getLhs()))
    if (auto rhs = dyn_cast_or_null<IntegerAttr>(adaptor.getRhs()))
      return IntegerAttr::get(getType(), lhs.getValue() - rhs.getValue());

  return {};
}

LogicalResult SubOp::canonicalize(SubOp op, PatternRewriter &rewriter) {
  return failure();
}

//===----------------------------------------------------------------------===//
// MulOp
//===----------------------------------------------------------------------===//

/// Fold: x * 0 → 0, x * 1 → x, constant fold.
OpFoldResult MulOp::fold(FoldAdaptor adaptor) {
  // x * 0 → 0
  if (auto rhs = dyn_cast_or_null<IntegerAttr>(adaptor.getRhs()))
    if (rhs.getValue().isZero())
      return rhs;
  if (auto lhs = dyn_cast_or_null<IntegerAttr>(adaptor.getLhs()))
    if (lhs.getValue().isZero())
      return lhs;

  // x * 1 → x
  if (auto rhs = dyn_cast_or_null<IntegerAttr>(adaptor.getRhs()))
    if (rhs.getValue().isOne())
      return getLhs();
  if (auto lhs = dyn_cast_or_null<IntegerAttr>(adaptor.getLhs()))
    if (lhs.getValue().isOne())
      return getRhs();

  // Constant fold: int * int
  if (auto lhs = dyn_cast_or_null<IntegerAttr>(adaptor.getLhs()))
    if (auto rhs = dyn_cast_or_null<IntegerAttr>(adaptor.getRhs()))
      return IntegerAttr::get(getType(), lhs.getValue() * rhs.getValue());

  return {};
}

LogicalResult MulOp::canonicalize(MulOp op, PatternRewriter &rewriter) {
  return failure();
}

//===----------------------------------------------------------------------===//
// Cast verifiers
//===----------------------------------------------------------------------===//

LogicalResult ExtSIOp::verify() {
  auto srcWidth = mlir::cast<IntegerType>(getInput().getType()).getWidth();
  auto dstWidth = mlir::cast<IntegerType>(getType()).getWidth();
  if (srcWidth >= dstWidth)
    return emitOpError("requires source width (")
           << srcWidth << ") < result width (" << dstWidth << ")";
  return success();
}

bool ExtSIOp::areCastCompatible(TypeRange inputs, TypeRange outputs) {
  if (inputs.size() != 1 || outputs.size() != 1)
    return false;
  auto src = dyn_cast<IntegerType>(inputs[0]);
  auto dst = dyn_cast<IntegerType>(outputs[0]);
  return src && dst && src.getWidth() < dst.getWidth();
}

LogicalResult ExtUIOp::verify() {
  auto srcWidth = mlir::cast<IntegerType>(getInput().getType()).getWidth();
  auto dstWidth = mlir::cast<IntegerType>(getType()).getWidth();
  if (srcWidth >= dstWidth)
    return emitOpError("requires source width (")
           << srcWidth << ") < result width (" << dstWidth << ")";
  return success();
}

bool ExtUIOp::areCastCompatible(TypeRange inputs, TypeRange outputs) {
  if (inputs.size() != 1 || outputs.size() != 1)
    return false;
  auto src = dyn_cast<IntegerType>(inputs[0]);
  auto dst = dyn_cast<IntegerType>(outputs[0]);
  return src && dst && src.getWidth() < dst.getWidth();
}

LogicalResult TruncIOp::verify() {
  auto srcWidth = mlir::cast<IntegerType>(getInput().getType()).getWidth();
  auto dstWidth = mlir::cast<IntegerType>(getType()).getWidth();
  if (srcWidth <= dstWidth)
    return emitOpError("requires source width (")
           << srcWidth << ") > result width (" << dstWidth << ")";
  return success();
}

bool TruncIOp::areCastCompatible(TypeRange inputs, TypeRange outputs) {
  if (inputs.size() != 1 || outputs.size() != 1)
    return false;
  auto src = dyn_cast<IntegerType>(inputs[0]);
  auto dst = dyn_cast<IntegerType>(outputs[0]);
  return src && dst && src.getWidth() > dst.getWidth();
}

LogicalResult ExtFOp::verify() {
  auto srcWidth = mlir::cast<FloatType>(getInput().getType()).getWidth();
  auto dstWidth = mlir::cast<FloatType>(getType()).getWidth();
  if (srcWidth >= dstWidth)
    return emitOpError("requires source float width (")
           << srcWidth << ") < result float width (" << dstWidth << ")";
  return success();
}

bool ExtFOp::areCastCompatible(TypeRange inputs, TypeRange outputs) {
  if (inputs.size() != 1 || outputs.size() != 1)
    return false;
  auto src = dyn_cast<FloatType>(inputs[0]);
  auto dst = dyn_cast<FloatType>(outputs[0]);
  return src && dst && src.getWidth() < dst.getWidth();
}

LogicalResult TruncFOp::verify() {
  auto srcWidth = mlir::cast<FloatType>(getInput().getType()).getWidth();
  auto dstWidth = mlir::cast<FloatType>(getType()).getWidth();
  if (srcWidth <= dstWidth)
    return emitOpError("requires source float width (")
           << srcWidth << ") > result float width (" << dstWidth << ")";
  return success();
}

bool TruncFOp::areCastCompatible(TypeRange inputs, TypeRange outputs) {
  if (inputs.size() != 1 || outputs.size() != 1)
    return false;
  auto src = dyn_cast<FloatType>(inputs[0]);
  auto dst = dyn_cast<FloatType>(outputs[0]);
  return src && dst && src.getWidth() > dst.getWidth();
}

// SIToFPOp and FPToSIOp — CastOpInterface. Always compatible.
bool SIToFPOp::areCastCompatible(TypeRange inputs, TypeRange outputs) {
  if (inputs.size() != 1 || outputs.size() != 1)
    return false;
  return isa<IntegerType>(inputs[0]) && isa<FloatType>(outputs[0]);
}

bool FPToSIOp::areCastCompatible(TypeRange inputs, TypeRange outputs) {
  if (inputs.size() != 1 || outputs.size() != 1)
    return false;
  return isa<FloatType>(inputs[0]) && isa<IntegerType>(outputs[0]);
}
