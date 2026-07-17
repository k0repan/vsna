#pragma once
#include <string>
#include <fstream>
#include <filesystem>
#include <sstream>
#include <stdexcept>
#include <cstdint>
#include <nlohmann/json.hpp>
#include "addr.h"

using json = nlohmann::json;

class Config {
public:
    Addr addr;
    std::string path;

    Config()=default;

    static Config loadFromFile(STRING_ARG);
    void setAddr(const Addr&);
    void setPath(STRING_ARG);
    std::string getAddr() const;
};