# VSNA
**Virtual Storage and Network Access** is open-source TUI project, implemented on C++, to exchange data between devices on a _VLAN_.

# Dependencies
- `boost` - asio + beast (_websocket_);
- `nlohmann/json` - JSON parsing library;
- `cpptui` - text based user interface library.

# Build
First you need to initialize boost via `vcpkg`.

If you don't have it, you can install it by running:

```bash
.\init_modules.bat --init # See more flags with --help
```

**Default build**

Then you can build the project (for client or server):

```bash
.\build.bat
```

Otherwise, if you do not specify the flag, both configurations will be built.

If you use _Unix_ system, you can do the same actions via Shell scripts:

```bash
./init_modules.sh
./build.sh #--no-vcpkg if boost is installed
```

# To run
```bash
.\out\Debug\vsna.exe -c .\config\config.example.json
```

**Project Tree**
```
vsna/
├── .clang-format              # правила форматирования кода
├── Makefile                   # хелпер форматирования кода
├── .gitignore
├── init_modules.bat / .sh     # инициализация зависимостей (vcpkg)
├── build.bat / build.sh       # сборка: cmake configure + build (--server/--client/--clean)
├── CMakeLists.txt             # корневой сценарий сборки (цели: vsna exe + utils/client/server libs)
├── README.md
│
├── config/                    # конфиги приложения
│   ├── config.example.json    # шаблон для новых развёртываний
│   └── config.json
│
├── libs/                      # header-only сторонние библиотеки
│   ├── cpptui.hpp             # TUI-фреймворк
│   └── json.hpp               # парсинг config.json
│
└── src/                       # весь исходный код
    ├── main.cpp               # точка входа; BUILD_SERVER/BUILD_CLIENT выбирают роль
    │
    ├── client/                # КЛИЕНТСКАЯ ЧАСТЬ
    │   ├── client.{h,cpp}     # Client: io_context, connect/sendFiles/download (stub'ы)
    │   ├── cli/               # CLI11-парсинг клиента
    │   │   └── client_cli.{h,cpp}
    │   ├── commands/          # система команд
    │   │   ├── command/       # базовый класс Command
    │   │   │   └── command.{h,cpp}
    │   │   └── manager/       # CommandManager: реестр и вызов команд
    │   │       └── com_manager.{h,cpp}
    │   ├── receiver/          # приём файлов
    │   │   ├── receiver.{h,cpp}
    │   │   └── session/       # ServerSession-сторона на клиенте
    │   │       └── receiver_session.{h,cpp}
    │   └── session/           # исходящий WebSocket-сеанс (ClientSession, пока one-shot)
    │       └── client_session.{h,cpp}
    │
    ├── common/types/          # общие типы
    │   ├── types.h            # STRING_ARG, ARG_VECTOR и др. алиасы
    │   └── pch.h              # precompiled header: boost/beast алиасы, fail()
    │
    ├── tui/                   # TUI-фреймворк (в сборку не входит, ждёт адаптации)
    │   └── tui.{h,cpp}
    │
    └── utils/                 # утилиты общего назначения (цель utils.lib)
        ├── cli.h              # CLI-утилиты
        ├── helper.h           # inline-утилиты: trim, splitArgs, isValidIPv4
        ├── logger.h           # Logger: файловый лог с уровнями
        ├── addr/              # Addr: ip:port, валидация, toString
        │   └── addr.{h,cpp}
        ├── config/            # Config: загрузка из json, getAddr/getPath
        │   └── config.{h,cpp}
        └── output/            # TUI-вывод
            └── tui_output.{h,cpp}
```
