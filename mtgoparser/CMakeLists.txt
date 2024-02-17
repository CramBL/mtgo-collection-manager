cmake_minimum_required(VERSION 3.28.1) # Boost 1.84.0 requires >= 3.28.2


# Only set the cxx_standard if it is not set by someone else
if (NOT DEFINED CMAKE_CXX_STANDARD)
  set(CMAKE_CXX_STANDARD 20)
endif()
set(CMAKE_CXX_STANDARD_REQUIRED True)

# strongly encouraged to enable this globally to avoid conflicts between
# -Wpedantic being enabled and -std=c++20 and -std=gnu++20 for example
# when compiling with PCH enabled
set(CMAKE_CXX_EXTENSIONS OFF)

project(
   mtgoparser
   VERSION 0.1.0
   DESCRIPTION "Parser library for the MTGO collection manager app"
   HOMEPAGE_URL "https://github.com/CramBL/mtgo-collection-manager/mtgoparser"
   LANGUAGES CXX
)

message(STATUS "Compiler: ${CMAKE_CXX_COMPILER_ID} ${CMAKE_CXX_COMPILER_VERSION}")
include(cmake/PreventInSourceBuilds.cmake)
include(ProjectOptions.cmake)

mtgoparser_setup_options()

mtgoparser_global_options()
include(Dependencies.cmake)
mtgoparser_setup_dependencies()

mtgoparser_local_options()

# don't know if this should be set globally from here or not...
set(CMAKE_CXX_VISIBILITY_PRESET hidden)

set(GIT_SHA
    "Unknown"
    CACHE STRING "SHA this build was generated from")
string(
  SUBSTRING "${GIT_SHA}"
            0
            8
            GIT_SHORT_SHA)

target_compile_features(mtgoparser_options INTERFACE cxx_std_${CMAKE_CXX_STANDARD})


add_library(mtgoparser::mtgoparser_options ALIAS mtgoparser_options)
add_library(mtgoparser::mtgoparser_warnings ALIAS mtgoparser_warnings)
add_subdirectory(configured_files)

# Define the library target
add_library(mtgoparser INTERFACE)
target_include_directories(mtgoparser INTERFACE
$<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}/include>
$<BUILD_INTERFACE:${PROJECT_BINARY_DIR}/include>
$<INSTALL_INTERFACE:include>
)
include(cmake/SystemLink.cmake)
target_link_system_library(${PROJECT_NAME} INTERFACE fmt::fmt)
target_link_system_library(${PROJECT_NAME} INTERFACE glaze::glaze)
target_link_system_library(${PROJECT_NAME} INTERFACE spdlog::spdlog)
target_link_system_library(${PROJECT_NAME} INTERFACE rapidxml)
target_link_system_library(${PROJECT_NAME} INTERFACE tomlplusplus::tomlplusplus)
target_link_system_library(${PROJECT_NAME} INTERFACE Boost::headers)
target_link_system_library(${PROJECT_NAME} INTERFACE Boost::outcome)
target_link_system_library(${PROJECT_NAME} INTERFACE Boost::conversion)
target_link_system_library(${PROJECT_NAME} INTERFACE Boost::unordered)

# To build the binary executable MTGO preprocessor that can do a full parse of all card data files
add_subdirectory(src)

# Don't even look at tests if we're not top level
if(NOT PROJECT_IS_TOP_LEVEL)
  return()
endif()

# Adding the tests:
include(CTest)

if(BUILD_TESTING)
  add_subdirectory(test)
endif()

# If MSVC is being used, and ASAN is enabled, we need to set the debugger environment
# so that it behaves well with MSVC's debugger, and we can run the target from visual studio
if(MSVC)
  get_all_installable_targets(all_targets)
  message("all_targets=${all_targets}")
  set_target_properties(${all_targets} PROPERTIES VS_DEBUGGER_ENVIRONMENT "PATH=$(VC_ExecutablePath_x64);%PATH%")
endif()

# set the startup project for the "play" button in MSVC
set_property(DIRECTORY PROPERTY VS_STARTUP_PROJECT mtgoparser)

if(CMAKE_SKIP_INSTALL_RULES)
  return()
endif()

include(cmake/PackageProject.cmake)

# Add other targets here, by default we just package one executable
mtgoparser_package_project(
 TARGETS
 mtgo_preprocessor
 VERSION 0.1.0
 mtgoparser_options
 mtgoparser_warnings
  #FIXME: this does not work! CK
  #PRIVATE_DEPENDENCIES_CONFIGURED project_options project_warnings
)

# Experience shows that explicit package naming can help make it easier to sort
# out potential ABI related issues before they start, while helping you
# track a build to a specific GIT SHA
set(CPACK_PACKAGE_FILE_NAME
   "${CMAKE_PROJECT_NAME}-${CMAKE_PROJECT_VERSION}-${GIT_SHORT_SHA}-${CMAKE_SYSTEM_NAME}-${CMAKE_BUILD_TYPE}-${CMAKE_CXX_COMPILER_ID}-${CMAKE_CXX_COMPILER_VERSION}"
)

include(InstallRequiredSystemLibraries)
#set(CPACK_RESOURCE_FILE_LICENSE "${CMAKE_CURRENT_SOURCE_DIR}/License.txt")
set(CPACK_SOURCE_GENERATOR "TGZ")
include(CPack)