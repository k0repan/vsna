#pragma once
#include <boost/asio.hpp>
#include <memory>
#include <string>
#include <unordered_map>

#include "client.h"
#include "com_manager.h"
#include "helper.h"
#include "cli.h"

class ClientCLI {
	Client _client;
	CommandManager _commandManager;

  public:
	ClientCLI() : _commandManager(_client)
	{}
	void run(int argc, char **argv);
	bool invoke(STRING_ARG input);
	bool isClientConnected() const
	{
	    return _client.isConnected();
	};
	void disconnectClient()
	{
	    _client.disconnect();
	}
};
