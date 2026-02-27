#include <iostream>
#include <thread>
#include <boost/beast/websocket.hpp>
#include <boost/beast/core.hpp>
#include <spdlog/spdlog.h>
#include <spdlog/sinks/basic_file_sink.h>
#include <spdlog/sinks/stdout_color_sinks.h>
#include "config.h"

using tcp = boost::asio::ip::tcp;

void start_server(const Config& config) {

    auto const address = boost::asio::ip::make_address(config.address);
    auto const port = static_cast<unsigned short>(std::stoi(config.port));

    spdlog::info("WebSocket server is listening on {}", config.get_addr());

    boost::asio::io_context ioc { 1 };
    tcp::acceptor acceptor { ioc, {address, port} };

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
        spdlog::set_level(spdlog::level::debug);
        Config config = Config::from_file();
        start_server(config);
    } catch (const std::exception& e) {
        spdlog::error("Error: ", e.what());
        return 1;
    }
    return 0;
}