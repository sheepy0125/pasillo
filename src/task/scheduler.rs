#[repr(C)]
pub struct Task {
    /// Pointer to the highest stack element
    pub stack_top: *const u8,
    pub stack_start: *const u8,
    pub stack_end: *const u8,
    /// Start run time in micros
    pub start_runtime: u32,
}
