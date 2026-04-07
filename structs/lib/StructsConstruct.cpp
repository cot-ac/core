//===- StructsConstruct.cpp - cot-structs construct registration -*- C++ -*-===//
//
// Registers cot-structs' type, ops, and lowering patterns with the
// COT framework. Provides type parsing/printing for !cir.struct<...>.
//
//===----------------------------------------------------------------------===//
#include "cot/Construct/Construct.h"
#include "cot/CIR/CIRDialect.h"
#include "structs/Types.h"
#include "structs/Ops.h"
#include "structs/Lowering.h"

#include "mlir/Dialect/LLVMIR/LLVMDialect.h"
#include "mlir/IR/DialectImplementation.h"

using namespace mlir;

namespace {

class StructsConstruct : public cot::Construct {
public:
  llvm::StringRef getName() const override { return "structs"; }

  void registerOpsAndTypes(MLIRContext &ctx) override {
    auto *dialect = ctx.getOrLoadDialect<cir::CIRDialect>();
    // Types registered from Types.cpp where storage classes are complete
    cir::registerStructsTypes(dialect);
    dialect->registerConstructOps<
        cir::StructInitOp,
        cir::FieldValOp,
        cir::FieldPtrOp
    >();
  }

  void populateLoweringPatterns(
      RewritePatternSet &patterns,
      TypeConverter &typeConverter) override {
    cot::populateStructsPatterns(patterns, typeConverter);
  }

  void addTypeConversions(TypeConverter &typeConverter) override {
    // !cir.struct<"N", fields...> → !llvm.struct<(convertedFields...)>
    typeConverter.addConversion(
        [&](cir::StructType type) -> Type {
          auto *ctx = type.getContext();
          SmallVector<Type> fieldTypes;
          for (auto ft : type.getFieldTypes())
            fieldTypes.push_back(typeConverter.convertType(ft));
          return LLVM::LLVMStructType::getLiteral(ctx, fieldTypes);
        });
  }

  OptionalParseResult parseType(
      llvm::StringRef keyword, DialectAsmParser &parser,
      Type &result) const override {
    if (keyword == "struct") {
      result = cir::StructType::parse(parser);
      return result ? success() : failure();
    }
    return {};  // Not handled by this construct
  }

  LogicalResult printType(
      Type type, DialectAsmPrinter &printer) const override {
    if (auto st = mlir::dyn_cast<cir::StructType>(type)) {
      printer << "struct";
      st.print(printer);
      return success();
    }
    return failure();  // Not handled by this construct
  }
};

} // namespace

COT_REGISTER_CONSTRUCT(StructsConstruct)
