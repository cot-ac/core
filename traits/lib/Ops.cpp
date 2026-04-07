//===- Ops.cpp - traits op implementations --------------------*- C++ -*-===//
#include "traits/Ops.h"

#include "mlir/IR/Builders.h"
#include "mlir/IR/OpImplementation.h"
#include "mlir/IR/SymbolTable.h"
#include "mlir/Dialect/Func/IR/FuncOps.h"

using namespace mlir;

//===----------------------------------------------------------------------===//
// WitnessTableOp — custom assembly format
//===----------------------------------------------------------------------===//
//
// cir.witness_table @Summable_Point {protocol = "Summable", type = "Point"}
//     ["sum" = @Point_sum, "zero" = @Point_zero]
//

ParseResult cir::WitnessTableOp::parse(OpAsmParser &parser,
                                         OperationState &result) {
  StringAttr nameAttr;
  if (parser.parseSymbolName(nameAttr, SymbolTable::getSymbolAttrName(),
                              result.attributes))
    return failure();

  // Parse ["method" = @impl, ...]
  SmallVector<Attribute> names, impls;
  if (parser.parseLSquare())
    return failure();
  if (parser.parseOptionalRSquare()) {
    do {
      std::string methodName;
      FlatSymbolRefAttr implRef;
      if (parser.parseString(&methodName) || parser.parseEqual() ||
          parser.parseAttribute(implRef))
        return failure();
      names.push_back(StringAttr::get(parser.getContext(), methodName));
      impls.push_back(implRef);
    } while (succeeded(parser.parseOptionalComma()));
    if (parser.parseRSquare())
      return failure();
  }

  result.addAttribute("method_names",
                       ArrayAttr::get(parser.getContext(), names));
  result.addAttribute("method_impls",
                       ArrayAttr::get(parser.getContext(), impls));

  // Parse optional attributes (protocol, conforming_type)
  if (parser.parseOptionalAttrDict(result.attributes))
    return failure();

  return success();
}

void cir::WitnessTableOp::print(OpAsmPrinter &printer) {
  printer << " ";
  printer.printSymbolName(getSymName());
  printer << " [";
  auto names = getMethodNames();
  auto impls = getMethodImpls();
  for (unsigned i = 0; i < names.size(); i++) {
    if (i > 0) printer << ", ";
    printer << "\"" << mlir::cast<StringAttr>(names[i]).getValue() << "\" = ";
    printer.printAttribute(impls[i]);
  }
  printer << "]";
  SmallVector<StringRef> elidedAttrs = {
      SymbolTable::getSymbolAttrName(), "method_names", "method_impls"};
  printer.printOptionalAttrDict((*this)->getAttrs(), elidedAttrs);
}

//===----------------------------------------------------------------------===//
// TraitCallOp — custom assembly format
//===----------------------------------------------------------------------===//
//
// %r = cir.trait_call @Summable::sum(%val) : (i32) -> i32
//

ParseResult cir::TraitCallOp::parse(OpAsmParser &parser,
                                     OperationState &result) {
  FlatSymbolRefAttr protocolAttr;
  if (parser.parseAttribute(protocolAttr, "protocol", result.attributes))
    return failure();

  // Parse ::method
  if (parser.parseColon() || parser.parseColon())
    return failure();
  std::string methodName;
  if (parser.parseKeywordOrString(&methodName))
    return failure();
  result.addAttribute("method",
                       StringAttr::get(parser.getContext(), methodName));

  // Parse (args)
  SmallVector<OpAsmParser::UnresolvedOperand> args;
  SmallVector<Type> argTypes, resultTypes;
  if (parser.parseLParen())
    return failure();
  if (parser.parseOptionalRParen()) {
    do {
      OpAsmParser::UnresolvedOperand arg;
      if (parser.parseOperand(arg))
        return failure();
      args.push_back(arg);
    } while (succeeded(parser.parseOptionalComma()));
    if (parser.parseRParen())
      return failure();
  }

  // Parse : (argTypes) -> resultTypes
  if (parser.parseColon() || parser.parseLParen())
    return failure();
  if (parser.parseOptionalRParen()) {
    do {
      Type ty;
      if (parser.parseType(ty))
        return failure();
      argTypes.push_back(ty);
    } while (succeeded(parser.parseOptionalComma()));
    if (parser.parseRParen())
      return failure();
  }
  if (parser.parseArrow())
    return failure();
  Type rty;
  if (parser.parseType(rty))
    return failure();
  resultTypes.push_back(rty);

  if (parser.resolveOperands(args, argTypes, parser.getCurrentLocation(),
                              result.operands))
    return failure();
  result.addTypes(resultTypes);
  return success();
}

void cir::TraitCallOp::print(OpAsmPrinter &printer) {
  printer << " " << getProtocolAttr() << "::" << getMethod() << "(";
  printer.printOperands(getArgs());
  printer << ") : (";
  llvm::interleaveComma(getArgs().getTypes(), printer,
                         [&](Type t) { printer.printType(t); });
  printer << ") -> ";
  printer.printType(getResult(0).getType());
}

LogicalResult cir::TraitCallOp::verifySymbolUses(
    SymbolTableCollection &symbolTable) {
  // The protocol reference is verified at resolution time
  return success();
}

//===----------------------------------------------------------------------===//
// WitnessMethodOp — custom assembly format
//===----------------------------------------------------------------------===//
//
// %fn = cir.witness_method %pwt, "sum" : (!cir.ptr) -> !cir.ptr
//

ParseResult cir::WitnessMethodOp::parse(OpAsmParser &parser,
                                          OperationState &result) {
  OpAsmParser::UnresolvedOperand pwt;
  Type pwtType, resultType;
  std::string methodName;

  if (parser.parseOperand(pwt) || parser.parseComma() ||
      parser.parseString(&methodName))
    return failure();
  result.addAttribute("method",
                       StringAttr::get(parser.getContext(), methodName));

  if (parser.parseColon() || parser.parseLParen() ||
      parser.parseType(pwtType) || parser.parseRParen() ||
      parser.parseArrow() || parser.parseType(resultType))
    return failure();

  if (parser.resolveOperand(pwt, pwtType, result.operands))
    return failure();
  result.addTypes(resultType);
  return success();
}

void cir::WitnessMethodOp::print(OpAsmPrinter &printer) {
  printer << " " << getPwt() << ", \"" << getMethod() << "\" : (";
  printer.printType(getPwt().getType());
  printer << ") -> ";
  printer.printType(getResult().getType());
}

//===----------------------------------------------------------------------===//
// OpenExistentialOp — custom assembly format
//===----------------------------------------------------------------------===//
//
// %buf, %vwt, %pwt = cir.open_existential %container
//     : !cir.existential<"Summable"> -> (!cir.ptr, !cir.ptr, !cir.ptr)
//

ParseResult cir::OpenExistentialOp::parse(OpAsmParser &parser,
                                            OperationState &result) {
  OpAsmParser::UnresolvedOperand container;
  Type containerType;
  SmallVector<Type> resultTypes;

  if (parser.parseOperand(container) || parser.parseColon() ||
      parser.parseType(containerType) || parser.parseArrow() ||
      parser.parseLParen())
    return failure();

  do {
    Type ty;
    if (parser.parseType(ty))
      return failure();
    resultTypes.push_back(ty);
  } while (succeeded(parser.parseOptionalComma()));

  if (parser.parseRParen())
    return failure();

  if (parser.resolveOperand(container, containerType, result.operands))
    return failure();
  result.addTypes(resultTypes);
  return success();
}

void cir::OpenExistentialOp::print(OpAsmPrinter &printer) {
  printer << " " << getContainer() << " : ";
  printer.printType(getContainer().getType());
  printer << " -> (";
  llvm::interleaveComma(getResults().getTypes(), printer,
                         [&](Type t) { printer.printType(t); });
  printer << ")";
}

#define GET_OP_CLASSES
#include "traits/Ops.cpp.inc"
