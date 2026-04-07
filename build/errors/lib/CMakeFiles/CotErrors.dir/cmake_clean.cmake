file(REMOVE_RECURSE
  "libCotErrors.a"
  "libCotErrors.pdb"
)

# Per-language clean rules from dependency scanning.
foreach(lang CXX)
  include(CMakeFiles/CotErrors.dir/cmake_clean_${lang}.cmake OPTIONAL)
endforeach()
