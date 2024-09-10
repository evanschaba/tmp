#!/bin/zsh

# Define a function to run custom logic
cargo_watch_exec() {
    # -x will use 'cargo run'
    # -q will suppress cargo output
    # -s will use a shell cmd
    cargo watch -s './run.sh'
}

# Check if cargo-watch is installed
if cargo watch --version >/dev/null 2>&1; then
    echo "cargo-watch is already installed."
    # Run the command you want if cargo-watch is installed
    cargo_watch_exec
else
    echo "cargo-watch is not installed. Installing..."
    # Install cargo-watch
    cargo install cargo-watch
    echo "cargo-watch has been installed."
    cargo_watch_exec
fi
