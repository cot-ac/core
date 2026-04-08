add_mlir_library(CotArith
  Ops.cpp
  Lowering.cpp
  Transform.cpp
  ArithConstruct.cpp

  DEPENDS
  CotArithOpsIncGen

  LINK_LIBS PUBLIC
  CIR
  COTConstruct
  MLIRIR
  MLIRFuncDialect
  MLIRLLVMDialect
  MLIRPass
  MLIRInferTypeOpInterface
  MLIRCastInterfaces
  MLIRSideEffectInterfaces
)
