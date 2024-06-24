//! Panic handler!

#[cfg(debug_assertions)]
use crate::debug::interactive::HallwayMonitor;
use crate::{
    console::{println, set_console},
    shared::{UsbSerial, BAUD_RATE},
};

use arduino_hal::{default_serial, delay_ms};
use core::panic::PanicInfo;

/// Panic and run [`HallwayMonitor`].
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    // Avoid race condition with the serial handle
    let peripherals = unsafe { arduino_hal::Peripherals::steal() };
    let pins = arduino_hal::pins!(peripherals);
    let serial: UsbSerial = default_serial!(peripherals, pins, BAUD_RATE);
    set_console(serial);

    // Print out panic location
    if let Some(message) = info.message() {
        println!("PANICKED! {}", message.as_str().unwrap_or_default());
    }
    if let Some(loc) = info.location() {
        println!("PANICKED! {}:{}:{}", loc.file(), loc.line(), loc.column());
    }

    #[cfg(debug_assertions)]
    unsafe {
        println!("Enter to start Hallway Monitor...");
        crate::console::read_line::<1>();
        HallwayMonitor::new().interactive()
    };
    #[cfg(not(debug_assertions))]
    println!("Run with debug assertions to start Hallway Monitor.");

    println!("Entering busy loop.");
    let mut led = pins.d13.into_output();
    loop {
        led.toggle();
        delay_ms(500);
    }
}
