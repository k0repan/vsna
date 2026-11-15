#pragma once
#include <string>
#include <string_view>
#include <vector>

#include "client.h"
#include "config.h"
#include "types.h"
#include "tui_output.h"

struct CommandInfo
{
	std::string name;
	std::string description;
	std::string usage{ "" };
};

class Command {
  protected:
	Client& _client;

  public:
	const CommandInfo _info;

	virtual ~Command() = default;
	Command(Client& client, const CommandInfo& info) : _client(client), _info(info)
	{}
	virtual bool handle(ARG_VECTOR) = 0;
};

class ConnectCommand : public Command {
  public:
	ConnectCommand(Client& client, const CommandInfo& info) : Command(client, info)
	{}
	bool handle(ARG_VECTOR args) override
	{
		_client.connect(args);
		return false;
	};
};

class ShowPathCommand : public Command {
  public:
	ShowPathCommand(Client& client, const CommandInfo& info) : Command(client, info)
	{}
	bool handle(ARG_VECTOR args) override
	{
		_client.showPath(args);
		return false;
	};
};

class MyPathCommand : public Command {
  public:
	MyPathCommand(Client& client, const CommandInfo& info) : Command(client, info)
	{}
	bool handle(ARG_VECTOR args) override
	{
		_client.myPath(args);
		return false;
	};
};

class SendFilesCommand : public Command {
  public:
	SendFilesCommand(Client& client, const CommandInfo& info) : Command(client, info)
	{}
	bool handle(ARG_VECTOR args) override
	{
		_client.sendFiles(args);
		return false;
	};
};

class DownloadCommand : public Command {
  public:
	DownloadCommand(Client& client, const CommandInfo& info) : Command(client, info)
	{}
	bool handle(ARG_VECTOR args) override
	{
		_client.download(args);
		return false;
	};
};

class PrintCommand : public Command {
  public:
	PrintCommand(Client& client, const CommandInfo& info) : Command(client, info)
	{}
	bool handle(ARG_VECTOR args) override
	{
		_client.print();
		return false;
	};
};

class ExitCommand : public Command {
  public:
	ExitCommand(Client& client, const CommandInfo& info) : Command(client, info)
	{}
	bool handle(ARG_VECTOR args) override
	{
		TUI::print("Program was exit.");
		return true;
	}
};

class CommandManager;

class HelpCommand : public Command {
	CommandManager& _manager;

  public:
	HelpCommand(Client& client, const CommandInfo& info, CommandManager& manager)
	    : Command(client, info), _manager(manager)
	{}
	bool handle(ARG_VECTOR args) override;
};

class SendMessageCommand : public Command {
  public:
	SendMessageCommand(Client& client, const CommandInfo& info) : Command(client, info)
	{}
	bool handle(ARG_VECTOR args) override
	{
		_client.sendMsg(args);
		return false;
	};
};