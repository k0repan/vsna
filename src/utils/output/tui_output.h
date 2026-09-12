#pragma once
#include <string>
#include <source_location>
#include <format>
#include <queue>

namespace TUI
{
    enum class OutputType {
		Info,
		Error,
	};
}

struct OutputMsg {
    TUI::OutputType type;
    std::string text;
};

class TUIOutputStream
{
public:
    static TUIOutputStream& instance(){
        static TUIOutputStream instance;
        return instance;
    }
	void push_msg(TUI::OutputType type, const std::string& str,
		std::source_location loc = std::source_location::current());
	std::vector<OutputMsg> drain();

private:
    TUIOutputStream() = default;
	~TUIOutputStream() = default;

	std::queue<OutputMsg> _queue;
};

namespace TUI
{
    inline void print(const std::string& str,
		std::source_location loc = std::source_location::current())
    {
        TUIOutputStream::instance().push_msg(OutputType::Info, str, loc);
    }

    inline void print_err(const std::string& str,
		std::source_location loc = std::source_location::current())
    {
        TUIOutputStream::instance().push_msg(OutputType::Error, str, loc);
    }
}
