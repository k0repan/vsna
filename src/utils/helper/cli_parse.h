#pragma once
#include <string>
#include <vector>
#include "types.h"
#include "helper.h"

namespace CLIParse
{
    // split str in name and args
    inline std::pair<std::string, STRING_VECTOR> parseArgs(STRING_ARG input)
    {
        STRING_VECTOR args = split(input);
        return { args[0], STRING_VECTOR(args.begin() + 1, args.end()) };
    }

    inline Config CLIParse(char **argv)
    {
        std::string arg1 = argv[1];
        if (arg1 != "-c" && arg1 != "--config")
        {
            TUI::print_err("Wrong flag! Only -c,--config is valid.");
            exit(-1);
        }
        std::string configFile = argv[2];
        Config config;
        if (std::filesystem::exists(configFile))
        {
            try
            {
                config = Config::loadFromFile(configFile);
            }
            catch (const std::exception& e)
            {
                TUI::print_err(e.what());
                exit(-1);
            }
        }
        else
        {
            TUI::print_err("File not found: " + configFile);
            exit(-1);
        }
        return config;
    }
}
