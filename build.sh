#!/usr/bin/env bash

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

BUILD_DIR=${BUILD_DIR:-out}
VCPKG_DIR="$SCRIPT_DIR/vcpkg"
TRIPLET="x64-linux"

usage() {
    echo "Usage: $0 <command>"
    echo "Commands:"
    echo "  --build     Configure and build via vcpkg"
    echp "  --no-vcpkg  Configure and build without vcpkg (via lib-devel)"
    echo "  --clean     Remove build directory"
    exit 1
}

cmd_build_no_vcpkg() {
    mkdir -p ${BUILD_DIR}
    echo "-- Looking in local packages"
    ( cmake -S . -B $BUILD_DIR; cmake --build $BUILD_DIR; )
}

cmd_build_via_vcpkg() {
    mkdir -p ${BUILD_DIR}
    if [ ! -d "$VCPKG_DIR" ]; then
        echo "-- Error: vcpkg not found. Run ./init_boost.sh"
        exit 1
    fi
    ( cmake -S . -B $BUILD_DIR -DCMAKE_TOOLCHAIN_FILE=$VCPKG_DIR/scripts/buildsystems/vcpkg.cmake -DVCPKG_TARGET_TRIPLET=$TRIPLET; cmake --build $BUILD_DIR; )

    echo "-- Build done!"
}

cmd_clean() {
    if [ -d "$BUILD_DIR" ]; then
    rm -rf "$BUILD_DIR"
    echo "-- Removed $BUILD_DIR"
  fi
  echo "-- Clean done!"
}

case "${1:---build}" in
    --build) cmd_build_via_vcpkg ;;
    --no-vcpkg) cmd_build_no_vcpkg ;;
    --clean) cmd_clean ;;
    -h|--help) usage ;;
    *)
        echo "Unkown command: $1"
        usage
        ;;
esac
