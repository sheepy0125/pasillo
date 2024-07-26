//! A service to read input from some buttons.

use core::{cell::RefMut, ptr::NonNull};

use crate::task::{
    scheduler::{cooperative_task, yield_here, Cooperation},
    state::CooperativeState,
};

pub struct InputReaderState {
    pub button_mask: usize,
}
impl CooperativeState for InputReaderState {}

#[allow(unreachable_code)]
pub fn input_reader_task(
    state: RefMut<dyn CooperativeState>,
    resume_pc: Option<NonNull<*const u8>>,
) -> Cooperation<()> {
    cooperative_task!(resume_pc);

    return yield_here(state, ());

    Cooperation::Done
}
