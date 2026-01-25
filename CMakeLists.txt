cmake_minimum_required(VERSION 3.30)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED TRUE)

option(BOOST_USE_FETCHCONTENT 
    "Всегда использовать FetchContent для Boost (перед сменой необходимо удалить кэш в папке out/build)"
    OFF
)
set(BOOST_FOUND FALSE)

project("VSNA" VERSION 1.0.0 LANGUAGES CXX)

file(GLOB_RECURSE SRV_SRCS "server/srcs/*.cpp")
file(GLOB_RECURSE SRV_HDRS "server/hdrs/*.h")

add_executable(server ${SRV_SRCS} ${SRV_HDRS})
add_executable(client client/client.cpp)


if(NOT BOOST_USE_FETCHCONTENT)
    find_package(Boost REQUIRED COMPONENTS filesystem QUIET)

    if(Boost_FOUND)
        message(STATUS "Boost ver: ${Boost_VERSION}")
        message(STATUS "Bosst include dirs: ${Boost_INCLUDE_DIRS}")
        message(STATUS "Boost library dirs: ${Boost_LIBRARY_DIRS}")
        set(BOOST_FOUND TRUE)
    else()
        message(WARNING "Boost устанавливается при помощи FetchContent...")
    endif()
endif()

if(NOT BOOST_FOUND OR BOOST_USE_FETCHCONTENT)
    include(FetchContent)

    function(include_dependency lib_name git_url git_tag)
        FetchContent_Declare(${lib_name}
            GIT_REPOSITORY ${git_url}
            GIT_TAG        ${git_tag}
            GIT_SHALLOW    TRUE
            GIT_PROGRESS   TRUE
        )
        FetchContent_MakeAvailable(${lib_name})
    endfunction()

    include_dependency(boost https://github.com/boostorg/boost.git boost-1.90.0)

    add_library(Boost::filesystem INTERFACE IMPORTED)
    target_link_libraries(Boost::filesystem INTERFACE boost_filesystem)
endif()


target_include_directories(server PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/server/hdrs)
target_include_directories(server PRIVATE ${Boost_INCLUDE_DIRS})
target_link_libraries(server PRIVATE
    Boost::filesystem
)
