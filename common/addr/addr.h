#pragma once
#include <string>
#include <sstream>

#define STRING_ARG const std::string&

class Addr {
public:
    Addr()=default;
    Addr(uint16_t, STRING_ARG);
    Addr(STRING_ARG, STRING_ARG);
    
    uint16_t port() const { return _port; }
    STRING_ARG ip() const { return _ip; }

    void setIp(STRING_ARG);
    void setPort(STRING_ARG);

    const std::string to_string() const {
        return _ip + ":" + std::to_string(_port);
    }
    
private:
    uint16_t _port;
    std::string _ip;
};