use crate::shared::{UsbSerial, BAUD_RATE};

use arduino_hal::{default_serial, delay_ms};
use core::panic::PanicInfo;
use ufmt::uwriteln;

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    delay_ms(1_000_u16);

    let peripherals = unsafe { arduino_hal::Peripherals::steal() };
    let pins = arduino_hal::pins!(peripherals);
    let serial: &mut UsbSerial = &mut default_serial!(peripherals, pins, BAUD_RATE);

    // Print out panic location
    // For whatever reason, when not running in release mode then we get
    // garbage printed out for the file, line, and column
    match info.location() {
        #[cfg(not(debug_assertions))]
        Some(loc) => uwriteln!(
            serial,
            "PANICKED {}:{}:{}",
            loc.file(),
            loc.line(),
            loc.column()
        )
        .unwrap(),
        #[cfg(debug_assertions)]
        Some(loc) => uwriteln!(
            serial,
            "PANICKED: not release mode, garbage: {}",
            loc.file()
        )
        .unwrap(),
        None => uwriteln!(serial, "Panicked! No information.").unwrap(),
    }

    let mut led = pins.d13.into_output();
    loop {
        led.toggle();
        delay_ms(500);
    }
}
