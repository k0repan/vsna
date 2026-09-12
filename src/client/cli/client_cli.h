#pragma once
#include <boost/asio.hpp>
#include <memory>
#include <string>
#include <unordered_map>

#include "client.h"
#include "com_manager.h"
#include "helper.h"
#include "cli_parse.h"

class ClientCLI {
	Client _client;
	CommandManager _commandManager;

  public:
	ClientCLI() : _commandManager(_client)
	{}
	void run(char **argv);
	bool invoke(STRING_ARG input);
};
