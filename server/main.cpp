#include "server.h"


int main(int argc, char* argv[]) {
    try {
        const Config config = Config::load_from_file("");
        start_server(config);
    } catch (const std::exception& e) {
        std::cerr << e.what() << std::endl;
        return 1;
    }
    return 0;
}