add_mlir_library(CotMemory
  Types.cpp
  Ops.cpp
  Lowering.cpp
  MemoryConstruct.cpp

  DEPENDS
  CotMemoryTypesIncGen
  CotMemoryOpsIncGen

  LINK_LIBS PUBLIC
  CIR
  COTConstruct
  MLIRIR
  MLIRLLVMDialect
  MLIRInferTypeOpInterface
  MLIRSideEffectInterfaces
)
