#pragma once
#include <iostream>
#include <string>
#include <CLI/CLI.hpp>
#include "config.h"

class Client {
private:
    Config config;
        
public:
    Client()=default;
    void CLIParse(int argc, char** argv);
    void runCLI() const;
};