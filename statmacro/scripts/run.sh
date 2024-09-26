#!/bin/zsh

# Create a temporary file to store output
tmp_file="tmp"

# Clear the tmp file if it exists
: > "$tmp_file"

# Run cargo commands to format and lint the code first
{
    cargo fmt
    cargo clippy --fix --all-targets --all-features --allow-dirty --allow-staged
} >> "$tmp_file" 2>&1

# Collect the first 1000 lines of all .rs files and append to tmp_file
head -n 1000 src/*.rs tests/*.rs lib/**/src/*.rs >> "$tmp_file"

# Get the line count of the .rs files collected
loc=$(wc -l < "$tmp_file")

# Collect the first 1000 lines of all .toml files and append to tmp_file
head -n 1000 Cargo.toml lib/**/*.toml >> "$tmp_file"

# Run cargo commands and append output to tmp_file
{
    cargo check
    cargo run
    cargo test
} >> "$tmp_file" 2>&1

# Copy the content of tmp_file to clipboard
cat "$tmp_file" | pbcopy

# Output the number of lines copied
echo "$loc"

# Clean up the temporary file
rm "$tmp_file"
