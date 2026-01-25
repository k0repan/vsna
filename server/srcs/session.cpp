#include "session.h"
#include <iostream>

// An error handling function
void fail(beast::error_code ec, char const* what) {
    std::cerr << what << ": " << ec.message() << "\n";
}

// Initializing websocket
session::session(tcp::socket&& socket) : _ws(std::move(socket)) {}

// Get the correct executor
void session::run() {
    net::dispatch(
        _ws.get_executor(),
        beast::bind_front_handler(&session::on_run, shared_from_this())
    );
}

// Start the asynchronous operation 
void session::on_run() {
    _ws.set_option(websocket::stream_base::timeout::suggested(
        beast::role_type::server
    ));
    _ws.set_option(websocket::stream_base::decorator(
        [](websocket::response_type& res){
            res.set(http::field::server, std::string(BOOST_BEAST_VERSION_STRING) + " websocket-server-async");
        }
    ));
    _ws.async_accept(beast::bind_front_handler(&session::on_accept, shared_from_this()));
}

// Accept the handshake
void session::on_accept(beast::error_code ec) {
    if (ec) return fail(ec, "accept");
    do_read();
}

// Read the message
void session::do_read() {
    _ws.async_read(_buf, beast::bind_front_handler(&session::on_read, shared_from_this()));
};

// Processes the message and sends the answer
void session::on_read(beast::error_code ec, size_t bytes_transferred) {
    boost::ignore_unused(bytes_transferred);

    if (ec == websocket::error::closed) return;
    if (ec) fail(ec, "read");

    // Echoes the message
    _ws.text(_ws.got_text());
    _ws.async_write(_buf.data(), beast::bind_front_handler(&session::on_write, shared_from_this()));
}

// Processes the answer
void session::on_write(beast::error_code ec, size_t bytes_transferred) {
    boost::ignore_unused(bytes_transferred);
    
    if (ec) fail(ec, "write");
    _buf.consume(_buf.size());
    do_read();
}