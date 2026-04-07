//===- Ops.cpp - cot-arrays op implementations ---------------*- C++ -*-===//
#include "arrays/Ops.h"

#include "mlir/IR/Builders.h"
#include "mlir/IR/OpImplementation.h"

using namespace mlir;

//===----------------------------------------------------------------------===//
// ArrayInitOp custom assembly format
// Syntax: cir.array_init(%a, %b, %c) : !cir.array<3 x i32>
//===----------------------------------------------------------------------===//

ParseResult cir::ArrayInitOp::parse(OpAsmParser &parser,
                                    OperationState &result) {
  SmallVector<OpAsmParser::UnresolvedOperand> elems;
  Type resultType;

  // Parse: (%elem1, %elem2, ...)
  if (parser.parseLParen())
    return failure();
  if (failed(parser.parseOptionalRParen())) {
    do {
      elems.emplace_back();
      if (parser.parseOperand(elems.back()))
        return failure();
    } while (succeeded(parser.parseOptionalComma()));
    if (parser.parseRParen())
      return failure();
  }

  // Parse: attr-dict : !cir.array<N x T>
  if (parser.parseOptionalAttrDict(result.attributes) ||
      parser.parseColonType(resultType))
    return failure();

  result.addTypes(resultType);

  // Resolve operand types from the array element type
  auto arrayType = mlir::cast<cir::ArrayType>(resultType);
  auto elemType = arrayType.getElementType();
  for (auto &elem : elems) {
    if (parser.resolveOperand(elem, elemType, result.operands))
      return failure();
  }

  return success();
}

void cir::ArrayInitOp::print(OpAsmPrinter &printer) {
  printer << "(";
  printer.printOperands(getElements());
  printer << ")";
  printer.printOptionalAttrDict((*this)->getAttrs());
  printer << " : " << getResult().getType();
}

#define GET_OP_CLASSES
#include "arrays/Ops.cpp.inc"
