//! Debug serial console
#![allow(unused_macros)]

use crate::{shared::UsbSerial, types::string::PStackStr};

use core::{cell::RefCell, str::FromStr as _};
use avr_device::interrupt::{self, Mutex};

pub static CONSOLE: Mutex<RefCell<Option<UsbSerial>>> = interrupt::Mutex::new(RefCell::new(None));

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
            ret.len += 1;
        }
    });
    helper_print!("", "\n", "{}", ret.as_ref());
    ret
}

// Print macros

macro_rules! helper_print {
    ($before:expr, $after:expr, $($t:tt)*) => {
        avr_device::interrupt::free(
            |critical_section| {
                if let Some(console) = crate::debug::console::CONSOLE.borrow(critical_section).borrow_mut().as_mut() {
                    let _ = ufmt::uwrite!(console, "{}", $before);
                    let _ = ufmt::uwrite!(console, $($t)*);
                    let _ = ufmt::uwrite!(console, "{}", $after);
                };
            },
        )
    };
}
macro_rules! trace {
    ($($t:tt)*) => {
        crate::debug::console::helper_print!("[trace] ", '\n', $($t)*)
    }
}
macro_rules! debug_println {
    ($($t:tt)*) => {
        crate::debug::console::helper_print!("[debug] ", '\n', $($t)*)
    }
}
macro_rules! debug_print {
    ($($t:tt)*) => {
        crate::debug::console::helper_print!("[debug] ", "", $($t)*)
    }
}

#[allow(unused_imports)]
pub(crate) use helper_print;
#[allow(unused_imports)]
pub(crate) use debug_println;
#[allow(unused_imports)]
pub(crate) use debug_print;
#[allow(unused_imports)]
pub(crate) use trace;
