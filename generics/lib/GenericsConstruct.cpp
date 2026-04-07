//===- GenericsConstruct.cpp - generics construct registration --*- C++ -*-===//
//
// Registers type_param type and generic_apply op with the COT framework.
//
//===----------------------------------------------------------------------===//
#include "cot/Construct/Construct.h"
#include "cot/CIR/CIRDialect.h"
#include "generics/Types.h"
#include "generics/Ops.h"
#include "generics/Lowering.h"
#include "generics/GenericSpecializer.h"

#include "mlir/Dialect/Func/IR/FuncOps.h"
#include "mlir/Pass/Pass.h"
#include "mlir/IR/DialectImplementation.h"
#include "mlir/Dialect/LLVMIR/LLVMDialect.h"
#include "mlir/Pass/PassManager.h"

using namespace mlir;

namespace {

class GenericsConstruct : public cot::Construct {
public:
  llvm::StringRef getName() const override { return "generics"; }

  void registerOpsAndTypes(MLIRContext &ctx) override {
    auto *dialect = ctx.getOrLoadDialect<cir::CIRDialect>();
    cir::registerGenericsTypes(dialect);
    dialect->registerConstructOps<
        cir::GenericApplyOp
    >();
  }

  void addTransformers(PassManager &preSemaPM,
                       PassManager &postSemaPM) override {
    // GenericSpecializer runs BEFORE Sema — it resolves type_param to
    // concrete types so Sema can validate them.
    preSemaPM.addPass(cot::createGenericSpecializerPass());
  }

  void populateLoweringPatterns(
      RewritePatternSet &patterns,
      TypeConverter &typeConverter) override {
    cot::populateGenericsPatterns(patterns, typeConverter);
  }

  void addTypeConversions(TypeConverter &typeConverter) override {
    // !cir.type_param<"T"> → !llvm.ptr (fallback — should be resolved first)
    typeConverter.addConversion(
        [&](cir::TypeParamType type) -> Type {
          return LLVM::LLVMPointerType::get(type.getContext());
        });
  }

  OptionalParseResult parseType(
      llvm::StringRef keyword, DialectAsmParser &parser,
      Type &result) const override {
    if (keyword == "type_param") {
      auto parsed = cir::TypeParamType::parse(parser);
      if (!parsed)
        return failure();
      result = parsed;
      return success();
    }
    return {};
  }

  LogicalResult printType(
      Type type, DialectAsmPrinter &printer) const override {
    if (auto tp = mlir::dyn_cast<cir::TypeParamType>(type)) {
      printer << "type_param<\"" << tp.getName() << "\">";
      return success();
    }
    return failure();
  }
};

} // namespace

COT_REGISTER_CONSTRUCT(GenericsConstruct)
