//===- Ops.cpp - unions op implementations --------------------*- C++ -*-===//
#include "unions/Ops.h"

#include "mlir/IR/OpImplementation.h"

using namespace mlir;

// UnionInitOp: cir.union_init "Variant"(%payload) : T to !cir.tagged_union<...>
// or:          cir.union_init "Variant" : !cir.tagged_union<...>  (no payload)
ParseResult cir::UnionInitOp::parse(OpAsmParser &parser,
                                     OperationState &result) {
  StringAttr variant;
  if (parser.parseAttribute(variant, "variant", result.attributes))
    return failure();

  // Optional payload operand
  OpAsmParser::UnresolvedOperand payloadOp;
  Type payloadType;
  bool hasPayload = false;
  if (succeeded(parser.parseOptionalLParen())) {
    if (parser.parseOperand(payloadOp) || parser.parseRParen() ||
        parser.parseColon() || parser.parseType(payloadType))
      return failure();
    hasPayload = true;
  }

  if (parser.parseKeyword("to"))
    return failure();

  Type resultType;
  if (parser.parseType(resultType))
    return failure();

  if (hasPayload) {
    if (parser.resolveOperand(payloadOp, payloadType, result.operands))
      return failure();
  }

  result.addTypes(resultType);
  return success();
}

void cir::UnionInitOp::print(OpAsmPrinter &printer) {
  printer << " \"" << getVariant() << "\"";
  if (getPayload()) {
    printer << "(" << getPayload() << ") : ";
    printer.printType(getPayload().getType());
  }
  printer << " to ";
  printer.printType(getResult().getType());
}

// UnionTagOp: cir.union_tag %val : !cir.tagged_union<...>
ParseResult cir::UnionTagOp::parse(OpAsmParser &parser,
                                    OperationState &result) {
  OpAsmParser::UnresolvedOperand input;
  Type inputType;
  if (parser.parseOperand(input) ||
      parser.parseOptionalAttrDict(result.attributes) ||
      parser.parseColonType(inputType) ||
      parser.resolveOperand(input, inputType, result.operands))
    return failure();
  result.addTypes(IntegerType::get(parser.getContext(), 8));
  return success();
}

void cir::UnionTagOp::print(OpAsmPrinter &printer) {
  printer << " " << getInput();
  printer.printOptionalAttrDict((*this)->getAttrs());
  printer << " : ";
  printer.printType(getInput().getType());
}

// UnionPayloadOp: cir.union_payload "Variant" %val : !cir.tagged_union<...> to T
ParseResult cir::UnionPayloadOp::parse(OpAsmParser &parser,
                                        OperationState &result) {
  StringAttr variant;
  if (parser.parseAttribute(variant, "variant", result.attributes))
    return failure();

  OpAsmParser::UnresolvedOperand input;
  Type inputType, resultType;
  if (parser.parseOperand(input) ||
      parser.parseOptionalAttrDict(result.attributes) ||
      parser.parseColonType(inputType) ||
      parser.parseKeyword("to") ||
      parser.parseType(resultType) ||
      parser.resolveOperand(input, inputType, result.operands))
    return failure();
  result.addTypes(resultType);
  return success();
}

void cir::UnionPayloadOp::print(OpAsmPrinter &printer) {
  printer << " \"" << getVariant() << "\" " << getInput();
  printer.printOptionalAttrDict((*this)->getAttrs(), {"variant"});
  printer << " : ";
  printer.printType(getInput().getType());
  printer << " to ";
  printer.printType(getResult().getType());
}

#define GET_OP_CLASSES
#include "unions/Ops.cpp.inc"
