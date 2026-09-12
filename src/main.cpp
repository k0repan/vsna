#include <boost/asio.hpp>
#include <boost/beast/core.hpp>
#include <boost/beast/websocket.hpp>
#include "tui.h"
#include "tui_output.h"

#include "server_cli.h"
#include "client_cli.h"


int main(int argc, char *argv[])
{
    TuiApp tuiApp;
    tuiApp.run(argc, argv);
    
	//ServerCLI serverCLI;
	//serverCLI.run(argc, argv);

	return EXIT_SUCCESS;
}