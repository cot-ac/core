add_mlir_library(CotOptionals
  Types.cpp
  Ops.cpp
  Lowering.cpp
  OptionalsConstruct.cpp

  DEPENDS
  CotOptionalsTypesIncGen
  CotOptionalsOpsIncGen

  LINK_LIBS PUBLIC
  CIR
  COTConstruct
  MLIRIR
  MLIRLLVMDialect
  MLIRInferTypeOpInterface
  MLIRSideEffectInterfaces
)
