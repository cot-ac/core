file(REMOVE_RECURSE
  "libCotArith.a"
  "libCotArith.pdb"
)

# Per-language clean rules from dependency scanning.
foreach(lang CXX)
  include(CMakeFiles/CotArith.dir/cmake_clean_${lang}.cmake OPTIONAL)
endforeach()
