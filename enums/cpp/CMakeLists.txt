add_mlir_library(CotEnums
  Types.cpp
  Ops.cpp
  Lowering.cpp
  EnumsConstruct.cpp

  DEPENDS
  CotEnumsTypesIncGen
  CotEnumsOpsIncGen

  LINK_LIBS PUBLIC
  CIR
  COTConstruct
  MLIRIR
  MLIRLLVMDialect
  MLIRInferTypeOpInterface
  MLIRSideEffectInterfaces
)
