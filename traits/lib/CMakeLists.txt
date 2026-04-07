add_mlir_library(CotTraits
  Types.cpp
  Ops.cpp
  Lowering.cpp
  TraitsConstruct.cpp

  DEPENDS
  CotTraitsTypesIncGen
  CotTraitsOpsIncGen

  LINK_LIBS PUBLIC
  CIR
  COTConstruct
  MLIRIR
  MLIRLLVMDialect
  MLIRFuncDialect
  MLIRInferTypeOpInterface
  MLIRSideEffectInterfaces
)
