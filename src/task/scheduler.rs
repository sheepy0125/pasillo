use crate::{debug::console::debug_println, task::state::CooperativeState};

use core::{
    cell::{Cell, RefCell, RefMut},
    hint::black_box,
    ptr::NonNull,
    sync::atomic::AtomicBool,
};

pub static CONTINUING_FROM_YIELD: AtomicBool = AtomicBool::new(false);

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

/// Save the program counter into `task` and return a [`Cooperation::Working`].
macro_rules! yield_here {
    ($task:expr, $work:expr) => {
        let pc = crate::debug::jump::save_pc!();
        // When we resume a task, we'll resume from right here.
        // We only want to return when we haven't yielded from here yet.
        if !crate::task::scheduler::CONTINUING_FROM_YIELD
            .load(core::sync::atomic::Ordering::Relaxed)
        {
            // no [`core::sync::atomic::AtomicBool::fetch_and`] on AVR
            crate::task::scheduler::CONTINUING_FROM_YIELD
                .store(false, core::sync::atomic::Ordering::Relaxed);
            crate::debug::console::trace!("yield_here: program counter saved as 0x{:x}", pc);
            unsafe {
                $task
                    .update_program_counter(Some(core::ptr::NonNull::new_unchecked(pc as *mut u16)))
            }
            return Cooperation::Working($work);
        }
    };
}
pub(crate) use yield_here;

/// Resume from a yielded program counter
macro_rules! cooperative_task {
    ($resume_pc:expr) => {
        if let Some(pc) = $resume_pc {
            crate::debug::console::trace!(
                "cooperative_task: resuming from 0x{:x}",
                pc.as_ptr().addr()
            );
            crate::task::scheduler::CONTINUING_FROM_YIELD
                .store(true, core::sync::atomic::Ordering::Relaxed);
            unsafe { crate::debug::jump::jump_pc!(pc.as_ptr()) }
        }
    };
}
pub(crate) use cooperative_task;

pub struct Scheduler {}
impl Scheduler {
    pub fn schedule<S: CooperativeState>(task: &CooperativeTask<S>) {
        debug_println!("scheduling task!");
        black_box((task.cooperative_task)(task, task.state.borrow_mut()));
    }
}
