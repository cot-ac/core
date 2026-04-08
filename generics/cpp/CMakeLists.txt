add_mlir_library(CotGenerics
  Types.cpp
  Ops.cpp
  Lowering.cpp
  GenericSpecializer.cpp
  GenericsConstruct.cpp

  DEPENDS
  CotGenericsTypesIncGen
  CotGenericsOpsIncGen

  LINK_LIBS PUBLIC
  CIR
  COTConstruct
  MLIRIR
  MLIRLLVMDialect
  MLIRFuncDialect
  MLIRPass
  MLIRInferTypeOpInterface
  MLIRSideEffectInterfaces
)
