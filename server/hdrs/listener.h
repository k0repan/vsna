#ifndef LISTENER_H
#define LISTENER_H
#include "session.h"
#include <boost/beast/core.hpp>
#include <boost/beast/websocket.hpp>
#include <boost/asio.hpp>


namespace net = boost::asio;
namespace beast = boost::beast;
using namespace beast;
using namespace beast::websocket;
using tcp = net::ip::tcp;


class listener : public std::enable_shared_from_this<listener> {
public:
    listener(net::io_context&, tcp::endpoint);
    void run();

private:
    net::io_context& _ioc;
    tcp::acceptor _acceptor;

    void do_accept();
    void on_accept(beast::error_code, tcp::socket);
};


#endif // LISTENER_H