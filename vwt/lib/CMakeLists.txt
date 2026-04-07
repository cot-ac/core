add_mlir_library(CotVWT
  Ops.cpp
  Lowering.cpp
  VWTConstruct.cpp

  DEPENDS
  CotVWTOpsIncGen

  LINK_LIBS PUBLIC
  CIR
  COTConstruct
  MLIRIR
  MLIRLLVMDialect
  MLIRSideEffectInterfaces
)
