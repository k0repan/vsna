cmake_minimum_required(VERSION 3.16)
project(vsna VERSION 1.0.0 LANGUAGES CXX)

# Settings
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
if (MSVC)
    add_compile_options(/bigobj)
    add_compile_options(/utf-8)
endif()
if(POLICY CMP0167)
    cmake_policy(SET CMP0167 OLD)
endif()
include(FetchContent)

# Paths
set(PROJECT_ROOT ${CMAKE_CURRENT_SOURCE_DIR})
set(SRC_DIR      ${PROJECT_ROOT}/src)
set(UTILS_DIR    ${SRC_DIR}/utils)
set(CLIENT_DIR   ${SRC_DIR}/client)
set(COMMON_DIR   ${SRC_DIR}/common)
set(TUI_DIR      ${SRC_DIR}/tui)

# Sources
set(UTILS_SOURCES
    ${UTILS_DIR}/addr/addr.cpp
    ${UTILS_DIR}/config/config.cpp
    ${UTILS_DIR}/logger/logger.h
    ${UTILS_DIR}/helper/helper.h
    ${UTILS_DIR}/helper/cli_parse.h
    ${UTILS_DIR}/output/tui_output.cpp
)
set(CLIENT_SOURCES
    ${CLIENT_DIR}/client.cpp
    ${CLIENT_DIR}/session/client_session.cpp
    ${CLIENT_DIR}/cli/client_cli.cpp
    ${CLIENT_DIR}/commands/command/command.cpp
    ${CLIENT_DIR}/commands/manager/com_manager.cpp
    ${CLIENT_DIR}/receiver/receiver.cpp
    ${CLIENT_DIR}/receiver/session/receiver_session.cpp
)
set(TUI_SOURCES
    ${TUI_DIR}/tui.cpp
)

# Main
add_executable(${PROJECT_NAME} ${SRC_DIR}/main.cpp)
set_target_properties(${PROJECT_NAME} PROPERTIES OUTPUT_NAME "${APP_NAME}")

# Boost
set(BOOST_REQUIRED_COMPONENTS
    filesystem
    system
    exception
)
find_package(Boost REQUIRED COMPONENTS ${BOOST_REQUIRED_COMPONENTS})
find_package(Threads REQUIRED)

if (Boost_FOUND)
    if (WIN32)
        add_definitions(-DHAS_BOOST -D_WIN32_WINNT=0x0A00 -DWIN32_LEAN_AND_MEAN)
    endif()
    target_include_directories(${PROJECT_NAME} PRIVATE ${Boost_INCLUDE_DIRS})
    target_link_libraries(${PROJECT_NAME} PRIVATE
        Boost::headers
        ${Boost_LIBRARIES}
        Threads::Threads
    )
    message("-- Boost status: Found")
else()
    message(FATAL_ERROR "-- Boost status: Not found")
endif()

# Utils
add_library(utils STATIC ${UTILS_SOURCES})
target_include_directories(utils PUBLIC
    ${PROJECT_ROOT}
    ${COMMON_DIR}/types
    ${UTILS_DIR}
    ${UTILS_DIR}/output
    ${UTILS_DIR}/addr
    ${UTILS_DIR}/config
    ${UTILS_DIR}/logger
    ${UTILS_DIR}/helper
)
target_link_libraries(utils PUBLIC Boost::headers ${Boost_LIBRARIES})
target_link_libraries(${PROJECT_NAME} PRIVATE utils)

# Compile definitions
# Client
add_library(client STATIC ${CLIENT_SOURCES})
target_link_libraries(client PRIVATE utils)
target_include_directories(client PUBLIC
    ${CLIENT_DIR}
    ${CLIENT_DIR}/cli
    ${CLIENT_DIR}/commands/command
    ${CLIENT_DIR}/commands/manager
    ${CLIENT_DIR}/session
    ${CLIENT_DIR}/receiver
    ${CLIENT_DIR}/receiver/session)

target_link_libraries(${PROJECT_NAME} PRIVATE client)
target_include_directories(${PROJECT_NAME} PRIVATE ${CLIENT_DIR})

# TUI
add_library(tui STATIC ${TUI_SOURCES})
target_link_libraries(tui PRIVATE utils client)
target_include_directories(tui PUBLIC
    ${TUI_DIR}
)

target_link_libraries(${PROJECT_NAME} PRIVATE tui)
target_include_directories(${PROJECT_NAME} PRIVATE ${TUI_DIR})
