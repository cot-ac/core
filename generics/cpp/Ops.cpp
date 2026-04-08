//===- Ops.cpp - generics op implementations ------------------*- C++ -*-===//
#include "generics/Ops.h"

#include "mlir/IR/Builders.h"
#include "mlir/IR/OpImplementation.h"
#include "mlir/IR/SymbolTable.h"
#include "mlir/Dialect/Func/IR/FuncOps.h"

using namespace mlir;

//===----------------------------------------------------------------------===//
// GenericApplyOp — custom assembly format
//===----------------------------------------------------------------------===//
//
// %r = cir.generic_apply @callee(%a, %b) subs ["T" = i32] : (i32, i32) -> i32
//

ParseResult cir::GenericApplyOp::parse(OpAsmParser &parser,
                                        OperationState &result) {
  FlatSymbolRefAttr calleeAttr;
  SmallVector<OpAsmParser::UnresolvedOperand> args;
  SmallVector<Type> argTypes, resultTypes;

  // Parse @callee
  if (parser.parseAttribute(calleeAttr, "callee", result.attributes))
    return failure();

  // Parse (args)
  if (parser.parseLParen())
    return failure();
  if (parser.parseOptionalRParen()) {
    // Non-empty arg list
    do {
      OpAsmParser::UnresolvedOperand arg;
      if (parser.parseOperand(arg))
        return failure();
      args.push_back(arg);
    } while (succeeded(parser.parseOptionalComma()));
    if (parser.parseRParen())
      return failure();
  }

  // Parse subs ["key" = type, ...]
  SmallVector<Attribute> keys, types;
  if (parser.parseKeyword("subs") || parser.parseLSquare())
    return failure();
  if (parser.parseOptionalRSquare()) {
    do {
      std::string key;
      Type ty;
      if (parser.parseString(&key) || parser.parseEqual() ||
          parser.parseType(ty))
        return failure();
      keys.push_back(StringAttr::get(parser.getContext(), key));
      types.push_back(TypeAttr::get(ty));
    } while (succeeded(parser.parseOptionalComma()));
    if (parser.parseRSquare())
      return failure();
  }
  result.addAttribute("sub_keys",
                       ArrayAttr::get(parser.getContext(), keys));
  result.addAttribute("sub_types",
                       ArrayAttr::get(parser.getContext(), types));

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

  // Result types — could be single type or tuple
  if (succeeded(parser.parseOptionalLParen())) {
    // Empty result: ()
    if (parser.parseRParen())
      return failure();
  } else {
    Type rty;
    if (parser.parseType(rty))
      return failure();
    resultTypes.push_back(rty);
  }

  // Resolve operands
  if (parser.resolveOperands(args, argTypes, parser.getCurrentLocation(),
                              result.operands))
    return failure();
  result.addTypes(resultTypes);

  return success();
}

void cir::GenericApplyOp::print(OpAsmPrinter &printer) {
  printer << " " << getCalleeAttr();

  // (args)
  printer << "(";
  printer.printOperands(getArgs());
  printer << ")";

  // subs ["key" = type, ...]
  printer << " subs [";
  auto keys = getSubKeys();
  auto types = getSubTypes();
  for (unsigned i = 0; i < keys.size(); i++) {
    if (i > 0) printer << ", ";
    printer << "\"" << mlir::cast<StringAttr>(keys[i]).getValue() << "\" = ";
    printer.printType(mlir::cast<TypeAttr>(types[i]).getValue());
  }
  printer << "]";

  // : (argTypes) -> resultTypes
  printer << " : (";
  llvm::interleaveComma(getArgs().getTypes(), printer,
                         [&](Type t) { printer.printType(t); });
  printer << ") -> ";
  if (getNumResults() == 0) {
    printer << "()";
  } else {
    printer.printType(getResult(0).getType());
  }
}

//===----------------------------------------------------------------------===//
// SymbolUserOpInterface — verify callee exists
//===----------------------------------------------------------------------===//

LogicalResult cir::GenericApplyOp::verifySymbolUses(
    SymbolTableCollection &symbolTable) {
  // Verify the callee symbol exists
  auto callee = (*this)->getAttrOfType<FlatSymbolRefAttr>("callee");
  if (!callee)
    return emitOpError("requires a 'callee' symbol reference attribute");

  auto fn = symbolTable.lookupNearestSymbolFrom<func::FuncOp>(
      *this, callee);
  if (!fn)
    return emitOpError() << "'" << callee.getValue()
                         << "' does not reference a valid function";
  return success();
}

#define GET_OP_CLASSES
#include "generics/Ops.cpp.inc"
