#!/bin/zsh

# Define a function to run custom logic with cargo-watch
run_cargo_watch() {
    echo "Starting cargo-watch..."
    cargo watch -w src/ -s './run.sh'
}

# Check if cargo-watch is installed
if command -v cargo-watch >/dev/null 2>&1; then
    echo "cargo-watch is already installed."
    run_cargo_watch
else
    echo "cargo-watch is not installed. Installing..."
    cargo install cargo-watch
    if [ $? -eq 0 ]; then
        echo "cargo-watch has been installed successfully."
        run_cargo_watch
    else
        echo "Failed to install cargo-watch. Please install it manually and try again."
        exit 1
    fi
fi
