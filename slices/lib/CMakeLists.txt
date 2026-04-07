add_mlir_library(CotSlices
  Types.cpp
  Ops.cpp
  Lowering.cpp
  SlicesConstruct.cpp

  DEPENDS
  CotSlicesTypesIncGen
  CotSlicesOpsIncGen

  LINK_LIBS PUBLIC
  CIR
  COTConstruct
  MLIRIR
  MLIRLLVMDialect
  MLIRInferTypeOpInterface
  MLIRSideEffectInterfaces
)
