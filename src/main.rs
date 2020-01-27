#![no_std]
#![no_main]

use panic_halt;
use teensy4_bsp as bsp;
use bsp::rt::entry;
use log;

use embedded_hal::digital::v2::ToggleableOutputPin;

use mcp9600;

#[entry]
fn main() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();
    peripherals.log.init(Default::default());
    bsp::delay(5000);

    loop {
        bsp::delay(1_000);
        peripherals.led.toggle().unwrap();
        log::info!("toggling led");
    }
}
