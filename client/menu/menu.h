#pragma once
#include <unordered_map>
#include <memory>
#include <string>
#include <vector>
#include <iostream>
#include <algorithm>
#include <sstream>
#include "config.h"

using ARG_VECTOR = std::vector<std::string>;
using STRING_ARG = const std::string&;

class MenuItem{
public:
    static constexpr const char* name=nullptr;
    virtual ~MenuItem()=default;
    virtual void handle(const ARG_VECTOR&)=0;
};

class ConnectCommand : public MenuItem {
public:
    const Config& config;
    static constexpr const char* name { "connect" };
    ConnectCommand(const Config& config) : config(config) {}
    void handle(const ARG_VECTOR&) override;
};

class ShowPathCommand : public MenuItem {
public:
    const Config& config;
    static constexpr const char* name { "path" };
    ShowPathCommand(const Config& config) : config(config) {}
    void handle(const ARG_VECTOR&) override;
};

class SendFilesCommand : public MenuItem {
public:
    const Config& config;
    static constexpr const char* name { "send" };
    SendFilesCommand(const Config& config) : config(config) {}
    void handle(const ARG_VECTOR&) override;
};

class DownloadCommand : public MenuItem {
public:
    const Config& config;
    static constexpr const char* name { "download" };
    DownloadCommand(const Config& config) : config(config) {}
    void handle(const ARG_VECTOR&) override;
};

class ExitCommand : public MenuItem {
public:
    bool& isExit;
    static constexpr const char* name { "exit" };
    ExitCommand(bool& exitFlag) : isExit(exitFlag) {}
    void handle(const ARG_VECTOR&) override;
};

class PrintCommand : public MenuItem {
public:
    const Config& config;
    static constexpr const char* name { "print" };
    PrintCommand(const Config& config) : config(config) {}
    void handle(const ARG_VECTOR&) override;
};


class Menu {
private:
    const Config& config{ Config(Addr("0.0.0.0", "8080"), ".") };
    bool isExit{ false };
    std::unordered_map<std::string, std::unique_ptr<MenuItem>> commands;
public:
    Menu()=default;
    Menu(const Config& config) : config(config) { this->buildCommands(); }
    ~Menu()=default;
    
    void buildCommands();
    void run();
};

ARG_VECTOR splitArgs(STRING_ARG input);
std::string toLowerCase(STRING_ARG str);