//! Pasillo: A kernel made for the `atmega2560` (Arduino MEGA).

#![no_std]
#![no_main]
#![feature(panic_info_message)]
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

use core::str::FromStr;

use console::set_console;
use debug::memory::{add_marker, add_marker_manual};

use arduino_hal::default_serial;
use types::string::PStackStr;

#[macro_use]
extern crate require_unsafe_in_body;

#[arduino_hal::entry]
fn main() -> ! {
    unsafe { add_marker_manual("main", __avr_device_rt_main as *const u8) };
    let peripherals = unsafe { arduino_hal::Peripherals::steal() };
    let pins = arduino_hal::pins!(peripherals);
    let serial = default_serial!(peripherals, pins, shared::BAUD_RATE);
    set_console(serial);

    let s = PStackStr::<64>::from_str("hello!!!").unwrap();
    add_marker!("s", s);

    panic!("init ended");
}
