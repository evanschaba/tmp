# [tmp]

Welcome to the `tmp` repository! This project is a collection of 42 school projects re-implemented in Rust instead of C. 

## Overview

This repository includes a series of small projects showcasing Rust implementations of traditional C exercises. The setup scripts automate the development workflow, including linting, formatting, testing, and more.

## Usage

### Environment

**macOS** is recommended for running the scripts, but the setup should be adaptable to other Unix-like environments with minimal modifications.

### Installation and Setup

1. **Clone the Repository**

    ```bash
    git clone https://github.com/evanschaba/tmp.git
    ```

2. **Navigate to the Repository Directory**

    ```bash
    cd tmp
    ```

3. **Make Scripts Executable**

    ```bash
    chmod u+x ./watch.sh ./run.sh
    ```

4. **Run the Watch Script**

    ```bash
    ./watch.sh
    ```

### What `watch.sh` Does

The `watch.sh` script automates the development workflow by performing the following tasks:

- **File Watching**: Monitors changes in the source files.
- **Linting**: Runs Clippy to lint and auto-fix your code according to its suggestions.
- **Formatting**: Automatically formats the code using `rustfmt`.
- **Error Checking**: Checks for compilation errors.
- **Running**: Executes the compiled code.
- **Testing**: Runs all the tests.
- **Clipboard Copying**: Copies all `src/*` files and stdout logs to the clipboard after each file save.

### Additional Information

- **Dependencies**: Ensure you have Rust and Cargo installed. You can install Rust using [rustup](https://rustup.rs/).
- **Clippy and Rustfmt**: Clippy and `rustfmt` are Rust tools for linting and formatting. They are included with the Rust toolchain but can be installed separately if needed.

## License

This project is licensed under the Unlicense - see the [UNLICENSE](UNLICENSE) file for details.
