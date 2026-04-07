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
class SliceType;
namespace detail {
struct SliceTypeStorage;
} // namespace detail
class SliceType : public ::mlir::Type::TypeBase<SliceType, ::mlir::Type, detail::SliceTypeStorage> {
public:
  using Base::Base;
  static constexpr ::llvm::StringLiteral name = "cir.slice";
  static constexpr ::llvm::StringLiteral dialectName = "cir";
  static SliceType get(::mlir::MLIRContext *context, ::mlir::Type elementType);
  static constexpr ::llvm::StringLiteral getMnemonic() {
    return {"slice"};
  }

  static ::mlir::Type parse(::mlir::AsmParser &odsParser);
  void print(::mlir::AsmPrinter &odsPrinter) const;
  ::mlir::Type getElementType() const;
};
} // namespace cir
MLIR_DECLARE_EXPLICIT_TYPE_ID(::cir::SliceType)

#endif  // GET_TYPEDEF_CLASSES

