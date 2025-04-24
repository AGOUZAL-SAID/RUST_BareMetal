#![feature(type_alias_impl_trait)]
#![no_std]
#![no_main]

// Use RTT for defmt logging
use defmt_rtt as _;

use embassy_stm32::interrupt;
use embassy_stm32::interrupt::InterruptExt;
// Link embassy-stm32 for vector table and runtime
use embassy_stm32 as _;
use embassy_stm32::Config;
use embassy_stm32::rcc::*;
use panic_probe as _;
use tp_led_matrix::DISPLAY_EXECUTOR;
use tp_led_matrix::tasks::screensaver;
// Matrix driver and related types
use tp_led_matrix::NEXT_IMAGE;
use tp_led_matrix::image::RED;
use tp_led_matrix::matrix::Matrix;
use tp_led_matrix::tasks::serial_receiver;
use tp_led_matrix::tasks::{blinker, display};
use tp_led_matrix::{Image, POOL};

// Heapless pool for boxed images
use heapless::pool::boxed::BoxBlock;
// Entry point for Embassy executor
#[embassy_executor::main]
async fn main(s: embassy_executor::Spawner) {
    // Confirm defmt is up and running
    defmt::info!("defmt correctly initialized");

    // Configure system clock to 80 MHz using HSI ×10 ÷2
    let mut config = Config::default();
    config.rcc.hsi = true;
    config.rcc.pll = Some(Pll {
        source: PllSource::HSI,
        prediv: PllPreDiv::DIV1,
        mul: PllMul::MUL10,
        divp: None,
        divq: None,
        divr: Some(PllRDiv::DIV2), // 16 MHz × 10 ÷ 2 = 80 MHz
    });
    config.rcc.sys = Sysclk::PLL1_R;
    // Initialize peripherals with this clock config
    let p = embassy_stm32::init(config);

    // UART configuration for serial receiver task
    let mut config_uart = embassy_stm32::usart::Config::default();
    config_uart.baudrate = 38_400;

    // Initialize the LED matrix (pins for rows/cols)
    let my_matrix = Matrix::new(
        p.PA2, p.PA3, p.PA4, p.PA5, p.PA6, p.PA7, p.PA15, p.PB0, p.PB1, p.PB2, p.PC3, p.PC4, p.PC5,
    )
    .await;

    // Set up a heapless pool of Image buffers (3 blocks)
    unsafe {
        #[allow(clippy::declare_interior_mutable_const)]
        const BLOCK: BoxBlock<Image> = BoxBlock::new();
        static mut MEMORY: [BoxBlock<Image>; 3] = [BLOCK; 3];
        // Allow static mutable references for pool management
        #[allow(static_mut_refs)]
        for block in &mut MEMORY {
            POOL.manage(block);
        }
    }

    // Spawn asynchronous tasks: serial receiver, blinker, and display
    s.spawn(serial_receiver(
        p.USART1,
        p.PB7,
        p.PB6,
        p.DMA1_CH4,
        p.DMA1_CH5,
        config_uart,
    ))
    .unwrap();
    s.spawn(blinker(p.PB14)).unwrap();
    s.spawn(screensaver()).unwrap();
    embassy_stm32::interrupt::Interrupt::UART4.set_priority(embassy_stm32::interrupt::Priority::P6);
    let spawner = DISPLAY_EXECUTOR.start(embassy_stm32::interrupt::Interrupt::UART4);
    spawner.spawn(display(my_matrix)).unwrap();

    // Allocate initial image (gradient) and signal the display task
    if let Ok(pool) = POOL.alloc(Image::gradient(RED.gamma_correct())) {
        NEXT_IMAGE.signal(pool);
    }
}

#[interrupt]
unsafe fn UART4() {
    unsafe {
        DISPLAY_EXECUTOR.on_interrupt();
    }
}
