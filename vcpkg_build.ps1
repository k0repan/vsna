if (Test-Path C:\vcpkg) {
    echo "vcpkg is already installed"
}
else {
    echo "installing vcpkg,.."
    git clone https://github.com/microsoft/vcpkg.git С:\
    c:\vcpkg\bootstrap-vcpkg.bat
}

$env:VCPKG_ROOT = "C:\vcpkg"
$env:PATH = "$env:VCPKG_ROOT;$env:PATH"

vcpkg new --application
vcpkg new port boost