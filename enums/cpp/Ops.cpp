//===- Ops.cpp - enums op implementations ---------------------*- C++ -*-===//
#include "enums/Ops.h"

#include "mlir/IR/OpImplementation.h"

using namespace mlir;

// EnumValueOp custom assembly format:
//   %tag = cir.enum_value %val : !cir.enum<...> to i32
ParseResult cir::EnumValueOp::parse(OpAsmParser &parser,
                                     OperationState &result) {
  OpAsmParser::UnresolvedOperand input;
  Type enumType, resultType;
  if (parser.parseOperand(input) || parser.parseOptionalAttrDict(result.attributes) ||
      parser.parseColonType(enumType) || parser.parseKeyword("to") ||
      parser.parseType(resultType) ||
      parser.resolveOperand(input, enumType, result.operands))
    return failure();
  result.addTypes(resultType);
  return success();
}

void cir::EnumValueOp::print(OpAsmPrinter &printer) {
  printer << " " << getInput();
  printer.printOptionalAttrDict((*this)->getAttrs());
  printer << " : ";
  printer.printType(getInput().getType());
  printer << " to ";
  printer.printType(getResult().getType());
}

#define GET_OP_CLASSES
#include "enums/Ops.cpp.inc"
