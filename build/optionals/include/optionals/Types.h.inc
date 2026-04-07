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
class OptionalType;
namespace detail {
struct OptionalTypeStorage;
} // namespace detail
class OptionalType : public ::mlir::Type::TypeBase<OptionalType, ::mlir::Type, detail::OptionalTypeStorage> {
public:
  using Base::Base;
  /// Returns true if the payload type is pointer-like (implements
  /// PointerLikeTypeInterface), enabling null-pointer optimization.
  bool isPointerLike() const;
  static constexpr ::llvm::StringLiteral name = "cir.optional";
  static constexpr ::llvm::StringLiteral dialectName = "cir";
  static OptionalType get(::mlir::MLIRContext *context, ::mlir::Type payloadType);
  static constexpr ::llvm::StringLiteral getMnemonic() {
    return {"optional"};
  }

  static ::mlir::Type parse(::mlir::AsmParser &odsParser);
  void print(::mlir::AsmPrinter &odsPrinter) const;
  ::mlir::Type getPayloadType() const;
};
} // namespace cir
MLIR_DECLARE_EXPLICIT_TYPE_ID(::cir::OptionalType)

#endif  // GET_TYPEDEF_CLASSES

