//===- Ops.cpp - cot-optionals op implementations ------------*- C++ -*-===//
#include "optionals/Ops.h"

#include "mlir/IR/Builders.h"
#include "mlir/IR/OpImplementation.h"

using namespace mlir;

//===----------------------------------------------------------------------===//
// IsNonNullOp custom assembly format
// Syntax: cir.is_non_null %opt : !cir.optional<i32> to i1
//===----------------------------------------------------------------------===//

ParseResult cir::IsNonNullOp::parse(OpAsmParser &parser,
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

void cir::IsNonNullOp::print(OpAsmPrinter &printer) {
  printer << " " << getInput();
  printer.printOptionalAttrDict((*this)->getAttrs());
  printer << " : ";
  printer.printType(getInput().getType());
  printer << " to ";
  printer.printType(getResult().getType());
}

#define GET_OP_CLASSES
#include "optionals/Ops.cpp.inc"
