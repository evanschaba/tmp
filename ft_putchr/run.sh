#!/bin/zsh

# Configuration
dir="target/"
name="tmp.out"
output="$dir$name"

# Remove and recreate the output file
rm -f $output
mkdir -p $dir
touch $output

# Function to log and execute commands
execute_and_log() {
    local command="$1"
    local description="$2"
    
    echo "--------------------------------------------------------------------- $description ---------------------------------------------------------------------" >> $output
    echo "Executing: $command"
    eval "$command" &>> $output
}

# Lint the files
execute_and_log "cargo clippy" "cargo clippy"
execute_and_log "cargo clippy --fix --lib -p libft" "cargo clippy --fix --lib -p libft"
execute_and_log "cargo fix --allow-dirty --allow-staged" "cargo fix --allow-dirty --allow-staged"

# Format the files
execute_and_log "cargo fmt" "cargo fmt"

# Check the code
execute_and_log "cargo check" "cargo check"

# Run the code
execute_and_log "cargo run" "cargo run"

# Test the code
execute_and_log "cargo test" "cargo test"

# Copy output to clipboard
head -n 10000 $output src/*.rs | pbcopy

# Print the output file
cat $output # Use 'bat' for better output visualization if installed

