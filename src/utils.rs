//! Generic helper utilities.

use core::mem::MaybeUninit;

/// [`core::mem::transmute`] ensures that `sizeof::<Src>()` == `sizeof::<Dst>()`. In the context of
/// const generics (ref. https://github.com/rust-lang/rust/issues/47966), this cannot be guaranteed.
///
/// # Safety
/// Please ensure the transmuting is "sane!"
#[require_unsafe_in_body]
pub unsafe fn unchecked_size_transmute<Src, Dst>(source: Src) -> Dst
where
    Src: Sized,
    Dst: Sized,
{
    let destination = unsafe { core::ptr::read(core::ptr::addr_of!(source) as *const Dst) };
    core::mem::forget(source);
    destination
}

/// Unsafely transmutes a [`MaybeUninit::uninit_array`], masking the [`MaybeUninit`]s.
#[require_unsafe_in_body]
pub unsafe fn decompose_uninit_array<T, const LEN: usize>(
    array: [MaybeUninit<T>; LEN],
) -> [T; LEN] {
    unsafe { unchecked_size_transmute::<_, [T; LEN]>(array) }
}
