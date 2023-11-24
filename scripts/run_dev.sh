#!/bin/bash

# Function to start the Rust backend and capture its PID
start_rust_backend() {
    echo "Starting Rust backend..."
    (cd cnc_api && cargo run) &
    echo $! > rust_pid.txt
}

# Function to start the Remix frontend and capture its PID
start_remix_frontend() {
    echo "Starting Remix frontend..."
    (cd cnc_frontend && pnpm run dev) &
    echo $! > remix_pid.txt
}

# Start both servers in the background
start_rust_backend
start_remix_frontend

# Wait for any process to exit
wait -n

# Read PIDs from files
RUST_PID=$(cat rust_pid.txt)
REMIX_PID=$(cat remix_pid.txt)

# Clean up PID files
rm rust_pid.txt remix_pid.txt

# Kill both processes if they are still running
if kill -0 $RUST_PID 2>/dev/null; then
    kill $RUST_PID
fi

if kill -0 $REMIX_PID 2>/dev/null; then
    kill $REMIX_PID
fi
