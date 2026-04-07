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
class StructType;
namespace detail {
struct StructTypeStorage;
} // namespace detail
class StructType : public ::mlir::Type::TypeBase<StructType, ::mlir::Type, detail::StructTypeStorage> {
public:
  using Base::Base;
  static constexpr ::llvm::StringLiteral name = "cir.struct";
  static constexpr ::llvm::StringLiteral dialectName = "cir";
  static StructType get(::mlir::MLIRContext *context, ::mlir::StringAttr name, ::llvm::ArrayRef<::mlir::StringAttr> fieldNames, ::llvm::ArrayRef<::mlir::Type> fieldTypes);
  static constexpr ::llvm::StringLiteral getMnemonic() {
    return {"struct"};
  }

  static ::mlir::Type parse(::mlir::AsmParser &odsParser);
  void print(::mlir::AsmPrinter &odsPrinter) const;
  ::mlir::StringAttr getName() const;
  ::llvm::ArrayRef<::mlir::StringAttr> getFieldNames() const;
  ::llvm::ArrayRef<::mlir::Type> getFieldTypes() const;
};
} // namespace cir
MLIR_DECLARE_EXPLICIT_TYPE_ID(::cir::StructType)

#endif  // GET_TYPEDEF_CLASSES

