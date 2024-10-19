#!/bin/zsh

head -n 5000 run.sh | pbcopy

# Remove the previous tmp file if it exists
rm -rf tmp ./target/logs/

# Create log directory if it doesn't exist
mkdir -p target/logs

# Run cargo commands and capture output
{
    # Display project structure without the target directory
    tree -I target

    # Output the first 1000 lines of the trace log and log4rs configuration
    head -n 1000 log4rs.yaml Cargo.toml lib/tracer/Cargo.toml

    # Check the project for any errors
    cargo check --all

    # Run Clippy for linting and automatic fixes
    cargo clippy --all --fix --allow-dirty --allow-staged

    # Format the code according to Rust's style guidelines
    cargo fmt

    # Optional: Uncomment the next line to clean the build
    # cargo clean

    # Output the first 1000 lines of the relevant source files and Cargo.toml
    head -n 1000 lib/tracer/src/lib.rs src/main.rs

    echo "\n---- cargo run output: ----- \n"

    # Run the application with backtrace enabled
    RUST_BACKTRACE=1 cargo run

    echo "\n---- target/logs/traces.logs: ----- \n"
    head -n 5000 target/logs/traces.log
} &>tmp

head -n 5000 tmp | pbcopy
