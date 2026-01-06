#include <iostream>
#include <asio.hpp>
#include <ctime>

using namespace asio;

// A function that return a current daytime as a string
std::string make_daytime_string() {
    std::time_t now = time(0);
    return ctime(&now);
}

int main() {
    std::uint16_t port = 15001;
    io_context io_context; // Main input/output component
    ip::udp::endpoint receiver(ip::udp::v4(), port);
    ip::udp::socket socket(io_context, receiver);
    
    if (socket.is_open()) {
        std::cout << "Server started on address: " << receiver.address().to_string() << ':' << receiver.port() << std::endl;
    }
    
    try {
        while (true) {
            std::array<char, 128> buf; // A message buffer
            ip::udp::endpoint sender; // Saving a sender endpoint data
            std::size_t bytes_transferred = 
                socket.receive_from(buffer(buf), sender); // Listening messages 
            std::cout << "Got message!\n"; 

            std::string message = make_daytime_string(); // Creating an answer
            std::error_code ignored_error; // To catch an error code
            socket.send_to(buffer(message, 128), sender, 0, ignored_error); // Send back to sender 
        }
    }
    catch (std::exception& e) {
        std::cerr << e.what() << std::endl;
    }
    
    return 0;
}