cmake_minimum_required(VERSION 3.20)
project(cot-core-constructs VERSION 0.1.0 LANGUAGES CXX C)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

find_package(MLIR REQUIRED CONFIG)
find_package(LLVM REQUIRED CONFIG)

list(APPEND CMAKE_MODULE_PATH "${MLIR_CMAKE_DIR}")
list(APPEND CMAKE_MODULE_PATH "${LLVM_CMAKE_DIR}")

include(TableGen)
include(AddLLVM)
include(AddMLIR)
include(HandleLLVMOptions)

include_directories(${LLVM_INCLUDE_DIRS})
include_directories(${MLIR_INCLUDE_DIRS})

# COT framework: source headers + generated .inc files
set(COT_SOURCE_DIR "${PROJECT_SOURCE_DIR}/../cot-dev" CACHE PATH
    "Path to cot-dev framework source")
set(COT_BUILD_DIR "${COT_SOURCE_DIR}/build" CACHE PATH
    "Path to cot framework build")

include_directories(${COT_SOURCE_DIR}/include)
include_directories(${COT_BUILD_DIR}/include)

add_definitions(${LLVM_DEFINITIONS})

# Import COT framework libraries as targets (shared by all constructs)
add_library(CIR STATIC IMPORTED)
set_target_properties(CIR PROPERTIES
  IMPORTED_LOCATION "${COT_BUILD_DIR}/lib/CIR/libCIR.a")

add_library(COTConstruct STATIC IMPORTED)
set_target_properties(COTConstruct PROPERTIES
  IMPORTED_LOCATION "${COT_BUILD_DIR}/lib/Construct/libCOTConstruct.a")

# Add include paths for all constructs (source + generated)
set(CORE_CONSTRUCTS arith memory flow structs arrays slices optionals errors test enums unions)
foreach(construct ${CORE_CONSTRUCTS})
  include_directories(${PROJECT_SOURCE_DIR}/${construct}/include)
  include_directories(${PROJECT_BINARY_DIR}/${construct}/include)
endforeach()

# Build each construct
add_subdirectory(arith)
add_subdirectory(memory)
add_subdirectory(flow)
add_subdirectory(structs)
add_subdirectory(arrays)
add_subdirectory(slices)
add_subdirectory(optionals)
add_subdirectory(errors)
add_subdirectory(test)
add_subdirectory(enums)
add_subdirectory(unions)
