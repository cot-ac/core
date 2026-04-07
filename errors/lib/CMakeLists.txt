add_mlir_library(CotErrors
  Types.cpp
  Ops.cpp
  Lowering.cpp
  ErrorsConstruct.cpp

  DEPENDS
  CotErrorsTypesIncGen
  CotErrorsOpsIncGen

  LINK_LIBS PUBLIC
  CIR
  COTConstruct
  MLIRIR
  MLIRLLVMDialect
  MLIRInferTypeOpInterface
  MLIRSideEffectInterfaces
)
