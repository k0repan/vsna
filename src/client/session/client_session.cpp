#include "client_session.h"

void ClientSession::run(char const *host, char const *port)
{
	_host = host;
	_port = port;

	_resolver.async_resolve(
	    host, port, beast::bind_front_handler(&ClientSession::on_resolve, shared_from_this()));
}

void ClientSession::on_resolve(beast::error_code ec, tcp::resolver::results_type results)
{
	if (ec)
		return fail(ec, "resolve");

	// Set the timeout for the operation
	beast::get_lowest_layer(_ws).expires_after(std::chrono::seconds(30));

	// Make the connection on the IP address we get from a lookup
	beast::get_lowest_layer(_ws).async_connect(
	    results, beast::bind_front_handler(&ClientSession::on_connect, shared_from_this()));
}

void ClientSession::on_connect(beast::error_code ec, tcp::resolver::results_type::endpoint_type ep)
{
	if (ec)
		return fail(ec, "connect");

	// Turn off the timeout on the tcp_stream, because
	// the websocket stream has its own timeout system.
	beast::get_lowest_layer(_ws).expires_never();

	// Set suggested timeout settings for the websocket
	_ws.set_option(websocket::stream_base::timeout::suggested(beast::role_type::client));

	// Set a decorator to change the User-Agent of the handshake
	_ws.set_option(websocket::stream_base::decorator([](websocket::request_type& req) {
		req.set(beast::http::field::user_agent,
		        std::string(BOOST_BEAST_VERSION_STRING) + " websocket-client-async");
	}));

	// Perform the websocket handshake
	_ws.async_handshake(
	    _host, "/", beast::bind_front_handler(&ClientSession::on_handshake, shared_from_this()));
}

void ClientSession::on_handshake(beast::error_code ec)
{
	if (ec)
		return fail(ec, "handshake");

	// TODO: Kepp connect live
	// Legacy:
	// _ws.async_write(asio::buffer(_text),
	// beast::bind_front_handler(&ClientSession::on_write, shared_from_this()));
	// _ws.async_read(_buffer, beast::bind_front_handler(&ClientSession::on_read,
	// shared_from_this()));
	TUI::print("Connected to " + _host + ":" + _port);
	do_read();
	do_write();
	TUI::print("Client session started.");
}

void ClientSession::do_read()
{
	_ws.async_read(
	    _buffer, asio::bind_executor(_ws.get_executor(), [self = shared_from_this()](
	                                                         beast::error_code ec,
	                                                         size_t bytes_transferred) {
		    if (ec)
		    {
			    TUI::print_err("Read error: " + beast::buffers_to_string(self->_buffer.data()));
			    self->_ws.async_close(
			        websocket::close_code::normal,
			        beast::bind_front_handler(&ClientSession::on_close, self->shared_from_this()));
			    return;
		    }
		    TUI::print("Received: " + beast::buffers_to_string(self->_buffer.data()));
		    self->_buffer.consume(self->_buffer.size());

		    self->do_read();
	    }));
}

void ClientSession::post_write(std::string msg)
{
	asio::post(asio::bind_executor(_ws.get_executor(),
	                               [self = shared_from_this(), msg = std::move(msg)]() {
		                               self->enqueue_write(std::move(msg));
	                               }));
}

void ClientSession::enqueue_write(std::string msg)
{
	_write_queue.push(std::move(msg));
	if (!_is_writing)
	{
		do_write();
	}
}

void ClientSession::do_write()
{
	if (_write_queue.empty())
	{
		_is_writing = false;
		return;
	}

	_is_writing = true;
	_current_msg = std::move(_write_queue.front());
	_write_queue.pop();

	_ws.async_write(
	    asio::buffer(_current_msg),
	    asio::bind_executor(
	        _ws.get_executor(),
	        [self = shared_from_this()](beast::error_code ec, size_t bytes_transferred) {
		        if (ec)
		        {
			        TUI::print_err("Write error: " + ec.message());
			        self->_ws.async_close(websocket::close_code::normal,
			                              beast::bind_front_handler(&ClientSession::on_close,
			                                                        self->shared_from_this()));
			        return;
		        }

		        TUI::print("Bytes written: " + std::to_string(bytes_transferred));
		        self->_current_msg.clear();
		        self->do_write();
	        }));
}

void ClientSession::on_read(beast::error_code ec, std::size_t bytes_transferred)
{
	if (ec)
		return fail(ec, "read");

	boost::ignore_unused(bytes_transferred);

	// Process the response from the server
	if (ec == websocket::error::closed)
		return;

	if (ec)
		return fail(ec, "read");

	TUI::print("Received: " + beast::buffers_to_string(_buffer.data()));

	// Echo the message back (TODO: Implement actual message handling logic)
	_ws.text(_ws.got_text());
	_ws.async_write(_buffer.data(),
	                beast::bind_front_handler(&ClientSession::on_write, shared_from_this()));
}

void ClientSession::on_write(beast::error_code ec, std::size_t bytes_transferred)
{
	if (ec)
		return fail(ec, "write");

	boost::ignore_unused(bytes_transferred);

	// Read a message into the buffer
	_ws.async_read(_buffer, beast::bind_front_handler(&ClientSession::on_read, shared_from_this()));
	this->do_read();
}

void ClientSession::on_close(beast::error_code ec)
{
	// Usage:
	// _ws.async_close(websocket::close_code::normal,
	//                 beast::bind_front_handler(&ClientSession::on_close, shared_from_this()));
	if (ec)
		return fail(ec, "close");

	// TODO: Add a normal close
	TUI::print("Closed: " + beast::buffers_to_string(_buffer.data()));
}
