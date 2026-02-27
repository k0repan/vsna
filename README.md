# VSNA
**Virtualized Storage and Network Access** is education open source project, realized on Rust{tokio, tungstenite} and CPP{boost} to exchange data between devices on a VLAN.

At the moment, the Rust implementation is ready for sending files between *client* and *server*.

# Run Rust (MVP)
To run one of Rust realization (*server*|*client*) you need to push `cargo run` cmd in one of the (*server*|*client*) dir.

# Run CPP (in DEV)
To run project from scratch, you should goto `cpp_server/` and push `cmake -G Ninja -DCMAKE_BUILD_TYPE=Debug -DCMAKE_C_COMPILER="cl.exe" -DCMAKE_CXX_COMPILER="cl.exe" -DMSVC_TOOLSET_VERSION=143 -B build -DCMAKE_EXPORT_COMPILE_COMMANDS=On`. After that, move to `build/` and run `ninja`.