//===- MemoryConstruct.cpp - cot-memory construct registration *- C++ -*-===//
//
// Registers cot-memory's types, ops, and lowering patterns with the
// COT framework. Provides type parsing/printing for !cir.ptr and !cir.ref<T>.
//
//===----------------------------------------------------------------------===//
#include "cot/Construct/Construct.h"
#include "cot/CIR/CIRDialect.h"
#include "memory/Types.h"
#include "memory/Ops.h"
#include "memory/Lowering.h"

#include "mlir/Dialect/LLVMIR/LLVMDialect.h"
#include "mlir/IR/DialectImplementation.h"

using namespace mlir;

namespace {

class MemoryConstruct : public cot::Construct {
public:
  llvm::StringRef getName() const override { return "memory"; }

  void registerOpsAndTypes(MLIRContext &ctx) override {
    auto *dialect = ctx.getOrLoadDialect<cir::CIRDialect>();
    // Types registered from Types.cpp where storage classes are complete
    cir::registerMemoryTypes(dialect);
    dialect->registerConstructOps<
        cir::AllocaOp,
        cir::StoreOp,
        cir::LoadOp,
        cir::AddrOfOp,
        cir::DerefOp
    >();
  }

  void populateLoweringPatterns(
      RewritePatternSet &patterns,
      TypeConverter &typeConverter) override {
    cot::populateMemoryPatterns(patterns, typeConverter);
  }

  void addTypeConversions(TypeConverter &typeConverter) override {
    // !cir.ptr → !llvm.ptr
    typeConverter.addConversion([](cir::PointerType type) {
      return LLVM::LLVMPointerType::get(type.getContext());
    });

    // !cir.ref<T> → !llvm.ptr (zero-cost — just erase the pointee type)
    typeConverter.addConversion([](cir::RefType type) {
      return LLVM::LLVMPointerType::get(type.getContext());
    });
  }

  OptionalParseResult parseType(
      llvm::StringRef keyword, DialectAsmParser &parser,
      Type &result) const override {
    if (keyword == "ptr") {
      result = cir::PointerType::get(parser.getContext());
      return success();
    }
    if (keyword == "ref") {
      // Parse: ref<T>
      Type pointeeType;
      if (parser.parseLess() || parser.parseType(pointeeType) ||
          parser.parseGreater())
        return failure();
      result = cir::RefType::get(parser.getContext(), pointeeType);
      return success();
    }
    return {};  // Not handled by this construct
  }

  LogicalResult printType(
      Type type, DialectAsmPrinter &printer) const override {
    if (mlir::isa<cir::PointerType>(type)) {
      printer << "ptr";
      return success();
    }
    if (auto ref = mlir::dyn_cast<cir::RefType>(type)) {
      printer << "ref<";
      printer.printType(ref.getPointeeType());
      printer << ">";
      return success();
    }
    return failure();  // Not handled by this construct
  }
};

} // namespace

COT_REGISTER_CONSTRUCT(MemoryConstruct)
