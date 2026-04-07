//===- Types.cpp - cot-structs type implementations ----------*- C++ -*-===//
#include "structs/Types.h"
#include "cot/CIR/CIRDialect.h"

#include "mlir/IR/Builders.h"
#include "mlir/IR/DialectImplementation.h"
#include "llvm/ADT/TypeSwitch.h"

using namespace mlir;

#define GET_TYPEDEF_CLASSES
#include "structs/Types.cpp.inc"

//===----------------------------------------------------------------------===//
// StructType custom assembly format
// Syntax: !cir.struct<"Name", "field1": type1, "field2": type2>
//===----------------------------------------------------------------------===//

Type cir::StructType::parse(AsmParser &parser) {
  if (parser.parseLess())
    return {};

  // Parse struct name
  std::string name;
  if (parser.parseString(&name))
    return {};

  SmallVector<StringAttr> fieldNames;
  SmallVector<Type> fieldTypes;

  // Parse comma-separated "fieldName": fieldType pairs
  while (succeeded(parser.parseOptionalComma())) {
    std::string fieldName;
    Type fieldType;
    if (parser.parseString(&fieldName) || parser.parseColon() ||
        parser.parseType(fieldType))
      return {};
    fieldNames.push_back(StringAttr::get(parser.getContext(), fieldName));
    fieldTypes.push_back(fieldType);
  }

  if (parser.parseGreater())
    return {};

  return get(parser.getContext(),
             StringAttr::get(parser.getContext(), name),
             fieldNames, fieldTypes);
}

void cir::StructType::print(AsmPrinter &printer) const {
  printer << "<\"" << getName().getValue() << "\"";
  auto names = getFieldNames();
  auto types = getFieldTypes();
  for (unsigned i = 0; i < names.size(); i++) {
    printer << ", \"" << names[i].getValue() << "\": ";
    printer.printType(types[i]);
  }
  printer << ">";
}

void cir::registerStructsTypes(cir::CIRDialect *dialect) {
  dialect->registerConstructTypes<cir::StructType>();
}
