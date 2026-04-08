//===- OptionalsConstruct.cpp - cot-optionals construct registration -*- C++ -*-===//
//
// Registers cot-optionals' type, ops, and lowering patterns with the
// COT framework. Provides type parsing/printing for !cir.optional<T>.
//
//===----------------------------------------------------------------------===//
#include "cot/Construct/Construct.h"
#include "cot/CIR/CIRDialect.h"
#include "optionals/Types.h"
#include "optionals/Ops.h"
#include "optionals/Lowering.h"

#include "mlir/Dialect/LLVMIR/LLVMDialect.h"
#include "mlir/IR/DialectImplementation.h"

using namespace mlir;

namespace {

class OptionalsConstruct : public cot::Construct {
public:
  llvm::StringRef getName() const override { return "optionals"; }

  llvm::SmallVector<llvm::StringRef> getRequiredConstructs() const override {
    // Requires cot-memory: PointerLikeTypeInterface is implemented by
    // !cir.ptr and !cir.ref<T>, which must be loaded for isPointerLike()
    // to detect pointer-like payloads.
    return {"memory"};
  }

  void registerOpsAndTypes(MLIRContext &ctx) override {
    auto *dialect = ctx.getOrLoadDialect<cir::CIRDialect>();
    // Types registered from Types.cpp where storage classes are complete
    cir::registerOptionalsTypes(dialect);
    dialect->registerConstructOps<
        cir::NoneOp,
        cir::WrapOptionalOp,
        cir::IsNonNullOp,
        cir::OptionalPayloadOp
    >();
  }

  void populateLoweringPatterns(
      RewritePatternSet &patterns,
      TypeConverter &typeConverter) override {
    cot::populateOptionalsPatterns(patterns, typeConverter);
  }

  void addTypeConversions(TypeConverter &typeConverter) override {
    // !cir.optional<T> -> ptr (pointer-like) or struct<(T, i1)>
    // Uses OptionalType::isPointerLike() backed by PointerLikeTypeInterface.
    typeConverter.addConversion(
        [&](cir::OptionalType type) -> Type {
          auto *ctx = type.getContext();
          if (type.isPointerLike())
            return LLVM::LLVMPointerType::get(ctx);
          auto payloadType = typeConverter.convertType(type.getPayloadType());
          return LLVM::LLVMStructType::getLiteral(
              ctx, {payloadType, IntegerType::get(ctx, 1)});
        });
  }

  OptionalParseResult parseType(
      llvm::StringRef keyword, DialectAsmParser &parser,
      Type &result) const override {
    if (keyword == "optional") {
      // Parse: optional<T>
      Type payloadType;
      if (parser.parseLess() || parser.parseType(payloadType) ||
          parser.parseGreater())
        return failure();
      result = cir::OptionalType::get(parser.getContext(), payloadType);
      return success();
    }
    return {};  // Not handled by this construct
  }

  LogicalResult printType(
      Type type, DialectAsmPrinter &printer) const override {
    if (auto opt = mlir::dyn_cast<cir::OptionalType>(type)) {
      printer << "optional<";
      printer.printType(opt.getPayloadType());
      printer << ">";
      return success();
    }
    return failure();  // Not handled by this construct
  }
};

} // namespace

COT_REGISTER_CONSTRUCT(OptionalsConstruct)
