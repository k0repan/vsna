@echo off
set "TRIPLET=x64-windows"
set "BUILD_TYPE=Debug"

if not exist "vcpkg" (
    echo "-- Error: vcpkg not found. Run .\init_boost.bat"
    exit /b 1
)

if not exist "out" (
    md out
)

cmake -B out -S . ^
    -DCMAKE_TOOLCHAIN_FILE=vcpkg\scripts\buildsystems\vcpkg.cmake ^
    -DVCPKG_TARGET_TRIPLET=%TRIPLET% ^
    -DCMAKE_BUILD_TYPE=%BUILD_TYPE%

cmake --build out