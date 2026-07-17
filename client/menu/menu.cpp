#include "menu.h"

void Menu::buildCommands() {
    commands[ExitCommand::name] = std::make_unique<ExitCommand>(isExit);
    commands[PrintCommand::name] = std::make_unique<PrintCommand>(config);
    commands[ConnectCommand::name] = std::make_unique<ConnectCommand>(config);
    commands[ShowPathCommand::name] = std::make_unique<ShowPathCommand>(config);
    commands[DownloadCommand::name] = std::make_unique<DownloadCommand>(config);
    commands[SendFilesCommand::name] = std::make_unique<SendFilesCommand>(config);
}

void Menu::run() {
    std::string input;
    while (true) {
        std::cout << "> ";
        std::getline(std::cin, input);
        ARG_VECTOR args = splitArgs(input);
        if (args.empty()) continue;
        auto it = commands.find(args[0]);
        if (it != commands.end()) {
            it->second->handle(args);
        } else {
            std::cout << "Unknown command: " << args[0] << std::endl;
        }
        if (isExit) break;
    }
}

ARG_VECTOR splitArgs(STRING_ARG input) {
    ARG_VECTOR args;
    std::stringstream ss(input);
    std::string token;
    while (ss >> token) {
        args.push_back(token);
    }
    return args;
}

std::string toLowerCase(STRING_ARG str){
    std::string result = str;
    std::transform(result.begin(), result.end(), result.begin(), ::tolower);
    return result;
}

void ExitCommand::handle(const ARG_VECTOR& args) {
    isExit = true;
    std::cout << "[~] Programm was exit." << std::endl;
}

void PrintCommand::handle(const ARG_VECTOR& args) {
    std::cout << config.toString() << std::endl;
}

void ConnectCommand::handle(const ARG_VECTOR& args) {
    
}

void ShowPathCommand::handle(const ARG_VECTOR& args) {
    
}

void SendFilesCommand::handle(const ARG_VECTOR& args) {
    
}

void DownloadCommand::handle(const ARG_VECTOR& args) {
    
}
