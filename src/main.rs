#![no_std]
#![no_main]

use panic_halt;
use teensy4_bsp as bsp;
use bsp::rt::entry;
use log;

use embedded_hal::digital::v2::ToggleableOutputPin;
use bsp::hal::i2c::*;

use mcp9600::*;
use mcp9600::register_file::*;

#[entry]
fn main() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();
    peripherals.log.init(Default::default());
    bsp::delay(5000);

    log::info!("Enabling I2C clocks...");
    let (_, _, i2c3_builder, _) = peripherals.i2c.clock(
        &mut peripherals.ccm.handle,
        bsp::hal::ccm::i2c::ClockSelect::OSC,
        bsp::hal::ccm::i2c::PrescalarSelect::DIVIDE_1,
    );
    log::info!("Constructing I2C3 instance on pins 16 and 17...");
    let mut i2c3 = i2c3_builder.build(peripherals.pins.p16.alt1(), peripherals.pins.p17.alt1());
    if let Err(err) = i2c3.set_bus_idle_timeout(core::time::Duration::from_micros(200)) {
        log::warn!("Error when setting bus idle timeout: {:?}", err);
    }
    if let Err(err) = i2c3.set_pin_low_timeout(core::time::Duration::from_millis(1)) {
        log::warn!("Error when setting pin low timeout: {:?}", err);
    }

    let mut registers = RegisterFile::new(i2c3, 0x66);
    let mut mcp9600 = Mcp9600::new(registers, ThermocoupleType::TypeK, FilterCoefficients::NoFilter);

    log::info!("Starting I/O loop...");
    loop {
        bsp::delay(1_000);

        peripherals.led.toggle().unwrap();
        log::info!("toggling led");

        let fp_temp = mcp9600.read_temp();
        let ipart = {(fp_temp & 0xFFF0) >> 4} as f32;
        let fpart = {(fp_temp & 0xF)} as f32;
        let temp = ipart + fpart / {(1 << 4)} as f32;
        log::info!("temp: {}", temp);
    }
}
