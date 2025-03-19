#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embassy_stm32 as _;   // Just to link it in the executable (it provides the vector table)

#[panic_handler]
fn panic_handler(_panic_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
#[entry]
fn main() -> ! {
    panic!("The program stopped");
}