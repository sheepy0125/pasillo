//! Stack locations for tasks

use core::arch::asm;

const DEFAULT_STACK_POS: usize = 0x21ff;

/// Set the top of the stack to `location`.
#[allow(unused_macros)]
macro_rules! jump_to_stack {
    ($location:expr) => {
        core::arch::asm!(
            "out __SP_L__, r30",
            "out __SP_H__, r31",
            in("r30") crate::utils::low!($location),
            in("r31") crate::utils::high!($location),
        )
    }
}
#[allow(unused_macros)]
/// Set the program counter to `location`.
macro_rules! jump_pc {
    ($location:expr) => {
        core::arch::asm!(
            "ijmp",
            in("r30") crate::utils::low!($location as *const _),
            in("r31") crate::utils::high!($location as *const _),
            options(noreturn)
        )
    }
}
#[allow(unused_imports)]
pub(crate) use jump_pc;
/// Find the program counter.
#[allow(unused_macros)]
macro_rules! save_pc {
    () => {{
        let msb: u8;
        let lsb: u8;
        unsafe {
            core::arch::asm!(
                "rcall .",
                "pop r16",
                "pop r17",
                "pop r18",
                out("r17") msb,
                out("r18") lsb
            )
        };
        ((msb as u16) << 8) | lsb as u16
    }};
}
#[allow(unused_imports)]
pub(crate) use save_pc;
/// Call a function, resetting the stack pointer to its default position.
#[allow(unused)]
#[require_unsafe_in_body]
pub unsafe fn call(location: *const fn() -> ()) {
    // Reset the stack pointer to its default position
    unsafe { jump_to_stack!(core::ptr::from_exposed_addr(DEFAULT_STACK_POS)) }
    // Jump to function pointer
    unsafe { jump_pc!(location) }
}
