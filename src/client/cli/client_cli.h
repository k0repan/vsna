#pragma once
#include <boost/asio.hpp>
#include <libs/CLI11.hpp>
#include <memory>
#include <string>
#include <unordered_map>

#include "client.h"
#include "com_manager.h"
#include "config.h"
#include "command.h"
#include "helper.h"
#include "tui_output.h"

class ClientCLI {
	Client _client;
	CommandManager _commandManager;

  public:
	ClientCLI() : _commandManager(_client)
	{}
	void CLIParse(int argc, char **argv);
	void run(int argc, char **argv);
	bool execute(STRING_ARG input);
};
