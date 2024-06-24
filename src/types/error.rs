//! Error types; notably, [`PError`].

use crate::types::{magic::Magic, string::PStr};

#[repr(C)]
pub struct PError {
    _magic: [u8; 4],
    pub variant: PErrorVariant,
    pub context: PStr<'static, 64>,
}
impl PError {
    pub fn new(variant: PErrorVariant, context: PStr<'static, 64>) -> Self {
        Self {
            _magic: Self::MAGIC,
            variant,
            context,
        }
    }
}
unsafe impl Magic<4> for PError {
    const MAGIC: [u8; 4] = [b'e', b'r', b'r', b'!'];
}

#[repr(u8)]
pub enum PErrorVariant {
    Unknown = 0,
}
