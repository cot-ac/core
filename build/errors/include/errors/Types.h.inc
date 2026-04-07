/*===- TableGen'erated file -------------------------------------*- C++ -*-===*\
|*                                                                            *|
|* TypeDef Declarations                                                       *|
|*                                                                            *|
|* Automatically generated file, do not edit!                                 *|
|*                                                                            *|
\*===----------------------------------------------------------------------===*/

#ifdef GET_TYPEDEF_CLASSES
#undef GET_TYPEDEF_CLASSES


namespace mlir {
class AsmParser;
class AsmPrinter;
} // namespace mlir
namespace cir {
class ErrorUnionType;
namespace detail {
struct ErrorUnionTypeStorage;
} // namespace detail
class ErrorUnionType : public ::mlir::Type::TypeBase<ErrorUnionType, ::mlir::Type, detail::ErrorUnionTypeStorage> {
public:
  using Base::Base;
  static constexpr ::llvm::StringLiteral name = "cir.error_union";
  static constexpr ::llvm::StringLiteral dialectName = "cir";
  static ErrorUnionType get(::mlir::MLIRContext *context, ::mlir::Type payloadType);
  static constexpr ::llvm::StringLiteral getMnemonic() {
    return {"error_union"};
  }

  static ::mlir::Type parse(::mlir::AsmParser &odsParser);
  void print(::mlir::AsmPrinter &odsPrinter) const;
  ::mlir::Type getPayloadType() const;
};
} // namespace cir
MLIR_DECLARE_EXPLICIT_TYPE_ID(::cir::ErrorUnionType)

#endif  // GET_TYPEDEF_CLASSES

