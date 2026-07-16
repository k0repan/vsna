#include "config.h"

Config::Config(const json& j) {
    from_json(j, *this);
}
    
Config Config::from_file(const std::string& filename) {
    std::fstream file;
    file.open(filename);
    if (!file.is_open()) {
        throw std::runtime_error("[!] Cannot open config file: " + filename);
    }
    
    json j;
    file >> j;
    return Config(j);
}
    
std::string Config::get_addr() const {
    return ip + ":" + std::to_string(port);
}