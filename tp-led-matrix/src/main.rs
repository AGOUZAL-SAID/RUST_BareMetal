#![feature(type_alias_impl_trait)]
#![no_std]
#![no_main]
use defmt_rtt as _;
use embassy_stm32 as _; // Just to link it in the executable (it provides the vector table)
use embassy_stm32::Config;
use embassy_stm32::rcc::*;
use panic_probe as _;
use tp_led_matrix::image::BLUE;
use tp_led_matrix::image::GREEN;
use tp_led_matrix::image::RED;
use tp_led_matrix::matrix::Matrix;
use tp_led_matrix::NEXT_IMAGE;
use tp_led_matrix::{Image,POOL};
use tp_led_matrix::tasks::serial_receiver;
use tp_led_matrix::tasks::{blinker, display};
use heapless::pool::boxed::BoxBlock;
use embassy_time::Timer;

#[embassy_executor::main]
async fn main(s: embassy_executor::Spawner) {
    defmt::info!("defmt correctly initialized");

    // Setup the clocks at 80MHz using HSI (by default since HSE/MSI
    // are not configured): HSI(16MHz)×10/2=80MHz. The flash wait
    // states will be configured accordingly.
    let mut config = Config::default();
    config.rcc.hsi = true;
    config.rcc.pll = Some(Pll {
        source: PllSource::HSI,
        prediv: PllPreDiv::DIV1,
        mul: PllMul::MUL10,
        divp: None,
        divq: None,
        divr: Some(PllRDiv::DIV2), // 16 * 10 / 2 = 80MHz
    });
    config.rcc.sys = Sysclk::PLL1_R;

    let p = embassy_stm32::init(config);

    let mut config_uart = embassy_stm32::usart::Config::default();
    config_uart.baudrate = 38_400;
    config_uart.data_bits = embassy_stm32::usart::DataBits::DataBits8;
    config_uart.parity = embassy_stm32::usart::Parity::ParityNone;
    config_uart.stop_bits = embassy_stm32::usart::StopBits::STOP1;

    let my_matrix = Matrix::new(
        p.PA2, p.PA3, p.PA4, p.PA5, p.PA6, p.PA7, p.PA15, p.PB0, p.PB1, p.PB2, p.PC3, p.PC4, p.PC5,
    )
    .await;
    unsafe {
        const BLOCK: BoxBlock<Image> = BoxBlock::new();
        static mut MEMORY: [BoxBlock<Image>; 3] = [BLOCK; 3];
        // By defaut, mutable reference static data is forbidden. We want
        // to allow it.
        #[allow(static_mut_refs)]
        for block in &mut MEMORY {
          POOL.manage(block);
        }
    }
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
    s.spawn(display(my_matrix)).unwrap();
    loop{
        if let Ok(pool) = POOL.alloc(Image::gradient(RED.gamma_correct())){
            NEXT_IMAGE.signal(pool);
        }
        Timer::after_millis(1000).await;
        if let Ok(pool) = POOL.alloc(Image::gradient(GREEN.gamma_correct())){
            NEXT_IMAGE.signal(pool);
        }
        Timer::after_millis(1000).await;
        if let Ok(pool) = POOL.alloc(Image::gradient(BLUE.gamma_correct())){
            NEXT_IMAGE.signal(pool);
        }
        Timer::after_millis(1000).await;
    }
}
