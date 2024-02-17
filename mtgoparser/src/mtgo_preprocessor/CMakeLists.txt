find_package(fmt)
# Boost 1.84 is needed from above
# With these components: outcome core conversion detail headers

add_executable(mtgo_preprocessor main.cpp setup.cpp run.cpp)

target_link_libraries(
  mtgo_preprocessor
  PRIVATE mtgoparser::mtgoparser_options
          mtgoparser::mtgoparser_warnings
          )

target_link_system_libraries(
  mtgo_preprocessor
  PRIVATE glaze::glaze
          fmt::fmt
          spdlog::spdlog
          rapidxml
          mtgoparser
          Boost::headers
          Boost::outcome
          Boost::conversion
          Boost::core
          Boost::detail
          )

target_include_directories(mtgo_preprocessor PRIVATE "${CMAKE_BINARY_DIR}/configured_files/include")
include(../../cmake/Utilities.cmake)
copy_runtime_dlls(mtgo_preprocessor)
