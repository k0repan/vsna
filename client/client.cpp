#include <boost/beast/core.hpp>
#include <boost/beast/websocket.hpp>
#include <boost/asio/connect.hpp>
#include <boost/asio/ip/tcp.hpp>
#include <cstdlib>
#include <string>
#include <iostream>


namespace beast = boost::beast;
namespace http = beast::http;
namespace websocket = beast::websocket;
namespace net = boost::asio;
using tcp = boost::asio::ip::tcp;


int main(int argc, char** argv) {
    try {
        if (argc != 4) {
            std::cerr << "Usage: " << argv[0] << " <host> <port> <text>" << std::endl;
            return 1; 
        }
        
        std::string host = argv[1];
        auto const  port = argv[2];
        auto const  text = argv[3];

        net::io_context ioc;
        tcp::resolver resolver(ioc);
        websocket::stream<tcp::socket> ws(ioc);

        beast::flat_buffer buffer;

        auto const results = resolver.resolve(host, port);
        auto ep = net::connect(ws.next_layer(), results);

        host += ":" + std::to_string(ep.port());

        ws.set_option(websocket::stream_base::decorator(
            [](websocket::request_type& req) {
                req.set(http::field::user_agent, std::string(BOOST_BEAST_VERSION_STRING) + " websocket-client-coro");
            }));
        ws.handshake(host, "/");

        // Logic of interaction (echo for now)
        ws.write(net::buffer(std::string(text)));
        ws.read(buffer);
        ws.close(websocket::close_code::normal);

        std::cout << beast::make_printable(buffer.data()) << std::endl;
    } catch (std::exception const& e) {
        std::cerr << "Error: " << e.what() << std::endl;
        return 1;
    }
    return 0;
}