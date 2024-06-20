#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(trait_alias)]
#![feature(stmt_expr_attributes)]

use console::set_console;

use arduino_hal::{default_serial, delay_ms};

pub mod console;
pub mod panic;
pub mod pins;
pub mod rpm;
pub mod shared;

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);
    let serial = default_serial!(peripherals, pins, shared::BAUD_RATE);
    set_console(serial);

    // Main loop

    loop {
        delay_ms(1000);
    }
}
