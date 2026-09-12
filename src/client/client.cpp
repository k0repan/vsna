#include "client.h"

void Client::print() const
{
	TUI::print(_config.toString());
}

void Client::showPath(ARG_VECTOR args) const
{
	TUI::print("Server path: " + _config.getPath());
}

void Client::myPath(ARG_VECTOR args) const
{
	TUI::print("Current path: " + _config.getPath());
}

void Client::sendFiles(ARG_VECTOR args)
{
	TUI::print("Sending files...");
}

void Client::download(ARG_VECTOR args)
{
	TUI::print("Downloading...");
}

void Client::connect(ARG_VECTOR args)
{
	Addr addr;
	if (args.empty())
	{
		addr = _config.getAddr();
	}
	else
	{
		if (args.size() != 1)
		{
			TUI::print_err("Usage: connect [ip:port]");
			return;
		}
		auto tempVec = split(args[0], ":");
		if (tempVec.size() != 2)
		{
			TUI::print_err("Usage: connect [ip:port]");
			return;
		}
		addr = Addr(tempVec[0], tempVec[1]);
	}

	// Create session
	_session = std::make_shared<ClientSession>(_io_context);
	_session->run(addr.ip().c_str(), addr.port().c_str());

	// Run io_context in a separate thread
	_io_thread = std::thread([this]() { _io_context.run(); });

	// Handle console input in main thread
	// TODO: Can we use CLI11 here?
	std::string line;
	TUI::print("Connected. Type messages to send (type 'quit' to disconnect):\n");
	while (std::getline(std::cin, line))
	{
		if (line == "quit")
		{
			break;
		}
		if (!line.empty())
		{
			_session->post_write(line);
		}
	}

	// Cleanup
	disconnect();
}

void Client::sendMsg(ARG_VECTOR args)
{
	TUI::print("Sending message...");
	std::string msg = join(args, " ");
	TUI::print("Message: " + msg);
	TUI::print("Message length: " + std::to_string(msg.length()));
}

void Client::disconnect()
{
    TUI::print("Disconnecting...");
	_io_context.stop();

	// Wait for io thread to finish
	if (_io_thread.joinable())
	{
		_io_thread.join();
	}

	_session.reset();
}
