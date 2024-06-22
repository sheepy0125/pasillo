//! Pasillo: A kernel made for the `atmega2560` (Arduino MEGA).

#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(trait_alias)]
#![feature(never_type)]
#![feature(stmt_expr_attributes)]
#![feature(generic_const_exprs)]
#![feature(strict_provenance)]
#![feature(non_null_convenience)]
#![feature(generic_nonzero)]
#![feature(maybe_uninit_uninit_array)]
#![feature(const_maybe_uninit_uninit_array)]
#![feature(ptr_metadata)]
#![feature(pattern)]
#![feature(split_at_checked)]

#[macro_use]
extern crate require_unsafe_in_body;
// #[macro_use]
// extern crate packed_struct;

use core::str::FromStr;

use console::set_console;

use debug::memory::add_marker;
use types::{magic::Magic, string::PStackStr};

use arduino_hal::{default_serial, delay_ms};

pub mod console;
pub mod debug;
pub mod panic;
pub mod shared;
pub mod task;
pub mod types;
pub mod utils;

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
