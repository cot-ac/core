add_mlir_library(CotUnions
  Types.cpp
  Ops.cpp
  Lowering.cpp
  UnionsConstruct.cpp

  DEPENDS
  CotUnionsTypesIncGen
  CotUnionsOpsIncGen

  LINK_LIBS PUBLIC
  CIR
  COTConstruct
  MLIRIR
  MLIRLLVMDialect
  MLIRInferTypeOpInterface
  MLIRSideEffectInterfaces
)
