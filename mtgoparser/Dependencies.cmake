include(cmake/CPM.cmake)

# Done as a function so that updates to variables like
# CMAKE_CXX_FLAGS don't propagate out to other
# targets
function(mtgoparser_setup_dependencies)

  # For each dependency, see if it's
  # already been provided to us by a parent project

  if(NOT TARGET glaze::glaze)
    cpmaddpackage("gh:stephenberry/glaze@1.9.1")
  endif()

  if(NOT TARGET fmtlib::fmtlib)
    cpmaddpackage("gh:fmtlib/fmt#10.2.0")
  endif()

  if(NOT TARGET spdlog::spdlog)
    cpmaddpackage(
      NAME
      spdlog
      VERSION
      1.12.0
      GITHUB_REPOSITORY
      "gabime/spdlog"
      OPTIONS
      "SPDLOG_FMT_EXTERNAL ON")
  endif()

  if(NOT TARGET Catch2::Catch2WithMain)
    cpmaddpackage("gh:catchorg/Catch2@3.5.1")
  endif()

  if(NOT TARGET rapidxml::rapidxml)
    cpmaddpackage("gh:CodeFinder2/rapidxml@1.13")
  endif()

  if(NOT TARGET Boost)
    # Boost 1.84 is needed from above
    # With these components: outcome core conversion detail headers
    add_subdirectory(deps-submodules/boost EXCLUDE_FROM_ALL)
  endif()

  if (NOT TARGET tomlplusplus)
    cpmaddpackage(
      NAME tomlplusplus
      VERSION 3.4.0
      GIT_REPOSITORY https://github.com/marzer/tomlplusplus.git
      GIT_TAG        v3.4.0
    )
  endif()

endfunction()
