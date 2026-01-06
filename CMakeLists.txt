cmake_minimum_required(VERSION 3.10)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED TRUE)

set(LIB_NAME openvpn3)

project("VSNA" VERSION 1.0.0 LANGUAGES CXX)

add_executable(server src/server.cpp)
add_executable(client src/client.cpp)

include(FetchContent)

target_include_directories(server PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/asio-1.36.0/include)
target_include_directories(client PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/asio-1.36.0/include)
