//! String types, notably [`PStackStr`]

use crate::types::magic::Magic;

use core::str::FromStr;

/// A padded, null-terminated string that is stored on the stack with a fixed `LEN`.
/// The `inner` data is to be UTF-8. Derefs into `&str`.
#[repr(C)]
pub struct PStackStr<const LEN: usize> {
    pub _magic: [u8; 4],
    pub inner: [u8; LEN],
    /// Not guaranteed to be updated if `inner` is modified
    pub initial_len: usize,
}

unsafe impl<const STR_LEN: usize> Magic<4> for PStackStr<STR_LEN> {
    const MAGIC: [u8; 4] = [u8::MAX, b's', b't', b'r'];
}

impl<const LEN: usize> AsRef<str> for PStackStr<LEN> {
    fn as_ref(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.inner).trim_end_matches(0x0 as char) }
    }
}
impl<const LEN: usize> core::ops::Deref for PStackStr<LEN> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
impl<const LEN: usize> FromStr for PStackStr<LEN> {
    type Err = !;
    /// Wastefully converts from a string slice, copying the bytes up until `LEN`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inner = [0; LEN];

        let str_bytes = s.as_bytes();
        let len = match str_bytes.len() {
            l if l > LEN => LEN,
            l => l,
        };
        inner[0..len].clone_from_slice(&str_bytes[0..len]);

        Ok(Self {
            _magic: Self::MAGIC,
            initial_len: len,
            inner,
        })
    }
}

/// A string that could be a static str slice or custom string. The string type is determined through
/// a magic value; if none is present, the string is assumed to be a str slice.
///
/// The size of this union is always the stack length.
pub union PStr<'a, const STACK_LEN: usize = 0> {
    fixed: &'a str,
    stack: core::mem::ManuallyDrop<PStackStr<STACK_LEN>>,
}
impl<'a, const STACK_LEN: usize> From<PStackStr<STACK_LEN>> for PStr<'a, STACK_LEN> {
    fn from(value: PStackStr<STACK_LEN>) -> Self {
        Self {
            stack: core::mem::ManuallyDrop::new(value),
        }
    }
}
impl<'a, const STACK_LEN: usize> PStr<'a, STACK_LEN> {
    // We can't implement `FromStr` because its `from_str` method doesn't allow non-anonymous lifetimes.
    // And `str` has the `?Sized` supertrait, so `From<str>` wouldn't work.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(str: &'a str) -> Self {
        Self { fixed: str }
    }
}
impl<'a, const STACK_LEN: usize> AsRef<str> for PStr<'a, STACK_LEN> {
    fn as_ref(&self) -> &str {
        unsafe {
            if Magic::is_magic(core::ptr::addr_of!(self.stack) as *const PStackStr<STACK_LEN>) {
                self.stack.as_ref()
            } else {
                self.fixed
            }
        }
    }
}
