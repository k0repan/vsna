#ifndef SERVER_H
#define SERVER_H
#include <thread>
#include <fstream>
#include <boost/beast/core.hpp>
#include <boost/beast/http.hpp>
#include <boost/asio.hpp>
#include <boost/filesystem.hpp>
#include <boost/algorithm/string/predicate.hpp>

// THIS FILE WILL BE REWRITTEN IN THE FUTURE!!

namespace beast = boost::beast;
namespace http = beast::http;
namespace asio = boost::asio;
namespace fs = boost::filesystem;
using tcp = boost::asio::ip::tcp;


class server {
public:
    server(fs::path&, std::uint16_t);
    void run_server();

private:
    fs::path& _root_path;
    std::uint16_t _port;
    
    std::string generate_file_list(const fs::path&);
    void handle_request(const fs::path&, http::request<http::string_body>&, http::response<http::string_body>&, asio::ip::tcp::socket&);
    
};

#endif // SERVER_H