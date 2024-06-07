use rp2040_hal as hal;
use embedded_hal::digital::OutputPin;
use crate::rp::delay;

pub fn entry() {
    let mut pac = hal::pac::Peripherals::take().unwrap();
    
    let sio = hal::Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0, 
        pac.PADS_BANK0, 
        sio.gpio_bank0, 
        &mut pac.RESETS,
    );

    let mut led = pins.gpio13.into_push_pull_output();

    let mut neopixel = pins.gpio4.into_push_pull_output();

    loop{
        led.set_high().unwrap();
        delay::cycles(50_000);
        led.set_low().unwrap();
        delay::cycles(50_000);
        neopixel.set_high().unwrap();
        delay::cycles(50_000);
        neopixel.set_low().unwrap();
        delay::cycles(50_000);
    }
}
