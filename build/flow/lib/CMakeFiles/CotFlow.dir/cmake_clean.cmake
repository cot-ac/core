file(REMOVE_RECURSE
  "libCotFlow.a"
  "libCotFlow.pdb"
)

# Per-language clean rules from dependency scanning.
foreach(lang CXX)
  include(CMakeFiles/CotFlow.dir/cmake_clean_${lang}.cmake OPTIONAL)
endforeach()
