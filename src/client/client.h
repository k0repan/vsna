#pragma once
#include <boost/asio.hpp>
#include <string>
#include <vector>
#include <thread>
#include <memory>

#include "config.h"
#include "client_session.h"
#include "helper.h"
#include "receiver.h"
#include "tui_output.h"

class Client {
  public:
	Client() : _config(), _io_context()
	{
		_receiver = std::make_shared<Receiver>();
		_receiver_thread = std::thread([r = _receiver] { r->run(); });
	}

	void start_receiver()
	{
        if (_receiver || _receiver_thread.joinable()) return;
        _receiver = std::make_shared<Receiver>();
        _receiver_thread = std::thread([r = _receiver] { r->run(); });
	}
	
	~Client()
	{
        if (_receiver) _receiver->stop();
        if (_receiver_thread.joinable()) _receiver_thread.join();
	}

	void setConfig(const Config& config)
	{
		_config = config;
	}
	Config getConfig() const
	{
		return _config;
	}

	bool isConnected() const
	{
		return _is_connected;
	}	

	void connect(ARG_VECTOR);
	void print() const;
	void showPath(ARG_VECTOR) const;
	void myPath(ARG_VECTOR) const;
	void sendFiles(ARG_VECTOR);
	void download(ARG_VECTOR);
	void sendMsg(ARG_VECTOR);
	void disconnect();

  private:
	Config _config;
	bool _is_connected{ false };
	std::shared_ptr<Receiver> _receiver;
	std::thread _receiver_thread;
	boost::asio::io_context _io_context;
	std::shared_ptr<ClientSession> _session;
	std::thread _io_thread;
};
