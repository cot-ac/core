add_mlir_library(CotFlow
  Ops.cpp
  Lowering.cpp
  FlowConstruct.cpp

  DEPENDS
  CotFlowOpsIncGen

  LINK_LIBS PUBLIC
  CIR
  COTConstruct
  MLIRIR
  MLIRLLVMDialect
  MLIRControlFlowInterfaces
  MLIRSideEffectInterfaces
)
