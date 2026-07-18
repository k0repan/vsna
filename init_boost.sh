#!/usr/bin/env bash
set -e

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
VCPKG_DIR="$ROOT/vcpkg"
VCPKG_REPO="https://github.com/microsoft/vcpkg.git"
TRIPLET="x64-linux"

usage() {
    echo "Usage: $0 <command>"
    echo "Commands:"
    echo "  --init   Clone vcpkg and init boost"
    echo "  --clean  Remove vcpkg directory"
    exit 1
}

cmd_init() {
    if [ ! -d "$VCPKG_DIR" ]; then
        git clone "$VCPKG_REPO" "$VCPKG_DIR"
    fi

    cd "$VCPKG_DIR"
    ./bootstrap-vcpkg.sh

    ./vcpkg install boost-filesystem boost-system boost-asio boost-beast --triplet "$TRIPLET"
    ./vcpkg integrate install

    echo "-- Done!"
}

cmd_clean() {
    if [ -d "$VCPKG_DIR" ]; then
    rm -rf "$VCPKG_DIR"
    echo "-- Removed $VCPKG_DIR"
  fi
  echo "-- Clean done!"
}

case "${1:---init}" in
    --init) cmd_init ;;
    --clean) cmd_clean ;;
    -h|--help) usage ;;
    *)
        echo "Unkown command: $1"
        usage
        ;;
esac
