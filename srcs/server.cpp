#include "server.h"
#include <iostream>


server::server(fs::path& root_path, std::uint16_t port) : _root_path(root_path), _port(port) {}

std::string server::generate_file_list(const fs::path& current_path) {
    std::string html = "<html><body><h1>Files:</h1><ol>";

    if (current_path != _root_path) {
        fs::path parent_path = current_path.parent_path();
        std::string parent_link = fs::relative(parent_path, _root_path).string();
        html += "<li><a href=\"" + parent_link + "\">..</a></li>";
    }

    for (const auto& entry : fs::directory_iterator(current_path)) {
        std::string name = entry.path().filename().string();
        std::string link = fs::relative(entry.path(), _root_path).string();

        if (fs::is_directory(entry)) {
            html += "<li><a href=\"" + link + "\">" + name + "/</a></li>";
        } else if (fs::is_regular_file(entry)) {
            html += "<li><a href=\"" + link + "\">" + name + "</a></li>";
        }
    }

        html += "</ol></body></html>";

    return html;
}

void server::handle_request(
    const fs::path& root_path, 
    http::request<http::string_body>& req, 
    http::response<http::string_body>& res, 
    asio::ip::tcp::socket& socket
){
    std::string target = std::string(req.target());
    
    if (target.empty() || target == "/") {
        res.result(http::status::ok);
        res.body() = generate_file_list(root_path);
        res.set(http::field::content_type, "text/html");
        return;
    }

    target.erase(0, 1);
    fs::path file_path = root_path / target;

    if (fs::is_directory(file_path)) {
        res.result(http::status::ok);
        res.body() = generate_file_list(file_path);
        res.set(http::field::content_type, "text/html");
        return;
    }

    if (!fs::exists(file_path) || !fs::is_regular_file(file_path)) {
        res.result(http::status::not_found);
        res.body() = "File not found";
        return;
    }

    std::ifstream file(file_path.string(), std::ios::binary);
    if (!file) {
        res.result(http::status::internal_server_error);
        res.body() = "Cannot open file";
        return;
    }

    res.result(http::status::ok);
    res.set(http::field::content_type, "application/octet-stream");
    res.set(http::field::content_disposition, "attachment; filename=\"" + file_path.filename().string() + "\"");

    constexpr size_t buffer_size = 8192;
    char buffer[buffer_size];

    try {
        while (file) {
            file.read(buffer, buffer_size);
            std::streamsize bytes_read = file.gcount();
            if (bytes_read > 0) {
                res.body() = std::string(buffer, buffer + bytes_read);
                http::write(socket, res);
            }
        }
    } catch (std::exception& e) {
        res.result(http::status::internal_server_error);
        res.body() = "Error reading or sending file: " + std::string(e.what());
        return;
    }
}

void server::run_server() {
    try {
        asio::io_context ioc;
        tcp::acceptor acceptor(ioc, {tcp::v4(), _port});
        std::cout << "Server started at port " << _port << std::endl;

        while (true) {
            tcp::socket socket(ioc);
            acceptor.accept(socket);

            beast::flat_buffer buffer;
            http::request<http::string_body> req;
            http::response<http::string_body> res;

            try {
                http::read(socket, buffer, req);
            } catch (const boost::system::system_error& e) {
                if (e.code() == http::error::end_of_stream) {
                    std::cerr << "Client disconnected: " << e.what() << std::endl;
                    continue;
                } else{
                    std::cerr << "Error: " << e.what() << std::endl;
                    continue;
                }
            }

            handle_request(_root_path, req, res, socket);

            try {
                http::write(socket, res);

            } catch (const boost::system::system_error& e) {
                if (e.code() == boost::asio::error::broken_pipe) {
                    std::cerr << "Client disconnected: " << e.what() << std::endl;
                } else {
                    std::cerr << "Error: " << e.what() << std::endl;
                }
            }
        }
    } catch (std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
    }    
}