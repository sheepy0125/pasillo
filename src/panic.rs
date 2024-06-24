//! Panic handler!

use crate::{
    console::{println, set_console},
    debug::interactive::HallwayMonitor,
    shared::{UsbSerial, BAUD_RATE},
};

use arduino_hal::{default_serial, delay_ms};
use core::panic::PanicInfo;

/// Panic and run [`HallwayMonitor`].
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    // Avoid race condition with the serial handle
    delay_ms(1_000_u16);

    let peripherals = unsafe { arduino_hal::Peripherals::steal() };
    let pins = arduino_hal::pins!(peripherals);
    let serial: UsbSerial = default_serial!(peripherals, pins, BAUD_RATE);
    set_console(serial);

    // Print out panic location
    // For whatever reason, when not running in release mode then we get
    // garbage printed out for the file, line, and column
    match info.location() {
        #[cfg(not(debug_assertions))]
        Some(loc) => println!("PANICKED {}:{}:{}", loc.file(), loc.line(), loc.column()),
        #[cfg(debug_assertions)]
        Some(loc) => println!("PANICKED: not release mode, garbage: {}", loc.file()),
        None => println!("Panicked! No information."),
    }

    #[cfg(debug_assertions)]
    unsafe {
        HallwayMonitor::new().interactive()
    };
    #[cfg(not(debug_assertions))]
    println!("Run with debug assertions to start Hallway Monitor.");

    let mut led = pins.d13.into_output();
    loop {
        led.toggle();
        delay_ms(500);
    }
}
