/*===- TableGen'erated file -------------------------------------*- C++ -*-===*\
|*                                                                            *|
|* Enum Utility Definitions                                                   *|
|*                                                                            *|
|* Automatically generated file, do not edit!                                 *|
|* From: Ops.td                                                               *|
|*                                                                            *|
\*===----------------------------------------------------------------------===*/

namespace cir {
::llvm::StringRef stringifyCmpFPredicate(CmpFPredicate val) {
  switch (val) {
    case CmpFPredicate::oeq: return "oeq";
    case CmpFPredicate::ogt: return "ogt";
    case CmpFPredicate::oge: return "oge";
    case CmpFPredicate::olt: return "olt";
    case CmpFPredicate::ole: return "ole";
    case CmpFPredicate::one: return "one";
    case CmpFPredicate::ord: return "ord";
    case CmpFPredicate::ueq: return "ueq";
    case CmpFPredicate::ugt: return "ugt";
    case CmpFPredicate::uge: return "uge";
    case CmpFPredicate::ult: return "ult";
    case CmpFPredicate::ule: return "ule";
    case CmpFPredicate::une: return "une";
    case CmpFPredicate::uno: return "uno";
  }
  return "";
}

::std::optional<CmpFPredicate> symbolizeCmpFPredicate(::llvm::StringRef str) {
  return ::llvm::StringSwitch<::std::optional<CmpFPredicate>>(str)
      .Case("oeq", CmpFPredicate::oeq)
      .Case("ogt", CmpFPredicate::ogt)
      .Case("oge", CmpFPredicate::oge)
      .Case("olt", CmpFPredicate::olt)
      .Case("ole", CmpFPredicate::ole)
      .Case("one", CmpFPredicate::one)
      .Case("ord", CmpFPredicate::ord)
      .Case("ueq", CmpFPredicate::ueq)
      .Case("ugt", CmpFPredicate::ugt)
      .Case("uge", CmpFPredicate::uge)
      .Case("ult", CmpFPredicate::ult)
      .Case("ule", CmpFPredicate::ule)
      .Case("une", CmpFPredicate::une)
      .Case("uno", CmpFPredicate::uno)
      .Default(::std::nullopt);
}
::std::optional<CmpFPredicate> symbolizeCmpFPredicate(uint64_t value) {
  switch (value) {
  case 0: return CmpFPredicate::oeq;
  case 1: return CmpFPredicate::ogt;
  case 2: return CmpFPredicate::oge;
  case 3: return CmpFPredicate::olt;
  case 4: return CmpFPredicate::ole;
  case 5: return CmpFPredicate::one;
  case 6: return CmpFPredicate::ord;
  case 7: return CmpFPredicate::ueq;
  case 8: return CmpFPredicate::ugt;
  case 9: return CmpFPredicate::uge;
  case 10: return CmpFPredicate::ult;
  case 11: return CmpFPredicate::ule;
  case 12: return CmpFPredicate::une;
  case 13: return CmpFPredicate::uno;
  default: return ::std::nullopt;
  }
}

bool CmpFPredicateAttr::classof(::mlir::Attribute attr) {
  return (((::llvm::isa<::mlir::IntegerAttr>(attr))) && ((::llvm::cast<::mlir::IntegerAttr>(attr).getType().isSignlessInteger(64)))) && (((::llvm::cast<::mlir::IntegerAttr>(attr).getInt() == 0)) || ((::llvm::cast<::mlir::IntegerAttr>(attr).getInt() == 1)) || ((::llvm::cast<::mlir::IntegerAttr>(attr).getInt() == 2)) || ((::llvm::cast<::mlir::IntegerAttr>(attr).getInt() == 3)) || ((::llvm::cast<::mlir::IntegerAttr>(attr).getInt() == 4)) || ((::llvm::cast<::mlir::IntegerAttr>(attr).getInt() == 5)) || ((::llvm::cast<::mlir::IntegerAttr>(attr).getInt() == 6)) || ((::llvm::cast<::mlir::IntegerAttr>(attr).getInt() == 7)) || ((::llvm::cast<::mlir::IntegerAttr>(attr).getInt() == 8)) || ((::llvm::cast<::mlir::IntegerAttr>(attr).getInt() == 9)) || ((::llvm::cast<::mlir::IntegerAttr>(attr).getInt() == 10)) || ((::llvm::cast<::mlir::IntegerAttr>(attr).getInt() == 11)) || ((::llvm::cast<::mlir::IntegerAttr>(attr).getInt() == 12)) || ((::llvm::cast<::mlir::IntegerAttr>(attr).getInt() == 13)));
}
CmpFPredicateAttr CmpFPredicateAttr::get(::mlir::MLIRContext *context, CmpFPredicate val) {
  ::mlir::IntegerType intType = ::mlir::IntegerType::get(context, 64);
  ::mlir::IntegerAttr baseAttr = ::mlir::IntegerAttr::get(intType, static_cast<uint64_t>(val));
  return ::llvm::cast<CmpFPredicateAttr>(baseAttr);
}
CmpFPredicate CmpFPredicateAttr::getValue() const {
  return static_cast<CmpFPredicate>(::mlir::IntegerAttr::getInt());
}
} // namespace cir

namespace cir {
::llvm::StringRef stringifyCmpIPredicate(CmpIPredicate val) {
  switch (val) {
    case CmpIPredicate::eq: return "eq";
    case CmpIPredicate::ne: return "ne";
    case CmpIPredicate::slt: return "slt";
    case CmpIPredicate::sle: return "sle";
    case CmpIPredicate::sgt: return "sgt";
    case CmpIPredicate::sge: return "sge";
    case CmpIPredicate::ult: return "ult";
    case CmpIPredicate::ule: return "ule";
    case CmpIPredicate::ugt: return "ugt";
    case CmpIPredicate::uge: return "uge";
  }
  return "";
}

::std::optional<CmpIPredicate> symbolizeCmpIPredicate(::llvm::StringRef str) {
  return ::llvm::StringSwitch<::std::optional<CmpIPredicate>>(str)
      .Case("eq", CmpIPredicate::eq)
      .Case("ne", CmpIPredicate::ne)
      .Case("slt", CmpIPredicate::slt)
      .Case("sle", CmpIPredicate::sle)
      .Case("sgt", CmpIPredicate::sgt)
      .Case("sge", CmpIPredicate::sge)
      .Case("ult", CmpIPredicate::ult)
      .Case("ule", CmpIPredicate::ule)
      .Case("ugt", CmpIPredicate::ugt)
      .Case("uge", CmpIPredicate::uge)
      .Default(::std::nullopt);
}
::std::optional<CmpIPredicate> symbolizeCmpIPredicate(uint64_t value) {
  switch (value) {
  case 0: return CmpIPredicate::eq;
  case 1: return CmpIPredicate::ne;
  case 2: return CmpIPredicate::slt;
  case 3: return CmpIPredicate::sle;
  case 4: return CmpIPredicate::sgt;
  case 5: return CmpIPredicate::sge;
  case 6: return CmpIPredicate::ult;
  case 7: return CmpIPredicate::ule;
  case 8: return CmpIPredicate::ugt;
  case 9: return CmpIPredicate::uge;
  default: return ::std::nullopt;
  }
}

bool CmpIPredicateAttr::classof(::mlir::Attribute attr) {
  return (((::llvm::isa<::mlir::IntegerAttr>(attr))) && ((::llvm::cast<::mlir::IntegerAttr>(attr).getType().isSignlessInteger(64)))) && (((::llvm::cast<::mlir::IntegerAttr>(attr).getInt() == 0)) || ((::llvm::cast<::mlir::IntegerAttr>(attr).getInt() == 1)) || ((::llvm::cast<::mlir::IntegerAttr>(attr).getInt() == 2)) || ((::llvm::cast<::mlir::IntegerAttr>(attr).getInt() == 3)) || ((::llvm::cast<::mlir::IntegerAttr>(attr).getInt() == 4)) || ((::llvm::cast<::mlir::IntegerAttr>(attr).getInt() == 5)) || ((::llvm::cast<::mlir::IntegerAttr>(attr).getInt() == 6)) || ((::llvm::cast<::mlir::IntegerAttr>(attr).getInt() == 7)) || ((::llvm::cast<::mlir::IntegerAttr>(attr).getInt() == 8)) || ((::llvm::cast<::mlir::IntegerAttr>(attr).getInt() == 9)));
}
CmpIPredicateAttr CmpIPredicateAttr::get(::mlir::MLIRContext *context, CmpIPredicate val) {
  ::mlir::IntegerType intType = ::mlir::IntegerType::get(context, 64);
  ::mlir::IntegerAttr baseAttr = ::mlir::IntegerAttr::get(intType, static_cast<uint64_t>(val));
  return ::llvm::cast<CmpIPredicateAttr>(baseAttr);
}
CmpIPredicate CmpIPredicateAttr::getValue() const {
  return static_cast<CmpIPredicate>(::mlir::IntegerAttr::getInt());
}
} // namespace cir

