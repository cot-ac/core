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
class ArrayType;
namespace detail {
struct ArrayTypeStorage;
} // namespace detail
class ArrayType : public ::mlir::Type::TypeBase<ArrayType, ::mlir::Type, detail::ArrayTypeStorage> {
public:
  using Base::Base;
  static constexpr ::llvm::StringLiteral name = "cir.array";
  static constexpr ::llvm::StringLiteral dialectName = "cir";
  static ArrayType get(::mlir::MLIRContext *context, int64_t size, ::mlir::Type elementType);
  static constexpr ::llvm::StringLiteral getMnemonic() {
    return {"array"};
  }

  static ::mlir::Type parse(::mlir::AsmParser &odsParser);
  void print(::mlir::AsmPrinter &odsPrinter) const;
  int64_t getSize() const;
  ::mlir::Type getElementType() const;
};
} // namespace cir
MLIR_DECLARE_EXPLICIT_TYPE_ID(::cir::ArrayType)

#endif  // GET_TYPEDEF_CLASSES

