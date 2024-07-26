use crate::{
    debug::{console::debug_println, jump::save_pc},
    task::state::CooperativeState,
};

use core::{
    cell::{Cell, RefCell, RefMut},
    ptr::NonNull,
};

pub struct CooperativeTask<S: CooperativeState> {
    pub cooperative_task: fn(&Self, RefMut<dyn CooperativeState>) -> Cooperation,
    pub program_counter: Cell<Option<NonNull<u16>>>,
    pub state: RefCell<S>,
}
impl<S: CooperativeState> CooperativeTask<S> {
    pub fn update_program_counter(&self, to: Option<NonNull<u16>>) {
        self.program_counter.set(to);
    }
}

pub enum Cooperation<T = ()> {
    Working(T),
    Done,
}

/// Save the program counter and
macro_rules! yield_here {
    ($task:expr, $work:expr) => {
        let mut pc = crate::debug::jump::save_pc!();
        unsafe {
            $task.update_program_counter(Some(core::ptr::NonNull::new_unchecked(
                &mut pc as *mut u16,
            )))
        }
        return Cooperation::Working($work);
    };
}
pub(crate) use yield_here;

macro_rules! cooperative_task {
    ($resume_pc:expr) => {
        debug_println!(
            "resuming from {:x}",
            $resume_pc.map(|pc| pc.addr().into()).unwrap_or(0_usize)
        );
        arduino_hal::delay_ms(100);
        if let Some(pc) = $resume_pc {
            unsafe { crate::debug::jump::jump_pc!(pc.as_ptr()) }
        }
    };
}
pub(crate) use cooperative_task;

pub struct Scheduler {}
impl Scheduler {
    pub fn schedule<S: CooperativeState>(task: &CooperativeTask<S>) {
        debug_println!("scheduling task!");
        (task.cooperative_task)(task, task.state.borrow_mut());
    }
}
