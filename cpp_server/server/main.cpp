#include <iostream>
#include <boost/beast/websocket.hpp>
#include <boost/beast/core.hpp>
#include "config.h"

/*
cmake -G Ninja -DCMAKE_BUILD_TYPE=Debug -DCMAKE_C_COMPILER="cl.exe" -DCMAKE_CXX_COMPILER="cl.exe" -DMSVC_TOOLSET_VERSION=143 -B build -DCMAKE_EXPORT_COMPILE_COMMANDS=On
*/

int main() {
    try {
        Config config = Config::from_file();
        
        auto const address = boost::asio::ip::make_address(config.address);
        auto const port = static_cast<unsigned short>(std::stoi(config.port));

        boost::asio::io_context ioc{1};
        boost::asio::ip::tcp::acceptor acceptor{ioc};

        std::cout << "Server started on " << config.get_addr() << std::endl;
        std::cout << "Max clients: " << config.max_clients << std::endl;
        std::cout << "Max size: " << config.max_size << std::endl;

    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
        return 1;
    }

    return 0;
}