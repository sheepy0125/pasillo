//! Error types; notably, [`PError`].

use crate::types::{magic::Magic, string::PStr};

pub struct PError {
    _magic: [u8; 4],
    pub variant: PErrorVariant,
    pub context: PStr<'static, 64>,
}
unsafe impl Magic<4> for PError {
    const MAGIC: [u8; 4] = [b'e', b'r', b'r', b'!'];
}

#[repr(u8)]
pub enum PErrorVariant {
    Unknown = 0,
}
