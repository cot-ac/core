add_mlir_library(CotComptime
  ComptimeStep.cpp

  LINK_LIBS PUBLIC
  CIR
  COTConstruct
  MLIRIR
  MLIRPass
)

# CIRSema headers from cot-dev (header-only dependency, no link needed)
target_include_directories(obj.CotComptime PRIVATE
  "${COT_SOURCE_DIR}/include"
  "${COT_BUILD_DIR}/include"
)
