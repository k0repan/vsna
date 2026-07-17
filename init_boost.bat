@echo off
setlocal enabledelayedexpansion

set "ROOT=%~dp0"
set "VCPKG_DIR=%ROOT%vcpkg"
set "VCPKG_REPO=https://github.com/microsoft/vcpkg.git"
set "TRIPLET=x64-windows"

if not exist "%VCPKG_DIR%" (
    git clone %VCPKG_REPO% "%VCPKG_DIR%"
    if errorlevel 1 (
        echo Failed to clone vcpkg
        exit /b 1
    )
)

pushd "%VCPKG_DIR%"

call bootstrap-vcpkg.bat
if errorlevel 1 (
    echo Failed to bootstrap vcpkg
    popd
    exit /b 1
)

vcpkg install boost-filesystem boost-system boost-asio boost-beast --triplet %TRIPLET%
if errorlevel 1 (
    echo Failed to install boost
    popd
    exit /b 1
)

vcpkg integrate install

popd

echo -- Done.
endlocal