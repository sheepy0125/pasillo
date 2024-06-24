use super::scheduler::Niceness;
use crate::types::string::PStackStr;

#[allow(dead_code)] // fixme
#[repr(C)]
pub struct KTaskState {
    niceness: Niceness,
    parent_id: u32,
    id: u32,
    cwd: PStackStr<32>,
}
