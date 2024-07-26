use crate::task::state::CooperativeState;

use core::{
    cell::{RefCell, RefMut},
    ptr::NonNull,
};

pub struct CooperativeTask {
    pub cooperative_task: fn(RefMut<dyn CooperativeState>) -> Cooperation,
    pub program_counter: Option<NonNull<*const u8>>,
    pub state: RefCell<dyn CooperativeState>,
}

pub enum Cooperation<T = ()> {
    Working(T),
    Done,
}

/// Save the program counter and
#[inline(always)]
pub fn yield_here<T>(state: RefMut<dyn CooperativeState>, work: T) -> Cooperation<T> {
    Cooperation::Working(work)
}

macro_rules! cooperative_task {
    ($resume_pc:expr) => {
        if let Some(pc) = $resume_pc {
            unsafe { crate::debug::jump::jump_pc!(pc.as_ptr()) }
        }
    };
}
pub(crate) use cooperative_task;
