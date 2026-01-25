#include <iostream>
#include "listener.h"


int main(int argc, char* argv[]) {
    if (argc != 4) {
        std::cerr << "Usage: " << argv[0] << " <address> <port> <threads>" << std::endl;
        return 1; 
    }
    
    auto const address = net::ip::make_address(argv[1]);
    auto const port = static_cast<unsigned short>(std::atoi(argv[2]));
    auto const threads = std::max<int>(1, std::atoi(argv[3]));

    net::io_context ioc{threads};
    std::shared_ptr<listener> listener_obj = std::make_shared<listener>(ioc, tcp::endpoint(address, port));
    listener_obj->run();

    std::vector<std::thread> v;
    v.reserve(threads - 1);
    for (auto i = threads - 1; i > 0; --i) {
        v.emplace_back([&ioc]{
            ioc.run();
        });
    }
    ioc.run();

    std::cout << "Threads started successfully!\n";

    return 0;
}