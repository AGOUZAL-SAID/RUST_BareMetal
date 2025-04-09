#![no_std]
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use image::BLACK;
pub mod gamma;
pub mod image;
pub mod matrix;
pub mod tasks;
pub use image::{Color, Image};
use heapless::pool::boxed::Box;
use embassy_sync::signal::Signal;

pub static IMAGE: Mutex<ThreadModeRawMutex, Image> = Mutex::new(Image::new_solid(BLACK));
heapless::box_pool!(POOL: Image);
pub static NEXT_IMAGE: Signal<ThreadModeRawMutex, Box<POOL>> = Signal::new();    