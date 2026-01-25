#ifndef SESSION_H
#define SESSION_H
#include <boost/beast/core.hpp>
#include <boost/beast/websocket.hpp>
#include <boost/asio.hpp>



namespace net = boost::asio;
namespace beast = boost::beast;
using namespace beast;
using namespace beast::websocket;
using tcp = net::ip::tcp;


void fail(beast::error_code, char const*);


class session : public std::enable_shared_from_this<session> {
public:
    explicit session(tcp::socket&&);
    void run();
    void on_run();
    void on_accept(beast::error_code);
    void do_read();
    void on_read(beast::error_code, size_t);
    void on_write(beast::error_code, size_t);    

private:
    websocket::stream<beast::tcp_stream> _ws;
    beast::flat_buffer _buf;
};


#endif // SESSION_H