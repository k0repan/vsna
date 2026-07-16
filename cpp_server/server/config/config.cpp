#include "config.h"

Config Config::loadFromFile(STRING_ARG filename) {
    std::ifstream file;
    file.open(filename);
    if (!file.is_open()) {
        throw std::runtime_error("[!] Cannot open config file: " + filename);
    }
    
    json j;
    file >> j;
    file.close();
    
    Config config;

    config.setIp(j["ip"].get<std::string>());
    config.setPort(j["port"].get<std::string>());
    config.setMaxClients(j["max_clients"].get<std::string>());
    config.setServerPath(j["server_path"].get<std::string>());
    
    return config;
}

void Config::setIp(STRING_ARG _ip) {
    std::stringstream ss(_ip);
    std::string part;
    int index = 0;
    
    while (std::getline(ss, part, '.') && index < 4) {
        if (part.empty()) {
            throw std::invalid_argument("[!] Empty IP part at position " + std::to_string(index));
        }
        
        for (char c : part) {
            if (!std::isdigit(static_cast<unsigned char>(c))) {
                throw std::invalid_argument("[!] Non-digit character in IP part: " + part);
            }
        }
        
        int num = std::stoi(part);
        if (num < 0 || num > 255) {
            throw std::out_of_range("[!] IP part out of range (0-255): " + part);
        }
        
        ip[index++] = static_cast<unsigned char>(num);
    }
    
    if (index != 4) {
        throw std::invalid_argument("[!] Invalid IPv4 format: expected 4 parts, got " + 
                                   std::to_string(index) + " in: " + _ip);
    }
    this->ip = _ip;
}

void Config::setPort(STRING_ARG _port) {
    int port_int = std::stoi(_port);
    if (port_int < 0 || port_int > 65535) {
        throw std::runtime_error("[!] Invalid port: " + _port);
    }
    this->port = port_int;
}

void Config::setMaxClients(STRING_ARG _max_clients) {
    int max_clients_int = std::stoi(_max_clients);
    if (max_clients_int < 0 || max_clients_int > 255) {
        throw std::runtime_error("[!] Invalid max_clients: " + _max_clients);
    }
    this->max_clients = max_clients_int;
}

void Config::setServerPath(STRING_ARG _server_path) {
    namespace fs = std::filesystem;
    
    if (_server_path.empty()) {
        throw std::invalid_argument("[!] Server path cannot be empty");
    }
    
    fs::path path(_server_path);
    
    try {
        if (!fs::exists(path)) {
            throw std::runtime_error("[!] Server path does not exist: " + _server_path);
        }
        
        if (!fs::is_directory(path)) {
            throw std::runtime_error("[!] Server path is not a directory: " + _server_path);
        }
        
        auto status = fs::status(path);
        if ((status.permissions() & fs::perms::owner_read) == fs::perms::none) {
            throw std::runtime_error("[!] Server path is not readable: " + _server_path);
        }
        
        this->server_path = fs::canonical(path).string();
    } catch (const fs::filesystem_error& e) {
        throw std::runtime_error("[!] Filesystem error: " + std::string(e.what()));
    }
}

std::string Config::get_addr() const {
    return ip + ":" + std::to_string(port);
}