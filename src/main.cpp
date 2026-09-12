#include <boost/asio.hpp>
#include <boost/beast/core.hpp>
#include <boost/beast/websocket.hpp>
#include <cstdlib>
#include "tui.h"

int main(int argc, char *argv[])
{
	TuiApp tuiApp;
	tuiApp.run(argv);

	return EXIT_SUCCESS;
}
