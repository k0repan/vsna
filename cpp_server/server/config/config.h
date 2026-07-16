#pragma once
#include <string>
#include <fstream>
#include <filesystem>
#include <sstream>
#include <stdexcept>
#include <nlohmann/json.hpp>

#define STRING_ARG const std::string&

using json = nlohmann::json;

class Config {
public:
    uint8_t max_clients;
    uint16_t port;
    std::string ip;
    std::string server_path;

    Config()=default;

    static Config loadFromFile(STRING_ARG);
    void setPort(STRING_ARG);
    void setMaxClients(STRING_ARG);
    void setIp(STRING_ARG);
    void setServerPath(STRING_ARG);
    std::string getAddr() const;
};