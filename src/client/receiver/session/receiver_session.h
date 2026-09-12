#pragma once
#include "pch.h"
#include "tui_output.h"

class ReceiverSession : public std::enable_shared_from_this<ReceiverSession> {
  public:
	explicit ReceiverSession(tcp::socket socket) : _ws(std::move(socket))
	{}
	void run();

  private:
	websocket::stream<tcp::socket> _ws;
	beast::flat_buffer _buffer;

	void on_accept(beast::error_code ec);
	void do_read();
	void on_read(beast::error_code ec, std::size_t bytes_transferred);
	void on_write(beast::error_code ec, std::size_t bytes_transferred);
};
