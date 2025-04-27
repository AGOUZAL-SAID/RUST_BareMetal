# LED Matrix Display Driver with Embassy on STM32

This Rust project part is for driving an 8×8 RGB LED matrix using the Embassy framework on STM32 microcontrollers. Features include gamma correction, serial image streaming, animations, and a scrolling screensaver.

## Features

* **8×8 LED Matrix Control**: Drive RGB LEDs with precise timing and row multiplexing.
* **Gamma Correction**: Precomputed gamma table for color accuracy.
* **Serial Image Streaming**: Receive images over UART (38_400 baudrate) and display them instantly.
* **Screensaver**: Scrolls "♥ FREE PALESTINE ♥" after 5 seconds of inactivity.
* **Blinker Task**: Visual heartbeat LED for system monitoring.
* **Image Pooling**: Heapless memory pool for efficient image buffer management.

## Dependencies

* `embassy-stm32`: Async HAL for STM32.
* `embedded-graphics`: Drawing primitives.
* `defmt`: Logging framework.
* `heapless`: Static memory pools.

## Core Components

* **`Color` and `Image` Types**:
  * `Color`: RGB pixel with gamma correction.
  * `Image`: 8×8 pixel buffer with row/column indexing and gradient generation.

* **Matrix Driver**: Handles  row activation.

* **Async Tasks**:
  * `display`: Updates the matrix at 80 Hz.
  * `serial_receiver`: Parses UART data into images.
  * `screensaver`: Manages inactivity-triggered animations.

## Code Structure

* `main.rs`: Contains the main entry point of the program (`no_std`).
* `matrix/mod.rs`: Implements shift-register protocols (`send_row`, `pulse_sck`) and GPIO management.
* `image/mod.rs`: Defines `Color` operations (gamma, scaling) and `Image` indexing/gradient logic.
* `tasks/mod.rs`:
  * `display`: Buffers images and drives the matrix.
  * `screensaver`: Handles text rendering and scrolling.
  * `serial_receiver`: Parses UART byte streams into valid `Image` buffers.
  * `blinker`: blinks three times per second.