/*===- TableGen'erated file -------------------------------------*- C++ -*-===*\
|*                                                                            *|
|* Enum Utility Declarations                                                  *|
|*                                                                            *|
|* Automatically generated file, do not edit!                                 *|
|* From: Ops.td                                                               *|
|*                                                                            *|
\*===----------------------------------------------------------------------===*/

namespace cir {
// float comparison predicate
enum class CmpFPredicate : uint64_t {
  oeq = 0,
  ogt = 1,
  oge = 2,
  olt = 3,
  ole = 4,
  one = 5,
  ord = 6,
  ueq = 7,
  ugt = 8,
  uge = 9,
  ult = 10,
  ule = 11,
  une = 12,
  uno = 13,
};

::std::optional<CmpFPredicate> symbolizeCmpFPredicate(uint64_t);
::llvm::StringRef stringifyCmpFPredicate(CmpFPredicate);
::std::optional<CmpFPredicate> symbolizeCmpFPredicate(::llvm::StringRef);
inline constexpr unsigned getMaxEnumValForCmpFPredicate() {
  return 13;
}


inline ::llvm::StringRef stringifyEnum(CmpFPredicate enumValue) {
  return stringifyCmpFPredicate(enumValue);
}

template <typename EnumType>
::std::optional<EnumType> symbolizeEnum(::llvm::StringRef);

template <>
inline ::std::optional<CmpFPredicate> symbolizeEnum<CmpFPredicate>(::llvm::StringRef str) {
  return symbolizeCmpFPredicate(str);
}

class CmpFPredicateAttr : public ::mlir::IntegerAttr {
public:
  using ValueType = CmpFPredicate;
  using ::mlir::IntegerAttr::IntegerAttr;
  static bool classof(::mlir::Attribute attr);
  static CmpFPredicateAttr get(::mlir::MLIRContext *context, CmpFPredicate val);
  CmpFPredicate getValue() const;
};
} // namespace cir

namespace mlir {
template <typename T, typename>
struct FieldParser;

template<>
struct FieldParser<::cir::CmpFPredicate, ::cir::CmpFPredicate> {
  template <typename ParserT>
  static FailureOr<::cir::CmpFPredicate> parse(ParserT &parser) {
    // Parse the keyword/string containing the enum.
    std::string enumKeyword;
    auto loc = parser.getCurrentLocation();
    if (failed(parser.parseOptionalKeywordOrString(&enumKeyword)))
      return parser.emitError(loc, "expected keyword for float comparison predicate");

    // Symbolize the keyword.
    if (::std::optional<::cir::CmpFPredicate> attr = ::cir::symbolizeEnum<::cir::CmpFPredicate>(enumKeyword))
      return *attr;
    return parser.emitError(loc, "invalid float comparison predicate specification: ") << enumKeyword;
  }
};

/// Support for std::optional, useful in attribute/type definition where the enum is
/// used as:
///
///    let parameters = (ins OptionalParameter<"std::optional<TheEnumName>">:$value);
template<>
struct FieldParser<std::optional<::cir::CmpFPredicate>, std::optional<::cir::CmpFPredicate>> {
  template <typename ParserT>
  static FailureOr<std::optional<::cir::CmpFPredicate>> parse(ParserT &parser) {
    // Parse the keyword/string containing the enum.
    std::string enumKeyword;
    auto loc = parser.getCurrentLocation();
    if (failed(parser.parseOptionalKeywordOrString(&enumKeyword)))
      return std::optional<::cir::CmpFPredicate>{};

    // Symbolize the keyword.
    if (::std::optional<::cir::CmpFPredicate> attr = ::cir::symbolizeEnum<::cir::CmpFPredicate>(enumKeyword))
      return attr;
    return parser.emitError(loc, "invalid float comparison predicate specification: ") << enumKeyword;
  }
};
} // namespace mlir

namespace llvm {
inline ::llvm::raw_ostream &operator<<(::llvm::raw_ostream &p, ::cir::CmpFPredicate value) {
  auto valueStr = stringifyEnum(value);
  return p << valueStr;
}
} // namespace llvm

namespace llvm {
template<> struct DenseMapInfo<::cir::CmpFPredicate> {
  using StorageInfo = ::llvm::DenseMapInfo<uint64_t>;

  static inline ::cir::CmpFPredicate getEmptyKey() {
    return static_cast<::cir::CmpFPredicate>(StorageInfo::getEmptyKey());
  }

  static inline ::cir::CmpFPredicate getTombstoneKey() {
    return static_cast<::cir::CmpFPredicate>(StorageInfo::getTombstoneKey());
  }

  static unsigned getHashValue(const ::cir::CmpFPredicate &val) {
    return StorageInfo::getHashValue(static_cast<uint64_t>(val));
  }

  static bool isEqual(const ::cir::CmpFPredicate &lhs, const ::cir::CmpFPredicate &rhs) {
    return lhs == rhs;
  }
};
}

namespace cir {
// integer comparison predicate
enum class CmpIPredicate : uint64_t {
  eq = 0,
  ne = 1,
  slt = 2,
  sle = 3,
  sgt = 4,
  sge = 5,
  ult = 6,
  ule = 7,
  ugt = 8,
  uge = 9,
};

::std::optional<CmpIPredicate> symbolizeCmpIPredicate(uint64_t);
::llvm::StringRef stringifyCmpIPredicate(CmpIPredicate);
::std::optional<CmpIPredicate> symbolizeCmpIPredicate(::llvm::StringRef);
inline constexpr unsigned getMaxEnumValForCmpIPredicate() {
  return 9;
}


inline ::llvm::StringRef stringifyEnum(CmpIPredicate enumValue) {
  return stringifyCmpIPredicate(enumValue);
}

template <typename EnumType>
::std::optional<EnumType> symbolizeEnum(::llvm::StringRef);

template <>
inline ::std::optional<CmpIPredicate> symbolizeEnum<CmpIPredicate>(::llvm::StringRef str) {
  return symbolizeCmpIPredicate(str);
}

class CmpIPredicateAttr : public ::mlir::IntegerAttr {
public:
  using ValueType = CmpIPredicate;
  using ::mlir::IntegerAttr::IntegerAttr;
  static bool classof(::mlir::Attribute attr);
  static CmpIPredicateAttr get(::mlir::MLIRContext *context, CmpIPredicate val);
  CmpIPredicate getValue() const;
};
} // namespace cir

namespace mlir {
template <typename T, typename>
struct FieldParser;

template<>
struct FieldParser<::cir::CmpIPredicate, ::cir::CmpIPredicate> {
  template <typename ParserT>
  static FailureOr<::cir::CmpIPredicate> parse(ParserT &parser) {
    // Parse the keyword/string containing the enum.
    std::string enumKeyword;
    auto loc = parser.getCurrentLocation();
    if (failed(parser.parseOptionalKeywordOrString(&enumKeyword)))
      return parser.emitError(loc, "expected keyword for integer comparison predicate");

    // Symbolize the keyword.
    if (::std::optional<::cir::CmpIPredicate> attr = ::cir::symbolizeEnum<::cir::CmpIPredicate>(enumKeyword))
      return *attr;
    return parser.emitError(loc, "invalid integer comparison predicate specification: ") << enumKeyword;
  }
};

/// Support for std::optional, useful in attribute/type definition where the enum is
/// used as:
///
///    let parameters = (ins OptionalParameter<"std::optional<TheEnumName>">:$value);
template<>
struct FieldParser<std::optional<::cir::CmpIPredicate>, std::optional<::cir::CmpIPredicate>> {
  template <typename ParserT>
  static FailureOr<std::optional<::cir::CmpIPredicate>> parse(ParserT &parser) {
    // Parse the keyword/string containing the enum.
    std::string enumKeyword;
    auto loc = parser.getCurrentLocation();
    if (failed(parser.parseOptionalKeywordOrString(&enumKeyword)))
      return std::optional<::cir::CmpIPredicate>{};

    // Symbolize the keyword.
    if (::std::optional<::cir::CmpIPredicate> attr = ::cir::symbolizeEnum<::cir::CmpIPredicate>(enumKeyword))
      return attr;
    return parser.emitError(loc, "invalid integer comparison predicate specification: ") << enumKeyword;
  }
};
} // namespace mlir

namespace llvm {
inline ::llvm::raw_ostream &operator<<(::llvm::raw_ostream &p, ::cir::CmpIPredicate value) {
  auto valueStr = stringifyEnum(value);
  return p << valueStr;
}
} // namespace llvm

namespace llvm {
template<> struct DenseMapInfo<::cir::CmpIPredicate> {
  using StorageInfo = ::llvm::DenseMapInfo<uint64_t>;

  static inline ::cir::CmpIPredicate getEmptyKey() {
    return static_cast<::cir::CmpIPredicate>(StorageInfo::getEmptyKey());
  }

  static inline ::cir::CmpIPredicate getTombstoneKey() {
    return static_cast<::cir::CmpIPredicate>(StorageInfo::getTombstoneKey());
  }

  static unsigned getHashValue(const ::cir::CmpIPredicate &val) {
    return StorageInfo::getHashValue(static_cast<uint64_t>(val));
  }

  static bool isEqual(const ::cir::CmpIPredicate &lhs, const ::cir::CmpIPredicate &rhs) {
    return lhs == rhs;
  }
};
}

