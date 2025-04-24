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
use embedded_graphics::Drawable;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::{Dimensions, Point};
use embedded_graphics::text::Text;
use futures::future::FutureExt;
use ibm437::IBM437_8X8_REGULAR;

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
            let mut ticker = Ticker::every(Duration::from_millis(60));
            // Otherwise, start the screensaver sequence
            defmt::info!("screensaver ON :)");
            let mut x = 8;
            let r = 224;
            let g = 176;
            let b = 255;
            loop {
                ticker.next().await;
                let mut im = Image::default();
                let text = "♥ FREE PALESTINE ♥";
                let style = MonoTextStyle::new(&IBM437_8X8_REGULAR, Rgb888::new(r, g, b));
                let text_object = Text::new(text, Point::new(x, 6), style);
                let size = text_object.bounding_box().size.width;
                text_object.draw(&mut im).unwrap();

                if let Some(new_signal) = NEW_IMAGE_RECEIVED.wait().now_or_never() {
                    last_signal = new_signal;
                    now = Instant::now();
                }
                if now.duration_since(last_signal) < Duration::from_secs(5) {
                    break;
                }
                if let Ok(pool) = POOL.alloc(im) {
                    NEXT_IMAGE.signal(pool);
                }
                x -= 1;
                if x == -(size as i32) {
                    x = 8;
                }
            }
        }
    }
}
