#pragma once
#include <iostream>
#include <string>
#include <CLI/CLI.hpp>
#include "config.h"
#include "menu.h"

class Client {
private:
    Config config;
    Menu menu;
        
public:
    Client() : config(Config()),
                menu(Menu(config)) {}
    void CLIParse(int argc, char** argv);
    void startCLI() const;
};