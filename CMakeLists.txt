cmake_minimum_required(VERSION 3.30)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED TRUE)

option(BOOST_USE_FETCHCONTENT "Всегда использовать FetchContent для Boost" OFF)
set(BOOST_FOUND FALSE)

project("VSNA" VERSION 1.0.0 LANGUAGES CXX)
file(GLOB_RECURSE SOURCES "srcs/*.cpp")
file(GLOB_RECURSE HEADERS "hdrs/*.h")
add_executable(server ${SOURCES} ${HEADERS})


if(NOT BOOST_USE_FETCHCONTENT)
    find_package(Boost REQUIRED COMPONENTS filesystem)

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
    FetchContent_Declare(
        boost
        GIT_REPOSITORY https://github.com/boostorg/boost.git
        GIT_TAG        boost-1.90.0
        GIT_SHALLOW    TRUE
        GIT_PROGRESS   TRUE
    )
    FetchContent_MakeAvailable(boost)

    if(boost_POPULATED)
        set(Boost_INCLUDE_DIRS ${boost_SOURCE_DIRS})
        
        if(NOT TARGET Boost::filesystem)
            add_library(Boost::filesystem INTERFACE IMPORTED)
            target_link_libraries(Boost::filesystem INTERFACE boost_filesystem)
        endif()
        
        if(NOT TARGET Boost::system)
            add_library(Boost::system INTERFACE IMPORTED)
            target_link_libraries(Boost::system INTERFACE boost_system)
        endif()
    else()
        message(FATAL_ERROR "Не удалось загрузить Boost при помощи FetchContent")
    endif()
endif()


target_include_directories(server PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/hdrs)
target_include_directories(server PRIVATE ${Boost_INCLUDE_DIRS})
target_link_libraries(server PRIVATE
    Boost::filesystem
)
