#!/bin/zsh



# -x will use 'cargo run'
# -q will suppress cargo output
# -s will use a shell cmd
cargo watch -s './run.sh'
