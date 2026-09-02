#include "pch.h"
#include <queue>

class ClientSession : public std::enable_shared_from_this<ClientSession> {
  public:
	explicit ClientSession(asio::io_context& ioc)
	    : _resolver(asio::make_strand(ioc)), _ws(asio::make_strand(ioc))
	{}
	void run(char const *host, char const *port);
	void on_resolve(beast::error_code ec, tcp::resolver::results_type results);
	void on_connect(beast::error_code ec, tcp::resolver::results_type::endpoint_type ep);
	void on_handshake(beast::error_code ec);
	void do_read();
	void post_write(std::string msg);
	void enqueue_write(std::string msg);
	void do_write();
	void on_write(beast::error_code ec, std::size_t bytes_transferred);
	void on_read(beast::error_code ec, std::size_t bytes_transferred);
	void on_close(beast::error_code ec);

  private:
	tcp::resolver _resolver;
	websocket::stream<beast::tcp_stream> _ws;
	beast::flat_buffer _buffer;
	std::string _host;
	std::string _port;

	std::queue<std::string> _write_queue;
	std::string _current_msg; // Keep message alive during async write
	bool _is_writing = false;
};
