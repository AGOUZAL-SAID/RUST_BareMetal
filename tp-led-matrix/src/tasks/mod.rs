use core::panic;

use crate::{Color, Image, POOL};
use crate::{NEXT_IMAGE, matrix::Matrix};
use futures::future::FutureExt;
use embassy_stm32::{gpio::*, peripherals::PB14};
use embassy_stm32::{
    peripherals::{DMA1_CH4, DMA1_CH5, PB6, PB7, USART1},
    usart::{Config, Uart},
};
use embassy_time::{Duration, Ticker, Timer};
#[embassy_executor::task]
pub async fn display(mut matrix: Matrix<'static>) {
    let mut ticker = Ticker::every(Duration::from_hz(80 * 8));
    let mut buffer: [Color; 64] = [Color::default(); 64];
    let mut image = NEXT_IMAGE.wait().await;
    loop {
        {   
            if let Some(v) = NEXT_IMAGE.wait().now_or_never() {
                image = v;
            }
            for row in 0..8 {
                buffer[row * 8..(row + 1) * 8].copy_from_slice(image.row(row));
            }
        }
        let new_image = Image::new_im(buffer);
        matrix.display_image(&new_image, &mut ticker).await;
    }
}
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
        Timer::after_millis(1000).await;
    }
}

#[embassy_executor::task]
pub async fn serial_receiver(
    usart: USART1,
    pb7: PB7,
    pb6: PB6,
    dma1_tx: DMA1_CH4,
    dma1_rx: DMA1_CH5,
    config_uart: Config,
) {
    embassy_stm32::bind_interrupts!(struct Irqs {
        USART1 => embassy_stm32::usart::InterruptHandler<embassy_stm32::peripherals::USART1>;
    });
    let mut usart1: Uart<'static, embassy_stm32::mode::Async> =
        Uart::new(usart, pb7, pb6, Irqs, dma1_tx, dma1_rx, config_uart).unwrap();
    let mut word: [u8; 1] = [0; 1];
    let mut count = 0;
    loop {
        let mut image;

        if let Ok(pool) = POOL.alloc(Image::default()){image = pool;} else {
            panic!("Is not BOX<POOL>");}
        
        usart1.read(&mut word).await.unwrap();
        if word[0] == 0xff {
            usart1.read(image.as_mut()).await.unwrap();

            loop {
                if image.as_mut()[count] == 0xff {
                    image.as_mut().rotate_left(count + 1);
                    usart1.read(&mut image.as_mut()[192 -count - 1..]).await.unwrap();
                    count = 0;
                } else if count == 191 {
                    count = 0;
                    break;
                } else {
                    count += 1;
                }
            }
            NEXT_IMAGE.signal(image);
        }
    }
}
