file(REMOVE_RECURSE
  "libCotStructs.a"
  "libCotStructs.pdb"
)

# Per-language clean rules from dependency scanning.
foreach(lang CXX)
  include(CMakeFiles/CotStructs.dir/cmake_clean_${lang}.cmake OPTIONAL)
endforeach()
