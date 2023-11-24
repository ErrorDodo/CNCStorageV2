@echo off
SETLOCAL EnableDelayedExpansion

:: Function to start the Rust backend and capture its PID
:start_rust_backend
echo Starting Rust backend...
start /b cmd /c "cd cnc_api && cargo run"
for /f "tokens=2" %%a in ('tasklist /nh /fi "imagename eq cmd.exe"') do (
    set RUST_PID=%%a
    goto :end_rust_backend
)
:end_rust_backend

:: Function to start the Remix frontend and capture its PID
:start_remix_frontend
echo Starting Remix frontend...
start /b cmd /c "cd cnc_frontend && pnpm run dev"
for /f "tokens=2" %%a in ('tasklist /nh /fi "imagename eq cmd.exe"') do (
    set REMIX_PID=%%a
    goto :end_remix_frontend
)
:end_remix_frontend

:: Wait for user input to terminate
echo.
echo Press any key to stop servers...
pause > nul

:: Kill the processes if they are still running
taskkill /PID !RUST_PID! /F 2>nul
taskkill /PID !REMIX_PID! /F 2>nul

echo Servers stopped. Exiting...

:end
