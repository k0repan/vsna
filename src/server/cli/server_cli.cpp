#include "server_cli.h"

void ServerCLI::run(char **argv)
{
    this->_server->setConfig(CLIParse::CLIParse(argv));
	this->_server->run();
}
