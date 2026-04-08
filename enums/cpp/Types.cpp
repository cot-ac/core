//===- Types.cpp - enums type implementations -----------------*- C++ -*-===//
#include "enums/Types.h"
#include "cot/CIR/CIRDialect.h"

#include "mlir/IR/Builders.h"
#include "mlir/IR/DialectImplementation.h"
#include "llvm/ADT/TypeSwitch.h"

using namespace mlir;

#define GET_TYPEDEF_CLASSES
#include "enums/Types.cpp.inc"

// Custom assembly format for EnumType:
//   !cir.enum<"Color", i32, "Red", "Green", "Blue">
Type cir::EnumType::parse(AsmParser &parser) {
  if (parser.parseLess())
    return {};
  std::string name;
  if (parser.parseString(&name) || parser.parseComma())
    return {};
  Type tagType;
  if (parser.parseType(tagType))
    return {};
  SmallVector<StringAttr> variants;
  while (succeeded(parser.parseOptionalComma())) {
    std::string variant;
    if (parser.parseString(&variant))
      return {};
    variants.push_back(StringAttr::get(parser.getContext(), variant));
  }
  if (parser.parseGreater())
    return {};
  return EnumType::get(parser.getContext(), StringRef(name), tagType, variants);
}

void cir::EnumType::print(AsmPrinter &printer) const {
  printer << "<\"" << getName() << "\", ";
  printer.printType(getTagType());
  for (auto v : getVariants())
    printer << ", \"" << v.getValue() << "\"";
  printer << ">";
}

void cir::registerEnumsTypes(cir::CIRDialect *dialect) {
  dialect->registerConstructTypes<cir::EnumType>();
}
