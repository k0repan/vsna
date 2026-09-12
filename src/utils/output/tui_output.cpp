#include "tui_output.h"

void TUIOutputStream::push_msg(TUI::OutputType type, const std::string& str,
	std::source_location loc)
{
	const std::string src_msg = std::string(loc.file_name()).find("server") != std::string::npos ? "SERVER" : "CLIENT";
	const std::string type_msg = type == TUI::OutputType::Error ? "ERROR" : "INFO";
	
	const std::string formatted = std::format("[{}][{}] {}\n",
		type_msg,
		src_msg,
		str);
	_queue.push(formatted);
}

queue_str TUIOutputStream::drain()
{
	queue_str drained = this->_queue;
	this->_queue = queue_str();
	return drained;
}