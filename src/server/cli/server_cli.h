#pragma once
#include <boost/asio.hpp>
#include <iostream>
#include <memory>
#include <string>
#include <filesystem>

#include "config.h"
#include "server.h"
#include "helper.h"
#include "cli_parse.h"

class ServerCLI {
  public:
	ServerCLI() = default;
	void run(char **argv);

  private:
	std::shared_ptr<Server> _server{ std::make_shared<Server>() };
};