file(REMOVE_RECURSE
  "libCotTest.a"
  "libCotTest.pdb"
)

# Per-language clean rules from dependency scanning.
foreach(lang CXX)
  include(CMakeFiles/CotTest.dir/cmake_clean_${lang}.cmake OPTIONAL)
endforeach()
