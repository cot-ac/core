/*===- TableGen'erated file -------------------------------------*- C++ -*-===*\
|*                                                                            *|
|* TypeDef Definitions                                                        *|
|*                                                                            *|
|* Automatically generated file, do not edit!                                 *|
|*                                                                            *|
\*===----------------------------------------------------------------------===*/

#ifdef GET_TYPEDEF_LIST
#undef GET_TYPEDEF_LIST

::cir::PointerType,
::cir::RefType

#endif  // GET_TYPEDEF_LIST

#ifdef GET_TYPEDEF_CLASSES
#undef GET_TYPEDEF_CLASSES

static ::mlir::OptionalParseResult generatedTypeParser(::mlir::AsmParser &parser, ::llvm::StringRef *mnemonic, ::mlir::Type &value) {
  return ::mlir::AsmParser::KeywordSwitch<::mlir::OptionalParseResult>(parser)
    .Case(::cir::PointerType::getMnemonic(), [&](llvm::StringRef, llvm::SMLoc) {
      value = ::cir::PointerType::get(parser.getContext());
      return ::mlir::success(!!value);
    })
    .Case(::cir::RefType::getMnemonic(), [&](llvm::StringRef, llvm::SMLoc) {
      value = ::cir::RefType::parse(parser);
      return ::mlir::success(!!value);
    })
    .Default([&](llvm::StringRef keyword, llvm::SMLoc) {
      *mnemonic = keyword;
      return std::nullopt;
    });
}

static ::llvm::LogicalResult generatedTypePrinter(::mlir::Type def, ::mlir::AsmPrinter &printer) {
  return ::llvm::TypeSwitch<::mlir::Type, ::llvm::LogicalResult>(def)    .Case<::cir::PointerType>([&](auto t) {
      printer << ::cir::PointerType::getMnemonic();
      return ::mlir::success();
    })
    .Case<::cir::RefType>([&](auto t) {
      printer << ::cir::RefType::getMnemonic();
t.print(printer);
      return ::mlir::success();
    })
    .Default([](auto) { return ::mlir::failure(); });
}

namespace cir {
} // namespace cir
MLIR_DEFINE_EXPLICIT_TYPE_ID(::cir::PointerType)
namespace cir {
namespace detail {
struct RefTypeStorage : public ::mlir::TypeStorage {
  using KeyTy = std::tuple<mlir::Type>;
  RefTypeStorage(mlir::Type pointeeType) : pointeeType(std::move(pointeeType)) {}

  KeyTy getAsKey() const {
    return KeyTy(pointeeType);
  }

  bool operator==(const KeyTy &tblgenKey) const {
    return (pointeeType == std::get<0>(tblgenKey));
  }

  static ::llvm::hash_code hashKey(const KeyTy &tblgenKey) {
    return ::llvm::hash_combine(std::get<0>(tblgenKey));
  }

  static RefTypeStorage *construct(::mlir::TypeStorageAllocator &allocator, KeyTy &&tblgenKey) {
    auto pointeeType = std::move(std::get<0>(tblgenKey));
    return new (allocator.allocate<RefTypeStorage>()) RefTypeStorage(std::move(pointeeType));
  }

  mlir::Type pointeeType;
};
} // namespace detail
RefType RefType::get(::mlir::MLIRContext *context, mlir::Type pointeeType) {
  return Base::get(context, std::move(pointeeType));
}

::mlir::Type RefType::parse(::mlir::AsmParser &odsParser) {
  ::mlir::Builder odsBuilder(odsParser.getContext());
  ::llvm::SMLoc odsLoc = odsParser.getCurrentLocation();
  (void) odsLoc;
  ::mlir::FailureOr<mlir::Type> _result_pointeeType;
  // Parse literal '<'
  if (odsParser.parseLess()) return {};

  // Parse variable 'pointeeType'
  _result_pointeeType = ::mlir::FieldParser<mlir::Type>::parse(odsParser);
  if (::mlir::failed(_result_pointeeType)) {
    odsParser.emitError(odsParser.getCurrentLocation(), "failed to parse CIR_RefType parameter 'pointeeType' which is to be a `mlir::Type`");
    return {};
  }
  // Parse literal '>'
  if (odsParser.parseGreater()) return {};
  assert(::mlir::succeeded(_result_pointeeType));
  return RefType::get(odsParser.getContext(),
      mlir::Type((*_result_pointeeType)));
}

void RefType::print(::mlir::AsmPrinter &odsPrinter) const {
  ::mlir::Builder odsBuilder(getContext());
  odsPrinter << "<";
  odsPrinter.printStrippedAttrOrType(getPointeeType());
  odsPrinter << ">";
}

mlir::Type RefType::getPointeeType() const {
  return getImpl()->pointeeType;
}

} // namespace cir
MLIR_DEFINE_EXPLICIT_TYPE_ID(::cir::RefType)

#endif  // GET_TYPEDEF_CLASSES

