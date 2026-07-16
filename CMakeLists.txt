cmake_minimum_required(VERSION 3.16)
project(VSNA VERSION 1.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

if(POLICY CMP0167)
    cmake_policy(SET CMP0167 OLD)
endif()

# Boost
set(Boost_USE_STATIC_LIBS ON)
set(BOOST_ROOT C:/local/boost_1_90_0)

find_package(Boost)
if(Boost_FOUND)
    message(STATUS "Boost found: ${Boost_VERSION}")
    message(STATUS "Boost include dir: ${Boost_INCLUDE_DIRS}")
    message(STATUS "Boost libraries: ${Boost_LIBRARIES}")
else()
    message(FATAL_ERROR "Boost not found!")
endif()

include(FetchContent)

# JSON
FetchContent_Declare(
  json
  GIT_REPOSITORY https://github.com/nlohmann/json
  GIT_TAG v3.12.0
)
FetchContent_MakeAvailable(json)

set(INCLUDE_DIRS
    config/
)

set(LIB_SOURCES
    config/config.cpp
)

set(LINK_LIBRARIES
    ${Boost_LIBRARIES}
    nlohmann_json::nlohmann_json
)

# Core
add_library(core STATIC ${LIB_SOURCES})
target_link_libraries(core PRIVATE ${LINK_LIBRARIES})
target_include_directories(core PRIVATE ${INCLUDE_DIRS})

# Server
add_executable(
    server
    server/main.cpp
    server/server.cpp
)
target_link_libraries(server PRIVATE
    core
    ${LINK_LIBS}
)
target_include_directories(server PRIVATE 
    ${INCLUDE_DIRS}
    ${CMAKE_CURRENT_SOURCE_DIR}
)