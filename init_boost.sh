#!/usr/bin/env bash
set -e

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
VCPKG_DIR="$ROOT/vcpkg"
VCPKG_REPO="https://github.com/microsoft/vcpkg.git"
TRIPLET="x64-linux"

if [ ! -d "$VCPKG_DIR" ]; then
    git clone "$VCPKG_REPO" "$VCPKG_DIR"
fi

cd "$VCPKG_DIR"
./bootstrap-vcpkg.sh

./vcpkg install boost-filesystem boost-system boost-asio boost-beast --triplet "$TRIPLET"
./vcpkg integrate install

echo "-- Done!"