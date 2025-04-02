use crate::matrix::Matrix;
use embassy_sync::mutex::Mutex;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use crate::{image::BLUE, Color, Image};
use embassy_time::{Timer,Duration,Ticker};
use embassy_stm32::{gpio::*,peripherals::PB14,};
#[embassy_executor::task]
pub async fn display(mut matrix : Matrix<'static>,mut_image:&'static  Mutex<ThreadModeRawMutex, Image>){
    let mut ticker = Ticker::every(Duration::from_hz(80*8)); 
    let mut buffer : [Color;64] = [BLUE;64];
    
    loop {
        {
            let image = mut_image.lock().await;
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