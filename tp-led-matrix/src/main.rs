#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embassy_stm32 as _;   // Just to link it in the executable (it provides the vector table)
use panic_rtt_target as _;
use rtt_target::{rprintln,rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello, world!");
    panic!("The program stopped");
 }