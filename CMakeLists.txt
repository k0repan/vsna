cmake_minimum_required(VERSION 3.16)
project(VSNA VERSION 1.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

if(POLICY CMP0167)
    cmake_policy(SET CMP0167 OLD)
endif()



include(FetchContent)

# JSON
FetchContent_Declare(
  json
  GIT_REPOSITORY https://github.com/nlohmann/json
  GIT_TAG v3.12.0
)
FetchContent_MakeAvailable(json)

# CLI
FetchContent_Declare(
  cli11
  GIT_REPOSITORY https://github.com/CLIUtils/CLI11
  GIT_TAG v2.6.2
)
FetchContent_MakeAvailable(cli11)

set(INCLUDE_DIRS
    common/addr/
    common/config/
)

set(LIB_SOURCES
    common/addr/addr.cpp
    common/config/config.cpp
)

set(LINK_LIBRARIES
    ${Boost_LIBRARIES}
    nlohmann_json::nlohmann_json
    CLI11::CLI11
)

# Core
add_library(core STATIC ${LIB_SOURCES})
target_link_libraries(core PUBLIC ${LINK_LIBRARIES})
target_include_directories(core PUBLIC ${INCLUDE_DIRS})

# Server
#add_executable(
#    server
#    server/main.cpp
#    server/server.cpp
#)
#target_link_libraries(server PRIVATE
#    core
#    ${LINK_LIBS}
#)
#target_include_directories(server PRIVATE 
#    ${INCLUDE_DIRS}
#    ${CMAKE_CURRENT_SOURCE_DIR}
#)

# Client
add_executable(
    client
    client/main.cpp
    client/client.cpp
    client/menu/menu.cpp
)
target_link_libraries(client PRIVATE
    core
)
target_include_directories(client PRIVATE 
    client/menu/
    ${INCLUDE_DIRS}
    ${CMAKE_CURRENT_SOURCE_DIR}
)