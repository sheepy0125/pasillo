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

use console::set_console;
use core::str::FromStr;
use debug::memory::add_marker;
use types::{magic::Magic, string::PStackStr};

use arduino_hal::{default_serial, delay_ms};

#[macro_use]
extern crate require_unsafe_in_body;

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);
    let serial = default_serial!(peripherals, pins, shared::BAUD_RATE);
    let mut monitor = debug::interactive::HallwayMonitor::new();
    set_console(serial);

    let str = PStackStr::<32>::from_str("1234").unwrap();
    unsafe { add_marker("str", core::ptr::addr_of!(str) as *const u8) }
    unsafe { monitor.interactive() };
    assert!(Magic::is_magic(core::ptr::addr_of!(str)));

    loop {
        delay_ms(1000);
    }
}
