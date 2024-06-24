//! Pasillo: A kernel made for the `atmega2560` (Arduino MEGA).

#![no_std]
#![no_main]
#![feature(never_type)]
#![feature(strict_provenance)]
#![feature(maybe_uninit_uninit_array)]
#![feature(const_maybe_uninit_uninit_array)]
#![feature(split_at_checked)]

pub mod console;
pub mod debug;
pub mod panic;
pub mod shared;
pub mod task;
pub mod types;
pub mod utils;

use console::{println, set_console};
use debug::memory::add_marker;
use types::{error::PError, magic::Magic, string::PStr};

use arduino_hal::{default_serial, delay_ms};

#[macro_use]
extern crate require_unsafe_in_body;

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);
    let serial = default_serial!(peripherals, pins, shared::BAUD_RATE);
    set_console(serial);

    unsafe {
        add_marker(
            "markers",
            core::ptr::addr_of!(debug::memory::MARKERS) as *const u8,
        )
    }

    let str = "1234";
    unsafe { add_marker("str", core::ptr::addr_of!(str) as *const u8) }
    let error = PError::new(types::error::PErrorVariant::Unknown, PStr::from_str(str));
    unsafe { add_marker("error", core::ptr::addr_of!(error) as *const u8) }

    assert!(Magic::is_magic(core::ptr::addr_of!(error)));
    println!("{}", error.context.as_ref());

    loop {
        delay_ms(1000);
    }
}
