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
class PointerType;
class RefType;
class PointerType : public ::mlir::Type::TypeBase<PointerType, ::mlir::Type, ::mlir::TypeStorage, ::cir::PointerLikeTypeInterface::Trait> {
public:
  using Base::Base;
  static constexpr ::llvm::StringLiteral name = "cir.ptr";
  static constexpr ::llvm::StringLiteral dialectName = "cir";
  static constexpr ::llvm::StringLiteral getMnemonic() {
    return {"ptr"};
  }

};
namespace detail {
struct RefTypeStorage;
} // namespace detail
class RefType : public ::mlir::Type::TypeBase<RefType, ::mlir::Type, detail::RefTypeStorage, ::cir::PointerLikeTypeInterface::Trait> {
public:
  using Base::Base;
  static constexpr ::llvm::StringLiteral name = "cir.ref";
  static constexpr ::llvm::StringLiteral dialectName = "cir";
  static RefType get(::mlir::MLIRContext *context, mlir::Type pointeeType);
  static constexpr ::llvm::StringLiteral getMnemonic() {
    return {"ref"};
  }

  static ::mlir::Type parse(::mlir::AsmParser &odsParser);
  void print(::mlir::AsmPrinter &odsPrinter) const;
  mlir::Type getPointeeType() const;
};
} // namespace cir
MLIR_DECLARE_EXPLICIT_TYPE_ID(::cir::PointerType)
MLIR_DECLARE_EXPLICIT_TYPE_ID(::cir::RefType)

#endif  // GET_TYPEDEF_CLASSES

