# VSNA
**Virtual Storage and Network Access** is open-source CLI project, implemented on C++ to exchange data between devices on a _VLAN_.

# Dependencies
- `boost` - asio + beast (_websocket_);
- `CLI11` - command line interface parser.
- `nlohmann/json` - JSON parsing library;

# To run
To run project, you should launch:

**Default build**
```bash
cmake -S . -B build
```

```bash
cmake --build build
```

**OR via ninja**

```bash
cmake -S . -B build -G Ninja 
```

```bash
cd build
ninja
```

# Run Examples (client)
1. With CLI flags
```bash
.\build\client.exe -i 0.0.0.0 -p 8080 -d /path/to
```

2. With config
```bash
.\build\client.exe -c .\__config__.example.json
```

# Run Examples (server)
1. With CLI flags
```bash
.\build\server.exe -i 0.0.0.0 -p 8080 -d /path/to
```

2. With config
```bash
.\build\server.exe -c .\__config__.example.json
```

# Instructions
1. **Server**
   - just running and accept all connections from _VLAN_ (logging parallel)

2. **Client**
   - connect to the server using _WebSocket_ protocol
   - send and receive data (with path tree visualization)

3. **CLI (common)**

|Short, Long name|Description|Default value|
|---|---|---|
| `-h`, `--help` | show help message |-|
| `-p`, `--port <port>` | set port | 8080 |
| `-i`, `--ip <ip>` | set server address | 0.0.0.0 |
| `-d`, `--dir <path>` | set client/server path | <current directory> |
| `-c`, `--config <path>` | set config file path | none |


# Commands
- `exit` - exit the program;
- `help` - show help message;
- `print` - print client config;