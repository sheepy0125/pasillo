//! Array types; notably, [`PStackArrUnchecked`].

use crate::types::magic::Magic;

use core::mem::MaybeUninit;

/// A stack-allocated array with a max length of `LEN`. Each element is stored as a `MaybeUninit`,
/// and `len` creates a contract that all elements in `&self.inner[0..len]` are valid.
///
/// See also [`crate::utils::decompose_uninit_array`] for self.inner`.
///
/// # Safety
/// *Always* update `len` if `inner` is updated.
#[repr(C)]
pub struct PStackArrUnchecked<T, const LEN: usize>
where
    T: Sized,
{
    _magic: [u8; 4],
    pub len: usize,
    pub inner: [MaybeUninit<T>; LEN],
}

unsafe impl<T, const LEN: usize> Magic<4> for PStackArrUnchecked<T, LEN>
where
    T: Sized,
{
    const MAGIC: [u8; 4] = [u8::MAX, b'a', b'r', b'y'];
}

impl<T, const LEN: usize> PStackArrUnchecked<T, LEN>
where
    T: Sized,
{
    pub const fn new() -> Self {
        Self {
            _magic: [0; 4],
            len: 0,
            inner: MaybeUninit::uninit_array::<LEN>(),
        }
    }
}
impl<T, const LEN: usize> Default for PStackArrUnchecked<T, LEN>
where
    T: Sized,
{
    fn default() -> Self {
        Self::new()
    }
}
