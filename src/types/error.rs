//! Error types; notably, [`PError`].

use crate::types::{magic::Magic, string::PStr};

pub type PStaticResult<T> = Result<T, PError<0>>;

#[repr(C)]
pub struct PError<const STACK_STR_LEN: usize = 64> {
    _magic: [u8; 4],
    pub variant: PErrorVariant,
    pub context: PStr<'static, STACK_STR_LEN>,
}
impl<const STACK_STR_LEN: usize> PError<STACK_STR_LEN> {
    pub fn new(variant: PErrorVariant, context: PStr<'static, STACK_STR_LEN>) -> Self {
        Self {
            _magic: Self::MAGIC,
            variant,
            context,
        }
    }
}
unsafe impl<const STACK_STR_LEN: usize> Magic<4> for PError<STACK_STR_LEN> {
    const MAGIC: [u8; 4] = [b'e', b'r', b'r', b'!'];
}

#[repr(u8)]
pub enum PErrorVariant {
    Unknown = 0,
    Stdio = 1,
}
