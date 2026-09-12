#pragma once
#include <libs/cpptui.hpp>

#include <algorithm>
#include <chrono>
#include <ctime>
#include <filesystem>
#include <string>
#include <vector>
#include <sstream>

#include "client_cli.h"
#include "server_cli.h"
#include "helper.h"
#include "tui_output.h"

namespace fs = std::filesystem;
using namespace cpptui;

std::string timestamp();

enum class LineKind {
	kSystem,
	kUser,
	kResult,
	kError,
};

struct OutputLine
{
	std::string text;
	LineKind kind;
};

std::string human_size(uintmax_t bytes);

class HistoryInput : public Input {
  public:
	std::string take_text();
	void add_history(STRING_ARG entry);
	bool on_event(const Event& event) override;

  private:
	void browse(int dir);

	static constexpr size_t kMaxHistory = 100;
	std::vector<std::string> entries_;
	int index_ = 0;
	std::string draft_;
};

class TuiApp {
  public:
	void run(char **argv);
	TuiApp() : clientCLI_(), serverCLI_() {}

  private:
	void build_ui();
	std::shared_ptr<Vertical> build_output_page();
	std::shared_ptr<Vertical> build_settings_page();
	void apply_theme(int idx);
	void submit();

	bool execute_command(STRING_ARG input);

	static void scan_dir_into(TreeNode& node, const fs::path& dir);
	void open_list_dialog(STRING_ARG path_str);
	void close_list_dialog();

	void append_system(STRING_ARG text);
	void append_result(STRING_ARG text);
	void add_line(LineKind kind, STRING_ARG text);
	void drain_output();
	void refresh_output();

	App app_;
	std::shared_ptr<Vertical> root_;
	std::shared_ptr<Tabs> tabs_;
	std::shared_ptr<ScrollableVertical> output_scroll_;
	std::shared_ptr<Static> output_text_;
	std::shared_ptr<HistoryInput> input_;
	std::shared_ptr<Input> settings_field_;
	std::shared_ptr<Dialog> list_dialog_;
	std::shared_ptr<TreeView> list_tree_;
	std::shared_ptr<RadioSet> theme_list_;
	std::shared_ptr<Checkbox> settings_checkbox_;
	TimerId focus_fix_timer_{ -1 };
	TimerId drain_timer_{ -1 };
	std::vector<OutputLine> lines_;

	ClientCLI clientCLI_;
	ServerCLI serverCLI_;
};
