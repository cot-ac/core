//===- SlicesConstruct.cpp - cot-slices construct registration -*- C++ -*-===//
//
// Registers cot-slices' type, ops, and lowering patterns with the
// COT framework. Provides type parsing/printing for !cir.slice<T>.
//
//===----------------------------------------------------------------------===//
#include "cot/Construct/Construct.h"
#include "cot/CIR/CIRDialect.h"
#include "slices/Types.h"
#include "slices/Ops.h"
#include "slices/Lowering.h"

#include "mlir/Dialect/LLVMIR/LLVMDialect.h"
#include "mlir/IR/DialectImplementation.h"

using namespace mlir;

namespace {

class SlicesConstruct : public cot::Construct {
public:
  llvm::StringRef getName() const override { return "slices"; }

  void registerOpsAndTypes(MLIRContext &ctx) override {
    auto *dialect = ctx.getOrLoadDialect<cir::CIRDialect>();
    cir::registerSlicesTypes(dialect);
    dialect->registerConstructOps<
        cir::StringConstantOp,
        cir::SlicePtrOp,
        cir::SliceLenOp,
        cir::SliceElemOp,
        cir::ArrayToSliceOp
    >();
  }

  void populateLoweringPatterns(
      RewritePatternSet &patterns,
      TypeConverter &typeConverter) override {
    cot::populateSlicesPatterns(patterns, typeConverter);
  }

  void addTypeConversions(TypeConverter &typeConverter) override {
    // !cir.slice<T> → !llvm.struct<(!llvm.ptr, i64)>
    typeConverter.addConversion(
        [](cir::SliceType type) -> Type {
          auto *ctx = type.getContext();
          auto ptr = LLVM::LLVMPointerType::get(ctx);
          auto i64 = IntegerType::get(ctx, 64);
          return LLVM::LLVMStructType::getLiteral(ctx, {ptr, i64});
        });
  }

  OptionalParseResult parseType(
      llvm::StringRef keyword, DialectAsmParser &parser,
      Type &result) const override {
    if (keyword == "slice") {
      result = cir::SliceType::parse(parser);
      return result ? success() : failure();
    }
    return {};
  }

  LogicalResult printType(
      Type type, DialectAsmPrinter &printer) const override {
    if (auto sl = mlir::dyn_cast<cir::SliceType>(type)) {
      printer << "slice";
      sl.print(printer);
      return success();
    }
    return failure();
  }
};

} // namespace

COT_REGISTER_CONSTRUCT(SlicesConstruct)
