file(REMOVE_RECURSE
  "libCotMemory.a"
  "libCotMemory.pdb"
)

# Per-language clean rules from dependency scanning.
foreach(lang CXX)
  include(CMakeFiles/CotMemory.dir/cmake_clean_${lang}.cmake OPTIONAL)
endforeach()
