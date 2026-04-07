file(REMOVE_RECURSE
  "libCotArrays.a"
  "libCotArrays.pdb"
)

# Per-language clean rules from dependency scanning.
foreach(lang CXX)
  include(CMakeFiles/CotArrays.dir/cmake_clean_${lang}.cmake OPTIONAL)
endforeach()
