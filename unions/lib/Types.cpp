//===- Types.cpp - unions type implementations ----------------*- C++ -*-===//
#include "unions/Types.h"
#include "cot/CIR/CIRDialect.h"

#include "mlir/IR/Builders.h"
#include "mlir/IR/DialectImplementation.h"
#include "llvm/ADT/TypeSwitch.h"

using namespace mlir;

#define GET_TYPEDEF_CLASSES
#include "unions/Types.cpp.inc"

// Custom assembly format for TaggedUnionType:
//   !cir.tagged_union<"Shape", "Circle": f64, "Rect": i64>
Type cir::TaggedUnionType::parse(AsmParser &parser) {
  if (parser.parseLess())
    return {};
  std::string name;
  if (parser.parseString(&name))
    return {};
  SmallVector<StringAttr> variantNames;
  SmallVector<Type> variantTypes;
  while (succeeded(parser.parseOptionalComma())) {
    std::string vname;
    if (parser.parseString(&vname))
      return {};
    if (parser.parseColon())
      return {};
    Type vtype;
    if (parser.parseType(vtype))
      return {};
    variantNames.push_back(StringAttr::get(parser.getContext(), vname));
    variantTypes.push_back(vtype);
  }
  if (parser.parseGreater())
    return {};
  return TaggedUnionType::get(parser.getContext(), StringRef(name),
                               variantNames, variantTypes);
}

void cir::TaggedUnionType::print(AsmPrinter &printer) const {
  printer << "<\"" << getName() << "\"";
  auto names = getVariantNames();
  auto types = getVariantTypes();
  for (unsigned i = 0; i < names.size(); i++) {
    printer << ", \"" << names[i].getValue() << "\": ";
    printer.printType(types[i]);
  }
  printer << ">";
}

unsigned cir::TaggedUnionType::getMaxPayloadBitWidth() const {
  unsigned maxBits = 0;
  for (auto t : getVariantTypes()) {
    if (auto intTy = mlir::dyn_cast<IntegerType>(t))
      maxBits = std::max(maxBits, intTy.getWidth());
    else if (auto floatTy = mlir::dyn_cast<FloatType>(t))
      maxBits = std::max(maxBits, floatTy.getWidth());
    else
      maxBits = std::max(maxBits, 64u); // default for complex types
  }
  return maxBits;
}

int64_t cir::TaggedUnionType::getVariantIndex(llvm::StringRef name) const {
  auto names = getVariantNames();
  for (unsigned i = 0; i < names.size(); i++) {
    if (names[i].getValue() == name)
      return i;
  }
  return -1;
}

Type cir::TaggedUnionType::getVariantType(llvm::StringRef name) const {
  auto idx = getVariantIndex(name);
  if (idx < 0) return {};
  return getVariantTypes()[idx];
}

void cir::registerUnionsTypes(cir::CIRDialect *dialect) {
  dialect->registerConstructTypes<cir::TaggedUnionType>();
}
