@echo off
setlocal

cd /d "%~dp0"

call :run
exit /b 0

:run
..\out\Debug\vsna.exe
exit /b %errorlevel%