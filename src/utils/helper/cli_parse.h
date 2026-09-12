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
        std::string configFile = argv[0];
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