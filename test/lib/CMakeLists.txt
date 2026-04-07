add_mlir_library(CotTest
  Ops.cpp
  Lowering.cpp
  TestRunnerGenerator.cpp
  TestConstruct.cpp

  DEPENDS
  CotTestOpsIncGen

  LINK_LIBS PUBLIC
  CIR
  COTConstruct
  MLIRIR
  MLIRFuncDialect
  MLIRLLVMDialect
  MLIRPass
  MLIRSideEffectInterfaces
)
