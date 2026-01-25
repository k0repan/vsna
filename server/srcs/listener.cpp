#include "listener.h"
#include <iostream>


listener::listener(net::io_context& ioc, tcp::endpoint endpoint) : _ioc(ioc), _acceptor(ioc) {
    beast::error_code ec;

    _acceptor.open(endpoint.protocol(), ec);
    if (ec) {
        fail(ec, "open");
        return;
    }

    _acceptor.set_option(net::socket_base::reuse_address(true), ec);
    if (ec) {
        fail(ec, "set_option");
        return;
    }

    _acceptor.bind(endpoint, ec);
    if (ec) {
        fail(ec, "bind");
        return;;
    }

    _acceptor.listen(net::socket_base::max_listen_connections, ec);
    if (ec) {
        fail(ec, "listen");
        return;
    }
}

// Start accepting incoming connections
void listener::run() { do_accept(); }

// Accepting the connection
void listener::do_accept() {
    _acceptor.async_accept(
        net::make_strand(_ioc), 
        beast::bind_front_handler(&listener::on_accept, shared_from_this())
    );
}

// Make a session for a new connection
void listener::on_accept(beast::error_code ec, tcp::socket socket) {
    if (ec) {
        fail(ec, "accept");
    } else {
        std::shared_ptr<session> sess = std::make_shared<session>(std::move(socket));
        sess->run();
    }
    do_accept();
}