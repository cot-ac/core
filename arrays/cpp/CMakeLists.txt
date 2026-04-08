add_mlir_library(CotArrays
  Types.cpp
  Ops.cpp
  Lowering.cpp
  ArraysConstruct.cpp

  DEPENDS
  CotArraysTypesIncGen
  CotArraysOpsIncGen

  LINK_LIBS PUBLIC
  CIR
  COTConstruct
  MLIRIR
  MLIRLLVMDialect
  MLIRInferTypeOpInterface
  MLIRSideEffectInterfaces
)
