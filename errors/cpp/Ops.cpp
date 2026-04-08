//===- Ops.cpp - cot-errors op implementations ---------------*- C++ -*-===//
#include "errors/Ops.h"

#include "mlir/IR/Builders.h"
#include "mlir/IR/OpImplementation.h"

using namespace mlir;

//===----------------------------------------------------------------------===//
// IsErrorOp custom assembly format
// Syntax: cir.is_error %eu : !cir.error_union<i32> to i1
//===----------------------------------------------------------------------===//

ParseResult cir::IsErrorOp::parse(OpAsmParser &parser,
                                   OperationState &result) {
  OpAsmParser::UnresolvedOperand input;
  Type inputType, resultType;

  if (parser.parseOperand(input) ||
      parser.parseOptionalAttrDict(result.attributes) ||
      parser.parseColonType(inputType) ||
      parser.parseKeyword("to") ||
      parser.parseType(resultType))
    return failure();

  if (parser.resolveOperand(input, inputType, result.operands))
    return failure();

  result.addTypes(resultType);
  return success();
}

void cir::IsErrorOp::print(OpAsmPrinter &printer) {
  printer << " " << getInput();
  printer.printOptionalAttrDict((*this)->getAttrs());
  printer << " : ";
  printer.printType(getInput().getType());
  printer << " to ";
  printer.printType(getResult().getType());
}

//===----------------------------------------------------------------------===//
// ErrorCodeOp custom assembly format
// Syntax: cir.error_code %eu : !cir.error_union<i32> to i16
//===----------------------------------------------------------------------===//

ParseResult cir::ErrorCodeOp::parse(OpAsmParser &parser,
                                     OperationState &result) {
  OpAsmParser::UnresolvedOperand input;
  Type inputType, resultType;

  if (parser.parseOperand(input) ||
      parser.parseOptionalAttrDict(result.attributes) ||
      parser.parseColonType(inputType) ||
      parser.parseKeyword("to") ||
      parser.parseType(resultType))
    return failure();

  if (parser.resolveOperand(input, inputType, result.operands))
    return failure();

  result.addTypes(resultType);
  return success();
}

void cir::ErrorCodeOp::print(OpAsmPrinter &printer) {
  printer << " " << getInput();
  printer.printOptionalAttrDict((*this)->getAttrs());
  printer << " : ";
  printer.printType(getInput().getType());
  printer << " to ";
  printer.printType(getResult().getType());
}

#define GET_OP_CLASSES
#include "errors/Ops.cpp.inc"
