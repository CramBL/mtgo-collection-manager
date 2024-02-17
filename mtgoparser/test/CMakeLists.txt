cmake_minimum_required(VERSION 3.28.1) # Boost 1.84.0 requires >= 3.28.2

project(mtgoparserTests LANGUAGES CXX)

# ---- Test as standalone project the exported config package ----
if(PROJECT_IS_TOP_LEVEL OR TEST_INSTALLED_VERSION)

  enable_testing()

  find_package(mtgoparser CONFIG REQUIRED) # for project_options, ...

  if(NOT TARGET mtgoparser_options)
    message(FATAL_ERROR "Required config package not found!")
    return()
  endif()
endif()

# ---- Dependencies ----
include(../cmake/SystemLink.cmake)

list(APPEND CMAKE_MODULE_PATH ${Catch2_SOURCE_DIR}/extras)
find_package(Catch2 3 REQUIRED)
include(Catch)
add_executable(test_xml_parse test_xml_parse.cpp)
target_link_libraries(test_xml_parse PRIVATE
                                    mtgoparser::mtgoparser_warnings
                                    mtgoparser::mtgoparser_options
                                    mtgoparser)
target_link_system_library(test_xml_parse PRIVATE Catch2::Catch2WithMain)


if(WIN32 AND BUILD_SHARED_LIBS)
  message(VERBOSE "WIN32 and BUILD_SHARED_LIBS")
  add_custom_command(
    TARGET test_xml_parse
    PRE_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy $<TARGET_RUNTIME_DLLS:test_xml_parse> $<TARGET_FILE_DIR:test_xml_parse>
    COMMAND_EXPAND_LISTS)
endif()


# automatically discover tests that are defined in catch based test files.
catch_discover_tests(
  test_xml_parse
  TEST_PREFIX
  "test_xml_parse."
  REPORTER
  XML
  OUTPUT_DIR
  .
  OUTPUT_PREFIX
  "test_xml_parse."
  OUTPUT_SUFFIX
  .xml)

# Add benchmarks for XML parsing
add_executable(benchmark_xml_parse benchmark_xml_parse.cpp)
target_link_libraries(
  benchmark_xml_parse
  PRIVATE mtgoparser::mtgoparser_warnings
          mtgoparser::mtgoparser_options
          mtgoparser)
target_link_system_library(benchmark_xml_parse PRIVATE Catch2::Catch2WithMain)

if(WIN32 AND BUILD_SHARED_LIBS)
  message(VERBOSE "WIN32 and BUILD_SHARED_LIBS")
  add_custom_command(
    TARGET benchmark_xml_parse
    PRE_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy $<TARGET_RUNTIME_DLLS:benchmark_xml_parse> $<TARGET_FILE_DIR:benchmark_xml_parse>
    COMMAND_EXPAND_LISTS)
endif()


catch_discover_tests(
  benchmark_xml_parse
  TEST_PREFIX
  "benchmark_xml_parse."
  REPORTER
  XML
  OUTPUT_DIR
  .
  OUTPUT_PREFIX
  "benchmark_xml_parse."
  OUTPUT_SUFFIX
  .xml)

# JSON parsing tests
add_executable(test_json_parse test_json_parse.cpp)
target_link_libraries(test_json_parse PRIVATE
                                     mtgoparser::mtgoparser_warnings
                                     mtgoparser::mtgoparser_options
                                     mtgoparser)
target_link_system_library(test_json_parse PRIVATE Catch2::Catch2WithMain)


if(WIN32 AND BUILD_SHARED_LIBS)
  message(VERBOSE "WIN32 and BUILD_SHARED_LIBS")
  add_custom_command(
    TARGET test_json_parse
    PRE_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy $<TARGET_RUNTIME_DLLS:test_json_parse> $<TARGET_FILE_DIR:test_json_parse>
    COMMAND_EXPAND_LISTS)
endif()


# automatically discover tests that are defined in catch based test files.
catch_discover_tests(
  test_json_parse
  TEST_PREFIX
  "test_json_parse."
  REPORTER
  XML
  OUTPUT_DIR
  .
  OUTPUT_PREFIX
  "test_json_parse."
  OUTPUT_SUFFIX
  .xml)

# Misc. tests
add_executable(tests tests.cpp)
target_link_libraries(tests PRIVATE
                                     mtgoparser::mtgoparser_warnings
                                     mtgoparser::mtgoparser_options
                                     mtgoparser)
target_link_system_library(tests PRIVATE Catch2::Catch2WithMain)

catch_discover_tests(
  tests
  TEST_PREFIX
  "tests."
  REPORTER
  XML
  OUTPUT_DIR
  .
  OUTPUT_PREFIX
  "tests."
  OUTPUT_SUFFIX
  .xml)


# Full mtgoparser tests (i.e. all data for a whole collection)
add_executable(test_full_collection_parse test_full_collection_parse.cpp)
target_link_libraries(test_full_collection_parse PRIVATE
                                     mtgoparser::mtgoparser_warnings
                                     mtgoparser::mtgoparser_options
                                     mtgoparser)
target_link_system_library(test_full_collection_parse PRIVATE Catch2::Catch2WithMain)

catch_discover_tests(
  test_full_collection_parse
  TEST_PREFIX
  "test_full_collection_parse."
  REPORTER
  XML
  OUTPUT_DIR
  .
  OUTPUT_PREFIX
  "test_full_collection_parse."
  OUTPUT_SUFFIX
  .xml)

# Tests of CSV parsing functionality
add_executable(test_csv_parse test_csv_parse.cpp)
target_link_libraries(test_csv_parse PRIVATE
                                     mtgoparser::mtgoparser_warnings
                                     mtgoparser::mtgoparser_options
                                     mtgoparser)
target_link_system_library(test_csv_parse PRIVATE Catch2::Catch2WithMain)

catch_discover_tests(
  test_csv_parse
  TEST_PREFIX
  "test_csv_parse."
  REPORTER
  XML
  OUTPUT_DIR
  .
  OUTPUT_PREFIX
  "test_csv_parse."
  OUTPUT_SUFFIX
  .xml)
