//===- EnumsConstruct.cpp - enums construct registration -------*- C++ -*-===//
//
// Registers enums' type, ops, and lowering patterns with the COT framework.
//
//===----------------------------------------------------------------------===//
#include "cot/Construct/Construct.h"
#include "cot/CIR/CIRDialect.h"
#include "enums/Types.h"
#include "enums/Ops.h"
#include "enums/Lowering.h"

#include "mlir/IR/DialectImplementation.h"

using namespace mlir;

namespace {

class EnumsConstruct : public cot::Construct {
public:
  llvm::StringRef getName() const override { return "enums"; }

  void registerOpsAndTypes(MLIRContext &ctx) override {
    auto *dialect = ctx.getOrLoadDialect<cir::CIRDialect>();
    cir::registerEnumsTypes(dialect);
    dialect->registerConstructOps<
        cir::EnumConstantOp,
        cir::EnumValueOp
    >();
  }

  void populateLoweringPatterns(
      RewritePatternSet &patterns,
      TypeConverter &typeConverter) override {
    cot::populateEnumsPatterns(patterns, typeConverter);
  }

  void addTypeConversions(TypeConverter &typeConverter) override {
    // !cir.enum<...> -> tag type directly (enum IS the integer)
    typeConverter.addConversion(
        [&](cir::EnumType type) -> Type {
          return typeConverter.convertType(type.getTagType());
        });
  }

  OptionalParseResult parseType(
      llvm::StringRef keyword, DialectAsmParser &parser,
      Type &result) const override {
    if (keyword == "enum") {
      auto parsed = cir::EnumType::parse(parser);
      if (!parsed)
        return failure();
      result = parsed;
      return success();
    }
    return {};
  }

  LogicalResult printType(
      Type type, DialectAsmPrinter &printer) const override {
    if (auto et = mlir::dyn_cast<cir::EnumType>(type)) {
      printer << "enum<\"" << et.getName() << "\", ";
      printer.printType(et.getTagType());
      for (auto v : et.getVariants())
        printer << ", \"" << v.getValue() << "\"";
      printer << ">";
      return success();
    }
    return failure();
  }
};

} // namespace

COT_REGISTER_CONSTRUCT(EnumsConstruct)
