/*===- TableGen'erated file -------------------------------------*- C++ -*-===*\
|*                                                                            *|
|* TypeDef Definitions                                                        *|
|*                                                                            *|
|* Automatically generated file, do not edit!                                 *|
|*                                                                            *|
\*===----------------------------------------------------------------------===*/

#ifdef GET_TYPEDEF_LIST
#undef GET_TYPEDEF_LIST

::cir::StructType

#endif  // GET_TYPEDEF_LIST

#ifdef GET_TYPEDEF_CLASSES
#undef GET_TYPEDEF_CLASSES

static ::mlir::OptionalParseResult generatedTypeParser(::mlir::AsmParser &parser, ::llvm::StringRef *mnemonic, ::mlir::Type &value) {
  return ::mlir::AsmParser::KeywordSwitch<::mlir::OptionalParseResult>(parser)
    .Case(::cir::StructType::getMnemonic(), [&](llvm::StringRef, llvm::SMLoc) {
      value = ::cir::StructType::parse(parser);
      return ::mlir::success(!!value);
    })
    .Default([&](llvm::StringRef keyword, llvm::SMLoc) {
      *mnemonic = keyword;
      return std::nullopt;
    });
}

static ::llvm::LogicalResult generatedTypePrinter(::mlir::Type def, ::mlir::AsmPrinter &printer) {
  return ::llvm::TypeSwitch<::mlir::Type, ::llvm::LogicalResult>(def)    .Case<::cir::StructType>([&](auto t) {
      printer << ::cir::StructType::getMnemonic();
t.print(printer);
      return ::mlir::success();
    })
    .Default([](auto) { return ::mlir::failure(); });
}

namespace cir {
namespace detail {
struct StructTypeStorage : public ::mlir::TypeStorage {
  using KeyTy = std::tuple<::mlir::StringAttr, ::llvm::ArrayRef<::mlir::StringAttr>, ::llvm::ArrayRef<::mlir::Type>>;
  StructTypeStorage(::mlir::StringAttr name, ::llvm::ArrayRef<::mlir::StringAttr> fieldNames, ::llvm::ArrayRef<::mlir::Type> fieldTypes) : name(std::move(name)), fieldNames(std::move(fieldNames)), fieldTypes(std::move(fieldTypes)) {}

  KeyTy getAsKey() const {
    return KeyTy(name, fieldNames, fieldTypes);
  }

  bool operator==(const KeyTy &tblgenKey) const {
    return (name == std::get<0>(tblgenKey)) && (fieldNames == std::get<1>(tblgenKey)) && (fieldTypes == std::get<2>(tblgenKey));
  }

  static ::llvm::hash_code hashKey(const KeyTy &tblgenKey) {
    return ::llvm::hash_combine(std::get<0>(tblgenKey), std::get<1>(tblgenKey), std::get<2>(tblgenKey));
  }

  static StructTypeStorage *construct(::mlir::TypeStorageAllocator &allocator, KeyTy &&tblgenKey) {
    auto name = std::move(std::get<0>(tblgenKey));
    auto fieldNames = std::move(std::get<1>(tblgenKey));
    auto fieldTypes = std::move(std::get<2>(tblgenKey));
    fieldNames = allocator.copyInto(fieldNames);
    fieldTypes = allocator.copyInto(fieldTypes);
    return new (allocator.allocate<StructTypeStorage>()) StructTypeStorage(std::move(name), std::move(fieldNames), std::move(fieldTypes));
  }

  ::mlir::StringAttr name;
  ::llvm::ArrayRef<::mlir::StringAttr> fieldNames;
  ::llvm::ArrayRef<::mlir::Type> fieldTypes;
};
} // namespace detail
StructType StructType::get(::mlir::MLIRContext *context, ::mlir::StringAttr name, ::llvm::ArrayRef<::mlir::StringAttr> fieldNames, ::llvm::ArrayRef<::mlir::Type> fieldTypes) {
  return Base::get(context, std::move(name), std::move(fieldNames), std::move(fieldTypes));
}

::mlir::StringAttr StructType::getName() const {
  return getImpl()->name;
}

::llvm::ArrayRef<::mlir::StringAttr> StructType::getFieldNames() const {
  return getImpl()->fieldNames;
}

::llvm::ArrayRef<::mlir::Type> StructType::getFieldTypes() const {
  return getImpl()->fieldTypes;
}

} // namespace cir
MLIR_DEFINE_EXPLICIT_TYPE_ID(::cir::StructType)

#endif  // GET_TYPEDEF_CLASSES

