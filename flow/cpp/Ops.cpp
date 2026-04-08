//===- Ops.cpp - cot-flow op implementations -----------------*- C++ -*-===//
#include "flow/Ops.h"

#include "mlir/IR/Builders.h"
#include "mlir/IR/OpImplementation.h"

using namespace mlir;

//===----------------------------------------------------------------------===//
// BrOp — BranchOpInterface
//===----------------------------------------------------------------------===//

SuccessorOperands cir::BrOp::getSuccessorOperands(unsigned index) {
  assert(index == 0 && "invalid successor index");
  return SuccessorOperands(getDestOperandsMutable());
}

//===----------------------------------------------------------------------===//
// CondBrOp — BranchOpInterface
//===----------------------------------------------------------------------===//

SuccessorOperands cir::CondBrOp::getSuccessorOperands(unsigned index) {
  assert(index < 2 && "invalid successor index");
  if (index == 0)
    return SuccessorOperands(getTrueDestOperandsMutable());
  return SuccessorOperands(getFalseDestOperandsMutable());
}

//===----------------------------------------------------------------------===//
// SwitchOp — BranchOpInterface + custom parse/print + verifier
//===----------------------------------------------------------------------===//

SuccessorOperands cir::SwitchOp::getSuccessorOperands(unsigned index) {
  // No operands forwarded through switch branches
  return SuccessorOperands(MutableOperandRange(getOperation(), 0, 0));
}

ParseResult cir::SwitchOp::parse(OpAsmParser &parser,
                                  OperationState &result) {
  OpAsmParser::UnresolvedOperand value;
  Type valueType;

  // Parse: value `:` type
  if (parser.parseOperand(value) || parser.parseColonType(valueType) ||
      parser.resolveOperand(value, valueType, result.operands))
    return failure();

  // Parse: `,`
  if (parser.parseComma())
    return failure();

  // Parse default destination
  Block *defaultDest;
  if (parser.parseSuccessor(defaultDest))
    return failure();
  result.addSuccessors(defaultDest);

  // Parse optional case list: `[` case_val `:` dest `,` ... `]`
  SmallVector<int64_t> caseValues;
  SmallVector<Block *> caseDests;

  if (!parser.parseOptionalLSquare()) {
    // Parse cases until `]`
    do {
      int64_t caseVal;
      Block *caseDest;
      if (parser.parseInteger(caseVal) || parser.parseColon() ||
          parser.parseSuccessor(caseDest))
        return failure();
      caseValues.push_back(caseVal);
      caseDests.push_back(caseDest);
    } while (succeeded(parser.parseOptionalComma()));

    if (parser.parseRSquare())
      return failure();
  }

  for (auto *dest : caseDests)
    result.addSuccessors(dest);

  result.addAttribute("case_values",
                       DenseI64ArrayAttr::get(parser.getContext(),
                                              caseValues));

  if (parser.parseOptionalAttrDict(result.attributes))
    return failure();

  return success();
}

void cir::SwitchOp::print(OpAsmPrinter &printer) {
  printer << " " << getValue() << " : " << getValue().getType() << ", ";
  printer.printSuccessor(getDefaultDest());

  auto caseVals = getCaseValues();
  auto caseDests = getCaseDests();
  if (!caseDests.empty()) {
    printer << " [";
    for (unsigned i = 0; i < caseDests.size(); ++i) {
      if (i > 0)
        printer << ", ";
      printer << caseVals[i] << " : ";
      printer.printSuccessor(caseDests[i]);
    }
    printer << "]";
  }

  printer.printOptionalAttrDict((*this)->getAttrs(), {"case_values"});
}

LogicalResult cir::SwitchOp::verify() {
  auto caseVals = getCaseValues();
  auto caseDests = getCaseDests();
  if (caseVals.size() != caseDests.size())
    return emitOpError("case_values count (")
           << caseVals.size() << ") must match case destinations count ("
           << caseDests.size() << ")";
  return success();
}

#define GET_OP_CLASSES
#include "flow/Ops.cpp.inc"
