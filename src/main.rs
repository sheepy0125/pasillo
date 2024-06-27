//! Pasillo: A kernel made for the `atmega2560` (Arduino MEGA).

#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(never_type)]
#![feature(strict_provenance)]
#![feature(maybe_uninit_uninit_array)]
#![feature(const_maybe_uninit_uninit_array)]
#![feature(split_at_checked)]
#![feature(const_trait_impl)]
#![feature(effects)]
#![feature(asm_experimental_arch)]

pub mod debug;
pub mod driver;
pub mod module;
pub mod panic;
pub mod shared;
pub mod task;
pub mod types;
pub mod utils;

use core::{alloc::Layout, hint::black_box};

use arduino_hal::default_serial;
use debug::{console::debug_println, memory::add_marker};
use task::stack::{jump_to_stack, STACK, STACK_LEN};

#[macro_use]
extern crate require_unsafe_in_body;

#[arduino_hal::entry]
fn main() -> ! {
    unsafe { debug::memory::add_marker_manual("main", __avr_device_rt_main as *const u8) };

    add_marker!("stack", STACK);
    unsafe {
        jump_to_stack(core::ptr::addr_of!(STACK[0]).add(STACK_LEN) as *const _);
    }

    let peripherals = unsafe { arduino_hal::Peripherals::steal() };
    let pins = arduino_hal::pins!(peripherals);
    let serial = default_serial!(peripherals, pins, shared::BAUD_RATE);
    debug::console::set_console(serial);

    let x = [b'G'; 128];
    add_marker!("x", x);

    panic!("")
}
