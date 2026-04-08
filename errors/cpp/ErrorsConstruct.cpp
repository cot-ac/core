//===- ErrorsConstruct.cpp - cot-errors construct registration -*- C++ -*-===//
//
// Registers cot-errors' type, ops, and lowering patterns with the
// COT framework. Provides type parsing/printing for !cir.error_union<T>.
//
//===----------------------------------------------------------------------===//
#include "cot/Construct/Construct.h"
#include "cot/CIR/CIRDialect.h"
#include "errors/Types.h"
#include "errors/Ops.h"
#include "errors/Lowering.h"

#include "mlir/Dialect/LLVMIR/LLVMDialect.h"
#include "mlir/IR/DialectImplementation.h"

using namespace mlir;

namespace {

class ErrorsConstruct : public cot::Construct {
public:
  llvm::StringRef getName() const override { return "errors"; }

  void registerOpsAndTypes(MLIRContext &ctx) override {
    auto *dialect = ctx.getOrLoadDialect<cir::CIRDialect>();
    // Types registered from Types.cpp where storage classes are complete
    cir::registerErrorsTypes(dialect);
    dialect->registerConstructOps<
        cir::WrapResultOp,
        cir::WrapErrorOp,
        cir::IsErrorOp,
        cir::ErrorPayloadOp,
        cir::ErrorCodeOp
    >();
  }

  void populateLoweringPatterns(
      RewritePatternSet &patterns,
      TypeConverter &typeConverter) override {
    cot::populateErrorsPatterns(patterns, typeConverter);
  }

  void addTypeConversions(TypeConverter &typeConverter) override {
    // !cir.error_union<T> -> !llvm.struct<(T, i16)>
    typeConverter.addConversion(
        [&](cir::ErrorUnionType type) -> Type {
          auto *ctx = type.getContext();
          auto payloadType = typeConverter.convertType(type.getPayloadType());
          return LLVM::LLVMStructType::getLiteral(
              ctx, {payloadType, IntegerType::get(ctx, 16)});
        });
  }

  OptionalParseResult parseType(
      llvm::StringRef keyword, DialectAsmParser &parser,
      Type &result) const override {
    if (keyword == "error_union") {
      // Parse: error_union<T>
      Type payloadType;
      if (parser.parseLess() || parser.parseType(payloadType) ||
          parser.parseGreater())
        return failure();
      result = cir::ErrorUnionType::get(parser.getContext(), payloadType);
      return success();
    }
    return {};  // Not handled by this construct
  }

  LogicalResult printType(
      Type type, DialectAsmPrinter &printer) const override {
    if (auto eu = mlir::dyn_cast<cir::ErrorUnionType>(type)) {
      printer << "error_union<";
      printer.printType(eu.getPayloadType());
      printer << ">";
      return success();
    }
    return failure();  // Not handled by this construct
  }
};

} // namespace

COT_REGISTER_CONSTRUCT(ErrorsConstruct)
