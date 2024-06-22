//! Currently, this only serves as a sequential serial interface; a debug console.

#![allow(unused_imports, unused_macros)]

use crate::{shared::UsbSerial, types::string::PStackStr};

use avr_device::interrupt::{self, Mutex};
use core::{cell::RefCell, str::FromStr};

pub static CONSOLE: Mutex<RefCell<Option<UsbSerial>>> = interrupt::Mutex::new(RefCell::new(None));

pub const DIGIT_LOOKUP: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];

macro_rules! println {
    ($($t:tt)*) => {
        avr_device::interrupt::free(
            |critical_section| {
                if let Some(console) = crate::console::CONSOLE.borrow(critical_section).borrow_mut().as_mut() {
                    let _ = ufmt::uwriteln!(console, $($t)*);
                };
            },
        )
    };
}
macro_rules! print {
    ($($t:tt)*) => {
        avr_device::interrupt::free(
            |critical_section| {
                if let Some(console) = crate::console::CONSOLE.borrow(critical_section).borrow_mut().as_mut() {
                    let _ = ufmt::uwrite!(console, $($t)*);
                };
            },
        )
    };
}
macro_rules! debug {
    ($($t:tt)*) => {
        // TODO: Call println! macro from here and compile time check
        if crate::shared::DEBUG {
            avr_device::interrupt::free(
                |critical_section| {
                    if let Some(console) = crate::console::CONSOLE.borrow(critical_section).borrow_mut().as_mut() {
                        let _ = ufmt::uwriteln!(console, $($t)*);
                    };
                }
            )
        }
    };
}
macro_rules! trace {
    ($($t:tt)*) => {
        // TODO: Call println! macro from here and compile time check
        if crate::shared::TRACE {
            avr_device::interrupt::free(
                |critical_section| {
                    if let Some(console) = crate::console::CONSOLE.borrow(critical_section).borrow_mut().as_mut() {
                        let _ = ufmt::uwriteln!(console, $($t)*);
                    };
                }
            )
        }
    }
}

pub fn set_console(console: UsbSerial) {
    interrupt::free(|cs| {
        *CONSOLE.borrow(cs).borrow_mut() = Some(console);
    })
}

/// Block until a line is read from the console, up until `LEN` characters.
/// A "newline" is defined as a carriage return, line feed, end of file, or NUL byte.
///
/// # Safety
/// This should only be used in debug situations within the kernel, as it *blocks*.
pub unsafe fn read_line<const LEN: usize>() -> PStackStr<LEN> {
    let mut ret = PStackStr::from_str("").unwrap();
    interrupt::free(|cs| {
        let mut console_cell = CONSOLE.borrow(cs).borrow_mut();
        let console = console_cell.as_mut().unwrap();
        for char in ret.inner.iter_mut() {
            let read = console.read_byte();
            if read == b'\r' /* CR*/
                || read == b'\n' /* LF */
                || read == 0x4 /* EOF */
                || read == 0x0
            {
                break;
            }
            *char = read;
        }
    });
    ret
}

pub(crate) use debug;
pub(crate) use print;
pub(crate) use println;
pub(crate) use trace;
