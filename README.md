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
.\client.exe -p 8080 -d /path/to/client -i 192.168.0.1
```

2. With config
```bash
.\client.exe -c .\__config__.example.json
```

# Run Examples (server)
1. With CLI flags
```bash
.\server.exe -p 8080 -d /path/to/server -i 0.0.0.0
```

2. With config
```bash
.\server.exe -c .\__config__.example.json
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

4. **CLI (client)**

|Short, Long name|Description|Default value|
|---|---|---|
| `-a`, `--auto-sync` | enable auto sync between client and server | false |

5. **CLI (server)**

|Short, Long name|Description|Default value|
|---|---|---|
| `-m`, `--max-clients <count>` | set max clients | 1 |

# Commands
- `exit` - exit the program;
- `help` - show help message;
- `print` - print client config;