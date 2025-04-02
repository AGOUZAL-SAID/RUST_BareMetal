use crate::{Color, Image};
use crate::{IMAGE, matrix::Matrix};
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

    loop {
        {
            let image = IMAGE.lock().await;
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
    let mut buffer: [u8; 192] = [0; 192];
    let mut word: [u8; 1] = [0; 1];
    let mut count = 0;
    loop {
        usart1.read(&mut word).await.unwrap();
        if word[0] == 0xff {
            usart1.read(&mut buffer).await.unwrap();
            loop {
                if buffer[count] == 0xff {
                    buffer.rotate_left(count + 1);
                    usart1.read(&mut buffer[count + 1..]).await.unwrap();
                    count += 1;
                } else if count == 191 {
                    count = 0;
                    break;
                } else {
                    count += 1;
                }
            }
        }
        {
            let mut im = IMAGE.lock().await;
            *im.as_mut() = buffer;
        }
    }
}
