#include "client.h"

void Client::CLIParse(int argc, char** argv) {
    CLI::App app{ "VSNA" };

    std::string ip{ "0.0.0.0" };
    std::string port{ "5555" };
    std::string path{ "path/to/" };
    std::string configFile;
    
    app.add_option("-i,--ip", ip, "IP address of the server");
    app.add_option("-p,--port", port, "Port of the server");
    app.add_option("-d,--dir", path, "Path to the server");
    app.add_option("-c,--config", configFile, "Path to the config file");

    try {
        app.parse(argc, argv);
    } catch (const CLI::ParseError& e) {
        app.exit(e);
        exit(-1);
    }

    Config _config;

    if (!configFile.empty()) {
        if (std::filesystem::exists(configFile)) {
            try {
                _config = Config::loadFromFile(configFile);
            } catch (const std::exception& e) {
                std::cerr << e.what() << std::endl;
                exit(-1);
            }
        } else {
            std::cerr << "[!] File not found: " << configFile << std::endl;
            exit(-1);
        }
    } else {
        _config = Config::makeNew(Addr(ip, port), path);
    }

    this->config = _config;
}

void Client::runCLI() const{
    std::cout << "Running CLI..." << std::endl;
    std::cout << config.toString() << std::endl;
}
