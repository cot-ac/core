file(REMOVE_RECURSE
  "libCotSlices.a"
  "libCotSlices.pdb"
)

# Per-language clean rules from dependency scanning.
foreach(lang CXX)
  include(CMakeFiles/CotSlices.dir/cmake_clean_${lang}.cmake OPTIONAL)
endforeach()
