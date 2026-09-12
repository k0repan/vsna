#include "server.h"

void Server::setup_acceptor()
{
	beast::error_code ec;
	tcp::endpoint endpoint(asio::ip::make_address(_config.getAddr().ip()),
	                       _config.getAddr().portNum());

	// Open the acceptor
	_acceptor.open(endpoint.protocol(), ec);
	if (ec)
	{
		fail(ec, "open");
		return;
	}

	// Allow address reuse
	_acceptor.set_option(asio::socket_base::reuse_address(true), ec);
	if (ec)
	{
		fail(ec, "set_option");
		return;
	}

	// Bind to the server address
	_acceptor.bind(endpoint, ec);
	if (ec)
	{
		fail(ec, "bind");
		return;
	}

	// Start listening for connections
	_acceptor.listen(asio::socket_base::max_listen_connections, ec);
	if (ec)
	{
		fail(ec, "listen");
		return;
	}
}

void Server::run()
{
	setup_acceptor();
	do_accept();
	TUI::print("Server is running on " + _config.getAddr().toString());

	_threads.reserve(max_threads - 1);

	for (size_t i = 0; i < max_threads - 1; ++i)
	{
		_threads.emplace_back([this] { _io_context.run(); });
	}

	TUI::print("Main thread started.");
	_io_context.run();
}

void Server::do_accept()
{
	// The new connection gets its own strand
	_acceptor.async_accept(asio::make_strand(_io_context),
	                       beast::bind_front_handler(&Server::on_accept, shared_from_this()));
}

void Server::on_accept(beast::error_code ec, tcp::socket socket)
{
	if (ec)
	{
		fail(ec, "accept");
	}
	else
	{
		TUI::print("Accepted " + socket.remote_endpoint().address().to_string() + ':'
		          + std::to_string(socket.remote_endpoint().port()));

		// Create the session and run it
		std::make_shared<ServerSession>(std::move(socket))->run();
	}

	// Accept another connection
	do_accept();
}