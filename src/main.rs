//! Pasillo: A kernel made for the `atmega2560` (Arduino MEGA).

#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(never_type)]
#![feature(strict_provenance)]
#![feature(exposed_provenance)]
#![feature(maybe_uninit_uninit_array)]
#![feature(const_maybe_uninit_uninit_array)]
#![feature(split_at_checked)]
#![feature(const_trait_impl)]
#![feature(effects)]
#![feature(asm_experimental_arch)]
#![feature(abi_avr_interrupt)]

pub mod debug;
pub mod panic;
pub mod services;
pub mod shared;
pub mod task;
pub mod types;
pub mod utils;

use core::{
    cell::{Cell, RefCell},
    ptr::NonNull,
};

use debug::{
    console::debug_println,
    memory::{add_marker, add_marker_manual},
};

use arduino_hal::{default_serial, delay_ms};
use services::input_reader::{input_reader_task, InputReaderState};
use task::{
    interrupt::{millis, millis_init},
    scheduler::{CooperativeTask, Scheduler},
};

#[macro_use]
extern crate require_unsafe_in_body;

#[arduino_hal::entry]
fn main() -> ! {
    unsafe { add_marker_manual("main", __avr_device_rt_main as *const _) }

    // Initialize peripherals
    let peripherals = unsafe { arduino_hal::Peripherals::steal() };
    let pins = arduino_hal::pins!(peripherals);
    let serial = default_serial!(peripherals, pins, shared::BAUD_RATE);
    debug::console::set_console(serial);
    debug_println!("running!");

    // Enable interrupts
    unsafe {
        millis_init(peripherals.TC0);
        avr_device::interrupt::enable();
    }

    let input_task = CooperativeTask {
        cooperative_task: input_reader_task,
        state: RefCell::new(InputReaderState::default()),
        program_counter: Cell::new(None),
    };
    Scheduler::schedule(&input_task);
    Scheduler::schedule(&input_task);

    panic!("init ended")
}
