//===- ArraysConstruct.cpp - cot-arrays construct registration -*- C++ -*-===//
//
// Registers cot-arrays' type, ops, and lowering patterns with the
// COT framework. Provides type parsing/printing for !cir.array<N x T>.
//
//===----------------------------------------------------------------------===//
#include "cot/Construct/Construct.h"
#include "cot/CIR/CIRDialect.h"
#include "arrays/Types.h"
#include "arrays/Ops.h"
#include "arrays/Lowering.h"

#include "mlir/Dialect/LLVMIR/LLVMDialect.h"
#include "mlir/IR/DialectImplementation.h"

using namespace mlir;

namespace {

class ArraysConstruct : public cot::Construct {
public:
  llvm::StringRef getName() const override { return "arrays"; }

  void registerOpsAndTypes(MLIRContext &ctx) override {
    auto *dialect = ctx.getOrLoadDialect<cir::CIRDialect>();
    cir::registerArraysTypes(dialect);
    dialect->registerConstructOps<
        cir::ArrayInitOp,
        cir::ElemValOp,
        cir::ElemPtrOp
    >();
  }

  void populateLoweringPatterns(
      RewritePatternSet &patterns,
      TypeConverter &typeConverter) override {
    cot::populateArraysPatterns(patterns, typeConverter);
  }

  void addTypeConversions(TypeConverter &typeConverter) override {
    // !cir.array<N x T> → !llvm.array<N x convertedT>
    typeConverter.addConversion(
        [&](cir::ArrayType type) -> Type {
          auto elemType = typeConverter.convertType(type.getElementType());
          return LLVM::LLVMArrayType::get(elemType, type.getSize());
        });
  }

  OptionalParseResult parseType(
      llvm::StringRef keyword, DialectAsmParser &parser,
      Type &result) const override {
    if (keyword == "array") {
      result = cir::ArrayType::parse(parser);
      return result ? success() : failure();
    }
    return {};  // Not handled by this construct
  }

  LogicalResult printType(
      Type type, DialectAsmPrinter &printer) const override {
    if (auto arr = mlir::dyn_cast<cir::ArrayType>(type)) {
      printer << "array";
      arr.print(printer);
      return success();
    }
    return failure();
  }
};

} // namespace

COT_REGISTER_CONSTRUCT(ArraysConstruct)
