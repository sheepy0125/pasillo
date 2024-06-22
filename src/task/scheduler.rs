use crate::types::error::PError;

/// A task's priority and niceness
#[repr(C)]
#[derive(Default)]
pub enum Niceness {
    #[default]
    LowPriority = 0,
    HighPriority = 1,
    /// Wait for the task to return execution
    Cooperative = 2,
}

pub trait KTask {
    fn change_niceness(to: Niceness) -> Result<(), PError>;
}
