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
    echo "  build  Configure and build"
    echo "  clean  Remove build directory"
    exit 1
}

cmd_build() {
    if [ ! -d "$VCPKG_DIR" ]; then
        echo "-- Error: vcpkg not found. Run ./init_boost.sh"
        exit 1
    fi

    mkdir -p ${BUILD_DIR}
    # ( cd ${BUILD_DIR}; cmake -S .. -B $BUILD_DIR; cmake --build .; )
    ( cd ${BUILD_DIR}; cmake -S .. -B $BUILD_DIR -DCMAKE_TOOLCHAIN_FILE=$VCPKG_DIR/scripts/buildsystems/vcpkg.cmake -DVCPKG_TARGET_TRIPLET=$TRIPLET; cmake --build .; )
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
    --build) cmd_build ;;
    --clean) cmd_clean ;;
    -h|--help) usage ;;
    *)
        echo "Unkown command: $1"
        usage
        ;;
esac
