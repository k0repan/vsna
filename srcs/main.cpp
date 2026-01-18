#include <iostream>
#include "server.h"


int main(int argc, char* argv[]) {
    if (argc != 3) {
        std::cerr << "Usage: " << argv[0] << " <path_to_directory> <port>" << std::endl;
        return 1; 
    }

    fs::path root_path(argv[1]);
    std::uint16_t port = static_cast<std::uint16_t>(std::atoi(argv[2]));

    if (!fs::exists(root_path) || !fs::is_directory(root_path)) {
        std::cerr << "Invalid directory path" << std::endl;
        return 1;
    }

    server srv(root_path, port);
    srv.run_server();
    
    return 0;
}