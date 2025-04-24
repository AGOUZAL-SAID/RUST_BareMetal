use crate::{Color, Image, POOL};
use crate::{NEW_IMAGE_RECEIVED, NEXT_IMAGE, matrix::Matrix};
use core::panic;
use embassy_stm32::{gpio::*, peripherals::PB14};
use embassy_stm32::{
    peripherals::{DMA1_CH4, DMA1_CH5, PB6, PB7, USART1},
    usart::{Config, Uart},
};
use embassy_time::Instant;
use embassy_time::{Duration, Ticker, Timer};
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::Point;
use embedded_graphics::prelude::{RgbColor, Size};
use embedded_graphics::primitives::Triangle;
use embedded_graphics::primitives::{Circle, Line, PrimitiveStyle, Rectangle, StyledDrawable};
use futures::future::FutureExt;

/// Task that pulls the next image from the pool and sends it to the LED matrix.
#[embassy_executor::task]
pub async fn display(mut matrix: Matrix<'static>) {
    // Tick at a rate matching the refresh (80 Hz × 8 rows)
    let mut ticker = Ticker::every(Duration::from_hz(80 * 8));
    let mut buffer: [Color; 64] = [Color::default(); 64];
    // Initial image to display
    let mut image = NEXT_IMAGE.wait().await;
    loop {
        {
            // If a new image is already available, swap it in immediately
            if let Some(v) = NEXT_IMAGE.wait().now_or_never() {
                image = v;
            }
            // Copy one row at a time into the buffer
            for row in 0..8 {
                buffer[row * 8..(row + 1) * 8].copy_from_slice(image.row(row));
            }
        }
        // Wrap buffer in a fresh Image object for display
        let new_image = Image::new_im(buffer);
        matrix.display_image(&new_image, &mut ticker).await;
    }
}

/// Task that toggles an LED on PB14 in a simple blinking pattern.
#[embassy_executor::task]
pub async fn blinker(pb14: PB14) {
    let mut pin_led = Output::new(pb14, Level::Low, Speed::VeryHigh);
    loop {
        pin_led.set_high();
        Timer::after_millis(100).await;
        pin_led.set_low();
        Timer::after_millis(100).await;
        pin_led.set_high();
        Timer::after_millis(100).await;
        pin_led.set_low();
        Timer::after_millis(100).await;
        pin_led.set_high();
        Timer::after_millis(100).await;
        pin_led.set_low();
        Timer::after_millis(1000).await; // Longer off period
    }
}

/// Task that reads commands and image data over USART1 + DMA, fills an Image from the pool,
/// and signals the display task when a complete image is received.
#[embassy_executor::task]
pub async fn serial_receiver(
    usart: USART1,
    pb7: PB7,
    pb6: PB6,
    dma1_tx: DMA1_CH4,
    dma1_rx: DMA1_CH5,
    config_uart: Config,
) {
    // Bind USART1 interrupt to the embassy handler
    embassy_stm32::bind_interrupts!(struct Irqs {
        USART1 => embassy_stm32::usart::InterruptHandler<embassy_stm32::peripherals::USART1>;
    });

    // Initialize UART in async mode with DMA
    let mut usart1: Uart<'static, embassy_stm32::mode::Async> =
        Uart::new(usart, pb7, pb6, Irqs, dma1_tx, dma1_rx, config_uart).unwrap();

    let mut word: [u8; 1] = [0; 1];
    let mut count = 0;

    loop {
        let mut image;
        // Allocate a new Image buffer from the pool
        if let Ok(pool) = POOL.alloc(Image::default()) {
            image = pool;
        } else {
            panic!("Failed to allocate IMAGE from POOL");
        }

        // Wait for start byte 0xFF
        usart1.read(&mut word).await.unwrap();
        if word[0] == 0xff {
            // Read the image payload into the pooled buffer
            usart1.read(image.as_mut()).await.unwrap();

            // Find end-of-image marker and handle rotation if needed
            loop {
                if image.as_mut()[count] == 0xff {
                    image.as_mut().rotate_left(count + 1); // Align image start
                    usart1
                        .read(&mut image.as_mut()[192 - count - 1..])
                        .await
                        .unwrap();
                    count = 0;
                } else if count == 191 {
                    count = 0;
                    break; // Full image received
                } else {
                    count += 1;
                }
            }
            // Signal the display task with the new image
            NEXT_IMAGE.signal(image);
            NEW_IMAGE_RECEIVED.signal(Instant::now());
        }
    }
}
#[embassy_executor::task]
pub async fn screensaver() {
    // Time of the last received signal
    let mut last_signal = Instant::now();

    loop {
        // Non-blocking check for a new signal
        if let Some(new_signal) = NEW_IMAGE_RECEIVED.wait().now_or_never() {
            last_signal = new_signal;
        }

        let mut now = Instant::now();
        let pause = Timer::after_secs(1);

        // If an image arrived recently, wait and continue
        if now.duration_since(last_signal) < Duration::from_secs(5) {
            pause.await;
        } else {
            // Otherwise, start the screensaver sequence
            defmt::info!("screensaver ON :)");

            loop {
                // RED Triangle frame
                let mut im = Image::default();
                let triangle = Triangle::new(Point::new(0, 0), Point::new(7, 0), Point::new(0, 7));
                let style = PrimitiveStyle::with_stroke(Rgb888::RED, 1);
                triangle.draw_styled(&style, &mut im).unwrap();
                if let Ok(pool) = POOL.alloc(im) {
                    NEXT_IMAGE.signal(pool);
                }
                Timer::after(Duration::from_millis(500)).await;

                if let Some(new_signal) = NEW_IMAGE_RECEIVED.wait().now_or_never() {
                    last_signal = new_signal;
                    now = Instant::now();
                }
                if now.duration_since(last_signal) < Duration::from_secs(5) {
                    break;
                }

                // GREEN Rectangle frame
                let mut im1 = Image::default();
                let rectangle = Rectangle::new(Point::new(0, 0), Size::new(8, 8));
                let style = PrimitiveStyle::with_stroke(Rgb888::GREEN, 1);
                rectangle.draw_styled(&style, &mut im1).unwrap();
                if let Ok(pool) = POOL.alloc(im1) {
                    NEXT_IMAGE.signal(pool);
                }
                Timer::after(Duration::from_millis(500)).await;

                if let Some(new_signal) = NEW_IMAGE_RECEIVED.wait().now_or_never() {
                    last_signal = new_signal;
                    now = Instant::now();
                }
                if now.duration_since(last_signal) < Duration::from_secs(5) {
                    break;
                }

                // BLUE Circle frame
                let mut im2 = Image::default();
                let circle = Circle::new(Point::new(0, 0), 8);
                let style = PrimitiveStyle::with_stroke(Rgb888::BLUE, 1);
                circle.draw_styled(&style, &mut im2).unwrap();
                if let Ok(pool) = POOL.alloc(im2) {
                    NEXT_IMAGE.signal(pool);
                }
                Timer::after(Duration::from_millis(500)).await;

                if let Some(new_signal) = NEW_IMAGE_RECEIVED.wait().now_or_never() {
                    last_signal = new_signal;
                    now = Instant::now();
                }
                if now.duration_since(last_signal) < Duration::from_secs(5) {
                    break;
                }

                // WHITE X frame
                let mut im3 = Image::default();
                let x1 = Line::new(Point::new(0, 0), Point::new(7, 7));
                let x2 = Line::new(Point::new(7, 0), Point::new(0, 7));
                let style = PrimitiveStyle::with_stroke(Rgb888::WHITE, 1);
                x1.draw_styled(&style, &mut im3).unwrap();
                x2.draw_styled(&style, &mut im3).unwrap();
                if let Ok(pool) = POOL.alloc(im3) {
                    NEXT_IMAGE.signal(pool);
                }
                Timer::after(Duration::from_millis(500)).await;

                if let Some(new_signal) = NEW_IMAGE_RECEIVED.wait().now_or_never() {
                    last_signal = new_signal;
                    now = Instant::now();
                }
                if now.duration_since(last_signal) < Duration::from_secs(5) {
                    break;
                }
            }
        }
    }
}
