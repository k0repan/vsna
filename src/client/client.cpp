#include "client.h"
#include "helper.h"

void Client::print() const
{
	std::cout << _config.toString() << std::endl;
}

void Client::showPath(ARG_VECTOR args) const
{
	std::cout << "Server path: " << _config.getPath() << std::endl;
}

void Client::myPath(ARG_VECTOR args) const
{
	std::cout << "Current path: " << _config.getPath() << std::endl;
}

void Client::sendFiles(ARG_VECTOR args)
{
	std::cout << "Sending files..." << std::endl;
}

void Client::download(ARG_VECTOR args)
{
	std::cout << "Downloading..." << std::endl;
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
			std::cerr << "Usage: connect [ip:port]" << std::endl;
			return;
		}
		auto tempVec = split(args[0], ":");
		if (tempVec.size() != 2)
		{
			std::cerr << "Usage: connect [ip:port]" << std::endl;
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
	std::cout << "Connected. Type messages to send (type 'quit' to disconnect):\n";
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
	std::cout << "Sending message..." << std::endl;
	std::string msg = join(args, " ");
	std::cout << "Message: " << msg << std::endl;
	std::cout << "Message length: " << msg.length() << std::endl;
}

void Client::disconnect()
{
	std::cout << "Disconnecting..." << std::endl;
	_io_context.stop();

	// Wait for io thread to finish
	if (_io_thread.joinable())
	{
		_io_thread.join();
	}

	_session.reset();
}
