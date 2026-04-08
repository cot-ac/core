//===- Ops.cpp - cot-structs op implementations --------------*- C++ -*-===//
#include "structs/Ops.h"

#include "mlir/IR/Builders.h"
#include "mlir/IR/OpImplementation.h"

using namespace mlir;

//===----------------------------------------------------------------------===//
// StructInitOp custom assembly format
// Syntax: cir.struct_init(%x, %y) : !cir.struct<"Point", "x": i32, "y": i32>
//===----------------------------------------------------------------------===//

ParseResult cir::StructInitOp::parse(OpAsmParser &parser,
                                     OperationState &result) {
  SmallVector<OpAsmParser::UnresolvedOperand> fields;
  Type resultType;

  // Parse: (%field1, %field2, ...)
  if (parser.parseLParen())
    return failure();
  if (failed(parser.parseOptionalRParen())) {
    do {
      fields.emplace_back();
      if (parser.parseOperand(fields.back()))
        return failure();
    } while (succeeded(parser.parseOptionalComma()));
    if (parser.parseRParen())
      return failure();
  }

  // Parse: attr-dict : !cir.struct<...>
  if (parser.parseOptionalAttrDict(result.attributes) ||
      parser.parseColonType(resultType))
    return failure();

  result.addTypes(resultType);

  // Resolve operand types from the struct type
  auto structType = mlir::cast<cir::StructType>(resultType);
  auto fieldTypes = structType.getFieldTypes();
  for (unsigned i = 0; i < fields.size(); i++) {
    if (parser.resolveOperand(fields[i], fieldTypes[i], result.operands))
      return failure();
  }

  return success();
}

void cir::StructInitOp::print(OpAsmPrinter &printer) {
  printer << "(";
  printer.printOperands(getFields());
  printer << ")";
  printer.printOptionalAttrDict((*this)->getAttrs());
  printer << " : " << getResult().getType();
}

#define GET_OP_CLASSES
#include "structs/Ops.cpp.inc"
