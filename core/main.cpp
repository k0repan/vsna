#include <iostream>
#include <boost/beast/core.hpp>
#include <boost/beast/websocket.hpp>

int main(int argc, char* argv[]) {
    std::cout << "Hello, World!" << std::endl;
    boost::asio::io_context ioc { 1 };
    return EXIT_SUCCESS;
}