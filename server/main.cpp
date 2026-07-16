#include <iostream>
#include <thread>
#include <boost/beast/websocket.hpp>
#include <boost/beast/core.hpp>
#include "config.h"

using tcp = boost::asio::ip::tcp;

void start_server(const Config& config) {

    auto const ip = boost::asio::ip::make_address(config.ip);
    auto const port = config.port;

    boost::asio::io_context ioc { 1 };
    tcp::acceptor acceptor { ioc, {ip, port} };

    while (1) {
        tcp::socket socket { ioc };
        acceptor.accept(socket);

        std::thread{[q = std::move(socket)]() mutable {

            boost::beast::websocket::stream<tcp::socket> ws {std::move(q)};
            ws.accept();

            while(1) {
                boost::beast::flat_buffer buffer;
                ws.read(buffer);
                auto out = boost::beast::buffers_to_string(buffer.cdata());
                std::cout << out << std::endl;
            }

        }}.detach();
    }
}


int main() {
    try {
        const Config config = Config::load_from_file("");
        start_server(config);
    } catch (const std::exception& e) {
        std::cerr << e.what() << std::endl;
        return 1;
    }
    return 0;
}