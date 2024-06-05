#![no_std]
#![no_main]

use rp2040_hal as hal;
use panic_halt as _;

use embedded_hal::digital::OutputPin;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

#[rp2040_hal::entry]
fn entry() -> ! {

    let mut pac = hal::pac::Peripherals::take().unwrap();
    
    let sio = hal::Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0, 
        pac.PADS_BANK0, 
        sio.gpio_bank0, 
        &mut pac.RESETS,
    );

    let mut led = pins.gpio13.into_push_pull_output();
    loop{
        led.set_high().unwrap();
    }
}
