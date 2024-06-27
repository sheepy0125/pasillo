//! Stack locations for tasks

use core::arch::asm;

pub const STACK_LEN: usize = 512;
#[link_name = "stack"]
pub static mut STACK: [[u8; STACK_LEN]; 1] = [[0xff; STACK_LEN]; 1];

pub unsafe fn jump_to_stack(location: *const u8) {
    let addr = location.addr() as u32;

    let stack_pointer_low = (addr & u8::MAX as u32) as i8;
    let stack_pointer_high = (addr >> 8) as i8;

    asm!("out __SP_L__, {low}", "out __SP_H__, {high}", low = in(reg) stack_pointer_low, high = in(reg) stack_pointer_high);
}
