#include "command.h"
#include "com_manager.h"

bool HelpCommand::handle(ARG_VECTOR args)
{
	auto commands = _manager.listCommands();
	TUI::print("[=] Available commands:");
	for (const auto& cmd : commands)
	{
		TUI::print("\t" + cmd.name + " " + cmd.usage + " - " + cmd.description);
	}
	return false;
}
