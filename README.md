# RUST_BareMetal

This repository is a collection of Rust projects and exercises, focusing on embedded programming and bare-metal development, as well as programming problems and virtual machine implementations.

## Repository Structure

The repository is organized into several subdirectories, each representing a distinct project or set of problems.

### `fibo`

This subdirectory contains a Rust project for calculating the Fibonacci sequence. It uses the `clap` library for command-line argument parsing, suggesting a console application for computing Fibonacci numbers.

*   **Core Functionality**: Calculates the nth Fibonacci number iteratively with `u32` overflow checking.
*   **Usage**: The program can be executed with arguments to specify the calculation range and verbose mode.
    ```bash
    # Example usage
    cargo run --release -- -v --mini 0 --value 10
    ```
*   **Dependencies**: `clap` (for command-line argument parsing).

### `problems`

This directory is intended to hold solutions to various programming problems. It is a standard Rust project with no specific external dependencies listed in its `Cargo.toml`, indicating it might contain algorithmic or basic data structure problems.

*   **Usage**: Contains implementations of solutions to programming challenges.

### `tp-led-matrix`

This project is a driver for an 8x8 RGB LED matrix, developed in Rust for STM32 microcontrollers using the Embassy framework. It implements advanced functionalities for LED control.

*   **Features**: 
    *   8x8 LED Matrix Control with row multiplexing.
    *   Gamma Correction for color accuracy.
    *   Serial Image Streaming via UART (38400 baud).
    *   Screensaver with scrolling text.
    *   Blinker Task for system monitoring.
    *   Image Pooling for efficient image buffer management.
*   **Dependencies**: `embassy-stm32`, `embedded-graphics`, `defmt`, `heapless`.
*   **Code Structure**: 
    *   `main.rs`: Program's entry point (`no_std`).
    *   `matrix/mod.rs`: Implements shift-register protocols and GPIO management.
    *   `image/mod.rs`: Defines `Color` operations and `Image` indexing/gradient logic.
    *   `tasks/mod.rs`: Contains asynchronous tasks for display, screensaver, serial reception, and blinking.

### `vm`

This subdirectory contains a virtual machine implementation in Rust. It includes examples of binary and disassembled programs for testing the VM, such as factorial calculation, Fibonacci sequence, and a simple "hello world" program.

*   **Functionality**: A custom virtual machine capable of executing simple programs.
*   **Examples**: `99bottles.bin`, `count.bin`, `factorial.bin`, `fibonacci.bin`, `hello_world.bin` (and their `.dis` disassembled counterparts).
*   **Tests**: Includes various tests for basic operations, assignments, functions, and recursive functions.

## Contribution

Contributions are welcome. Please follow established coding practices and submit pull requests for any improvements or bug fixes.

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details (if present).

