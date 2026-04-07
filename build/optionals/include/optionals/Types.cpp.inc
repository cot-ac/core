/*===- TableGen'erated file -------------------------------------*- C++ -*-===*\
|*                                                                            *|
|* TypeDef Definitions                                                        *|
|*                                                                            *|
|* Automatically generated file, do not edit!                                 *|
|*                                                                            *|
\*===----------------------------------------------------------------------===*/

#ifdef GET_TYPEDEF_LIST
#undef GET_TYPEDEF_LIST

::cir::OptionalType

#endif  // GET_TYPEDEF_LIST

#ifdef GET_TYPEDEF_CLASSES
#undef GET_TYPEDEF_CLASSES

static ::mlir::OptionalParseResult generatedTypeParser(::mlir::AsmParser &parser, ::llvm::StringRef *mnemonic, ::mlir::Type &value) {
  return ::mlir::AsmParser::KeywordSwitch<::mlir::OptionalParseResult>(parser)
    .Case(::cir::OptionalType::getMnemonic(), [&](llvm::StringRef, llvm::SMLoc) {
      value = ::cir::OptionalType::parse(parser);
      return ::mlir::success(!!value);
    })
    .Default([&](llvm::StringRef keyword, llvm::SMLoc) {
      *mnemonic = keyword;
      return std::nullopt;
    });
}

static ::llvm::LogicalResult generatedTypePrinter(::mlir::Type def, ::mlir::AsmPrinter &printer) {
  return ::llvm::TypeSwitch<::mlir::Type, ::llvm::LogicalResult>(def)    .Case<::cir::OptionalType>([&](auto t) {
      printer << ::cir::OptionalType::getMnemonic();
t.print(printer);
      return ::mlir::success();
    })
    .Default([](auto) { return ::mlir::failure(); });
}

namespace cir {
namespace detail {
struct OptionalTypeStorage : public ::mlir::TypeStorage {
  using KeyTy = std::tuple<::mlir::Type>;
  OptionalTypeStorage(::mlir::Type payloadType) : payloadType(std::move(payloadType)) {}

  KeyTy getAsKey() const {
    return KeyTy(payloadType);
  }

  bool operator==(const KeyTy &tblgenKey) const {
    return (payloadType == std::get<0>(tblgenKey));
  }

  static ::llvm::hash_code hashKey(const KeyTy &tblgenKey) {
    return ::llvm::hash_combine(std::get<0>(tblgenKey));
  }

  static OptionalTypeStorage *construct(::mlir::TypeStorageAllocator &allocator, KeyTy &&tblgenKey) {
    auto payloadType = std::move(std::get<0>(tblgenKey));
    return new (allocator.allocate<OptionalTypeStorage>()) OptionalTypeStorage(std::move(payloadType));
  }

  ::mlir::Type payloadType;
};
} // namespace detail
OptionalType OptionalType::get(::mlir::MLIRContext *context, ::mlir::Type payloadType) {
  return Base::get(context, std::move(payloadType));
}

::mlir::Type OptionalType::parse(::mlir::AsmParser &odsParser) {
  ::mlir::Builder odsBuilder(odsParser.getContext());
  ::llvm::SMLoc odsLoc = odsParser.getCurrentLocation();
  (void) odsLoc;
  ::mlir::FailureOr<::mlir::Type> _result_payloadType;
  // Parse literal '<'
  if (odsParser.parseLess()) return {};

  // Parse variable 'payloadType'
  _result_payloadType = ::mlir::FieldParser<::mlir::Type>::parse(odsParser);
  if (::mlir::failed(_result_payloadType)) {
    odsParser.emitError(odsParser.getCurrentLocation(), "failed to parse CIR_OptionalType parameter 'payloadType' which is to be a `::mlir::Type`");
    return {};
  }
  // Parse literal '>'
  if (odsParser.parseGreater()) return {};
  assert(::mlir::succeeded(_result_payloadType));
  return OptionalType::get(odsParser.getContext(),
      ::mlir::Type((*_result_payloadType)));
}

void OptionalType::print(::mlir::AsmPrinter &odsPrinter) const {
  ::mlir::Builder odsBuilder(getContext());
  odsPrinter << "<";
  odsPrinter.printStrippedAttrOrType(getPayloadType());
  odsPrinter << ">";
}

::mlir::Type OptionalType::getPayloadType() const {
  return getImpl()->payloadType;
}

} // namespace cir
MLIR_DEFINE_EXPLICIT_TYPE_ID(::cir::OptionalType)

#endif  // GET_TYPEDEF_CLASSES

