//! A service to read input from some buttons.

use core::cell::RefMut;

use arduino_hal::delay_ms;

use crate::{
    debug::{console::debug_println, memory::add_marker_manual},
    task::{
        scheduler::{cooperative_task, yield_here, Cooperation, CooperativeTask},
        state::CooperativeState,
    },
};

#[derive(Default)]
pub struct InputReaderState {
    pub button_mask: usize,
}
impl CooperativeState for InputReaderState {}

#[allow(unreachable_code)]
pub fn input_reader_task(
    task: &CooperativeTask<InputReaderState>,
    state: RefMut<dyn CooperativeState>,
) -> Cooperation<()> {
    unsafe { add_marker_manual("input reader task", input_reader_task as *const _) }

    cooperative_task!(task.program_counter.get());

    debug_println!("running task input reader!");
    delay_ms(1000);

    debug_println!("yielding from input reader!");
    yield_here!(task, ());

    debug_println!("yielded, continuing!");
    delay_ms(1000);
    debug_println!("done!");

    Cooperation::Done
}
