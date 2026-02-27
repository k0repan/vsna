#pragma once
#include <string>
#include <fstream>
#include <stdexcept>
#include <nlohmann/json.hpp>

using json = nlohmann::json;

class Config {
public:
    std::string address;
    std::string port;
    std::string server_path;
    uint16_t max_size;
    uint16_t max_clients;

    Config() = default;

    explicit Config(const json& j);
    static Config from_file(const std::string& filename = "..\\server\\__config__.json");
    std::string get_addr() const;

    friend void from_json(const json& j, Config& config) {
        j.at("address").get_to(config.address);
        j.at("port").get_to(config.port);
        j.at("server_path").get_to(config.server_path);
        j.at("max_size").get_to(config.max_size);
        j.at("max_clients").get_to(config.max_clients);
    }
};