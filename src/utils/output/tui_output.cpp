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
	_queue.push({type, formatted});
}

std::vector<OutputMsg> TUIOutputStream::drain()
{
	std::vector<OutputMsg> result;
	while (!_queue.empty())
	{
		result.push_back(std::move(_queue.front()));
		_queue.pop();
	}
	return result;
}