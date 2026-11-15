#include "client_cli.h"

void ClientCLI::run(int argc, char **argv)
{
	this->_client.setConfig(CLI::CLIParse(argc, argv));
	_commandManager.initCommands();
}

bool ClientCLI::invoke(STRING_ARG input)
{
	const std::string trimmed = trim(input);
	if (trimmed.empty())
		return true;
	if (trimmed[0] == '/')
	{
		auto [name, cmdArgs] = CLI::parseArgs(trimmed.substr(1));
		if (name.empty())
			return true;
		return _commandManager.execute(name, cmdArgs);
	}
	if (_client.isConnected())
		_client.sendMsg(ARG_VECTOR{ trimmed });
	else
		TUI::print_err("Not connected. Use /connect [ip:port] first.");
	return false;
}
