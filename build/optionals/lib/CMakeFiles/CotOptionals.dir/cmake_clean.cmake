file(REMOVE_RECURSE
  "libCotOptionals.a"
  "libCotOptionals.pdb"
)

# Per-language clean rules from dependency scanning.
foreach(lang CXX)
  include(CMakeFiles/CotOptionals.dir/cmake_clean_${lang}.cmake OPTIONAL)
endforeach()
