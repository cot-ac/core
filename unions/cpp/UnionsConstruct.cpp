//===- UnionsConstruct.cpp - unions construct registration -----*- C++ -*-===//
#include "cot/Construct/Construct.h"
#include "cot/CIR/CIRDialect.h"
#include "unions/Types.h"
#include "unions/Ops.h"
#include "unions/Lowering.h"

#include "mlir/Dialect/LLVMIR/LLVMDialect.h"
#include "mlir/IR/DialectImplementation.h"

using namespace mlir;

namespace {

class UnionsConstruct : public cot::Construct {
public:
  llvm::StringRef getName() const override { return "unions"; }

  void registerOpsAndTypes(MLIRContext &ctx) override {
    auto *dialect = ctx.getOrLoadDialect<cir::CIRDialect>();
    cir::registerUnionsTypes(dialect);
    dialect->registerConstructOps<
        cir::UnionInitOp,
        cir::UnionTagOp,
        cir::UnionPayloadOp
    >();
  }

  void populateLoweringPatterns(
      RewritePatternSet &patterns,
      TypeConverter &typeConverter) override {
    cot::populateUnionsPatterns(patterns, typeConverter);
  }

  void addTypeConversions(TypeConverter &typeConverter) override {
    // !cir.tagged_union<...> -> struct<(i8, [max_bytes x i8])>
    typeConverter.addConversion(
        [&](cir::TaggedUnionType type) -> Type {
          auto *ctx = type.getContext();
          unsigned maxBits = type.getMaxPayloadBitWidth();
          unsigned maxBytes = (maxBits + 7) / 8;
          if (maxBytes == 0) maxBytes = 1;
          auto payloadArray = LLVM::LLVMArrayType::get(
              IntegerType::get(ctx, 8), maxBytes);
          return LLVM::LLVMStructType::getLiteral(
              ctx, {IntegerType::get(ctx, 8), payloadArray});
        });
  }

  OptionalParseResult parseType(
      llvm::StringRef keyword, DialectAsmParser &parser,
      Type &result) const override {
    if (keyword == "tagged_union") {
      auto parsed = cir::TaggedUnionType::parse(parser);
      if (!parsed)
        return failure();
      result = parsed;
      return success();
    }
    return {};
  }

  LogicalResult printType(
      Type type, DialectAsmPrinter &printer) const override {
    if (auto tu = mlir::dyn_cast<cir::TaggedUnionType>(type)) {
      printer << "tagged_union";
      tu.print(printer);
      return success();
    }
    return failure();
  }
};

} // namespace

COT_REGISTER_CONSTRUCT(UnionsConstruct)
