cmake_minimum_required(VERSION 3.16)
project(vsna LANGUAGES CXX)

set(PROJECT_ROOT "${CMAKE_CURRENT_SOURCE_DIR}")
set(PROJECT_DEFINITIONS)

set(BOOST_REQUIRED_COMPONENTS
    filesystem
    optional
    asio
    beast
)

file(GLOB_RECURSE CORE_SRCS "${PROJECT_ROOT}/core/*.cpp")
file(GLOB_RECURSE CORE_HDRS "${PROJECT_ROOT}/core/*.h")

add_executable(${PROJECT_NAME} ${CORE_SRCS} ${CORE_HDRS})

find_package(Boost REQUIRED COMPONENTS ${BOOST_REQUIRED_COMPONENTS})
if (Boost_FOUND)
    include_directories(${Boost_INCLUDE_DIR})
    add_definitions("-DHAS_BOOST")
endif()

target_link_libraries(${PROJECT_NAME} PRIVATE 
    Boost::filesystem
    Boost::system
    Boost::asio
    Boost::beast
)