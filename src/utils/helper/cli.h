#pragma once
#include <iostream>
#include <string>
#include <vector>
#include "helper.h"
#include "types.h"

namespace CLI
{
// split str in name and args
inline std::pair<std::string, STRING_VECTOR> parseArgs(STRING_ARG input)
{
    STRING_VECTOR args = split(input);
    return { args[0], STRING_VECTOR(args.begin() + 1, args.end()) };
}

inline Config CLIParse(int argc, char **argv)
{
    std::cout << argc << std::endl;
	if (argc < 3)
	{
		std::cerr << "-c,--config flag is required.";
		return Config();
	}
	std::string arg1 = argv[1];
	if (arg1 != "-c" && arg1 != "--config")
	{
		TUI::print_err("Wrong flag! Only -c,--config is valid.");
		exit(-1);
	}
	std::string configFile = argv[2];
	std::cout << configFile << std::endl;
	Config config;
	if (std::filesystem::exists(configFile))
	{
		try
		{
			config = Config::loadFromFile(configFile);
		}
		catch (const std::exception& e)
		{
			std::cerr << e.what() << std::endl;
			exit(-1);
		}
	}
	else
	{
		std::cerr << "File not found: " << configFile << std::endl;
		exit(-1);
	}
	return config;
}

inline constexpr void print(STRING_ARG str)
{
    std::cout << str << std::endl;
}

}
