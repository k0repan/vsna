#include <iostream>
#include <asio.hpp>

using namespace asio;

int main(int argc, char* argv[]) {
    try {
        if (argc != 2) {
            std::cerr << "Usage: client <host>" << std::endl;
            return 1;
        }

        asio::io_context io_context;
        ip::udp::resolver resolver(io_context);
        ip::udp::endpoint receiver_endpoint = *resolver.resolve(ip::udp::v4(), argv[1], "daytime").begin();
        ip::udp::endpoint sender_endpoint;
        
        ip::udp::socket socket(io_context);
        socket.open(ip::udp::v4());
        if (socket.is_open()) {
            std::cout << "Socket opened successfully!\n"; 
        }

        std::array<char, 1> send_buf = {{0}};
        std::array<char, 128> recv_buf;

        socket.send_to(buffer(send_buf), receiver_endpoint);
        std::size_t bytes_transferred = socket.receive_from(buffer(recv_buf), sender_endpoint);
        std::cout.write(recv_buf.data(), bytes_transferred);
    }
    catch (std::exception& e) {
        std::cerr << e.what() << std::endl;
        return -1;
    }
}