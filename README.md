# VSNA
**Virtual Storage and Network Access** is open-source CLI project, implemented on C++ to exchange data between devices on a _VLAN_.

# Dependencies
- `boost` - asio + beast (_websocket_);
<!-- - `CLI11` - command line interface parser.
- `nlohmann/json` - JSON parsing library; -->

# To run
First you need to initialize boost via vcpkg, if you don't have it;. This is done once.:

**Boost initialization**
```bash
.\init.boost.bat
```

```shell
./ini
```
You can see more commands by:

```bash
.\init_boost.bat --help
```

**Default build**

Then you can build the project:

```bash
.\build.bat
```

If you use Unix system, you can do the same actions via Shell scripts:

```shell
./init_boost.sh
./build.sh # See more flags with --help
```