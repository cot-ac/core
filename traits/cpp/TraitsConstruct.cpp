//===- TraitsConstruct.cpp - traits construct registration ------*- C++ -*-===//
#include "cot/Construct/Construct.h"
#include "cot/CIR/CIRDialect.h"
#include "traits/Types.h"
#include "traits/Ops.h"
#include "traits/Lowering.h"

#include "mlir/IR/DialectImplementation.h"
#include "mlir/Dialect/LLVMIR/LLVMDialect.h"

using namespace mlir;

namespace {

class TraitsConstruct : public cot::Construct {
public:
  llvm::StringRef getName() const override { return "traits"; }

  void registerOpsAndTypes(MLIRContext &ctx) override {
    auto *dialect = ctx.getOrLoadDialect<cir::CIRDialect>();
    cir::registerTraitsTypes(dialect);
    dialect->registerConstructOps<
        cir::WitnessTableOp,
        cir::TraitCallOp,
        cir::WitnessMethodOp,
        cir::InitExistentialOp,
        cir::OpenExistentialOp,
        cir::DeinitExistentialOp
    >();
  }

  void populateLoweringPatterns(
      RewritePatternSet &patterns,
      TypeConverter &typeConverter) override {
    cot::populateTraitsPatterns(patterns, typeConverter);
  }

  void addTypeConversions(TypeConverter &typeConverter) override {
    // !cir.existential<"P"> → struct<([24 x i8], ptr, ptr)>
    typeConverter.addConversion(
        [&](cir::ExistentialType type) -> Type {
          auto ctx = type.getContext();
          auto buffer = LLVM::LLVMArrayType::get(
              IntegerType::get(ctx, 8), 24);
          auto llvmPtr = LLVM::LLVMPointerType::get(ctx);
          return LLVM::LLVMStructType::getLiteral(
              ctx, {buffer, llvmPtr, llvmPtr});
        });
  }

  OptionalParseResult parseType(
      llvm::StringRef keyword, DialectAsmParser &parser,
      Type &result) const override {
    if (keyword == "existential") {
      auto parsed = cir::ExistentialType::parse(parser);
      if (!parsed)
        return failure();
      result = parsed;
      return success();
    }
    return {};
  }

  LogicalResult printType(
      Type type, DialectAsmPrinter &printer) const override {
    if (auto et = mlir::dyn_cast<cir::ExistentialType>(type)) {
      printer << "existential<\"" << et.getName() << "\">";
      return success();
    }
    return failure();
  }
};

} // namespace

COT_REGISTER_CONSTRUCT(TraitsConstruct)
