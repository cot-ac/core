add_mlir_library(CotStructs
  Types.cpp
  Ops.cpp
  Lowering.cpp
  StructsConstruct.cpp

  DEPENDS
  CotStructsTypesIncGen
  CotStructsOpsIncGen

  LINK_LIBS PUBLIC
  CIR
  COTConstruct
  MLIRIR
  MLIRLLVMDialect
  MLIRInferTypeOpInterface
  MLIRSideEffectInterfaces
)
