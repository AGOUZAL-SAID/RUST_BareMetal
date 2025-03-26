#![feature(type_alias_impl_trait)]
#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_stm32 as _; // Just to link it in the executable (it provides the vector table)
use embassy_stm32::Config;
use embassy_stm32::rcc::*;
use panic_probe as _;
use tp_led_matrix::Image;
use tp_led_matrix::image::BLUE;
use tp_led_matrix::matrix::{Matrix, blinker};
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

    let bleu = BLUE;

    let im = Image::gradient(bleu);
    let my_matrix = Matrix::new(
        p.PA2, p.PA3, p.PA4, p.PA5, p.PA6, p.PA7, p.PA15, p.PB0, p.PB1, p.PB2, p.PC3, p.PC4, p.PC5,
    );
    my_matrix.await.display_image(&im);
    s.spawn(blinker(p.PB14)).unwrap();
    }
