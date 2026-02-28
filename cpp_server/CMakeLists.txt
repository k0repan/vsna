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
FetchContent_Declare(json URL https://github.com/nlohmann/json/releases/download/v3.12.0/json.tar.xz)
FetchContent_MakeAvailable(json)


# Logger
FetchContent_Declare(spdlog
    GIT_REPOSITORY https://github.com/gabime/spdlog.git
    GIT_TAG v1.12.0
    GIT_SHALLOW TRUE
)

FetchContent_GetProperties(spdlog)

if(NOT spdlog_POPULATED)
    FetchContent_Populate(spdlog)
    
    if(EXISTS ${spdlog_SOURCE_DIR}/CMakeLists.txt)
        set(SPDLOG_BUILD_TESTS OFF CACHE BOOL "" FORCE)
        set(SPDLOG_BUILD_EXAMPLE OFF CACHE BOOL "" FORCE)
        set(SPDLOG_FMT_EXTERNAL OFF CACHE BOOL "" FORCE)
        
        add_subdirectory(${spdlog_SOURCE_DIR} ${spdlog_BINARY_DIR})
    else()
        message(WARNING "spdlog CMakeLists.txt not found, using header-only mode")
        add_library(spdlog INTERFACE)
        target_include_directories(spdlog INTERFACE ${spdlog_SOURCE_DIR}/include)
    endif()
endif()


# Linker
file(GLOB_RECURSE SOURCES "server/*.cpp")
file(GLOB_RECURSE HEADERS "server/*.h" "server/*.hpp")

add_executable(${PROJECT_NAME} ${SOURCES} ${HEADERS})

target_include_directories(${PROJECT_NAME} PRIVATE include)
target_include_directories(${PROJECT_NAME} PRIVATE ${Boost_INCLUDE_DIRS})

target_link_libraries(${PROJECT_NAME} PRIVATE
${Boost_LIBRARIES}
nlohmann_json::nlohmann_json
spdlog::spdlog
)

