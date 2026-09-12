#include "client_cli.h"

void ClientCLI::run(char **argv)
{
    this->_client.setConfig(CLIParse::CLIParse(argv));
	_commandManager.initCommands();
}

bool ClientCLI::invoke(STRING_ARG input)
{
	auto [name, cmdArgs] = CLIParse::parseArgs(input);

	if (name.empty())
		return true;

	return _commandManager.execute(name, cmdArgs);
}

