#![no_std] // Use without the standard library

use embassy_executor::InterruptExecutor;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::Instant;
use heapless::pool::boxed::Box;

// Re-exported modules for gamma correction, image handling, matrix driver, and async tasks
pub mod gamma;
pub mod image;
pub mod matrix;
pub mod tasks;

// Make Color and Image types available at the crate root
pub use image::{Color, Image};

// Create a heapless pool of boxed Image buffers
heapless::box_pool!(POOL: Image);

// Signal used to pass the next image buffer between tasks (thread‑safe in main context)
pub static NEXT_IMAGE: Signal<CriticalSectionRawMutex, Box<POOL>> = Signal::new();
pub static NEW_IMAGE_RECEIVED: Signal<ThreadModeRawMutex, Instant> = Signal::new();

// Executor for interrupt‑driven tasks (e.g., display refresh)
pub static DISPLAY_EXECUTOR: InterruptExecutor = InterruptExecutor::new();
