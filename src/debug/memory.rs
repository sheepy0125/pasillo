//! A global table of pointers to known "markers."

use crate::types::array::PStackArrUnchecked;

use core::mem::MaybeUninit;

pub type MarkerEntry = (&'static str, *const u8);
#[cfg(debug_assertions)]
pub const NUM_MARKERS: usize = 16;
#[cfg(not(debug_assertions))]
pub const NUM_MARKERS: usize = 0;
pub static mut MARKERS: PStackArrUnchecked<MarkerEntry, NUM_MARKERS> = PStackArrUnchecked::new();

/// Add a marker to the global `MARKERS` array. If the array is full, nothing will occur.
#[inline(never)]
#[require_unsafe_in_body]
#[cfg(debug_assertions)]
pub unsafe fn add_marker(name: &'static str, ptr: *const u8) {
    if unsafe { MARKERS.len } >= NUM_MARKERS {
        return;
    }
    unsafe { MARKERS.inner[MARKERS.len] = MaybeUninit::new((name, ptr)) };
    unsafe { MARKERS.len += 1 };
}
#[cfg(not(debug_assertions))]
#[require_unsafe_in_body]
pub unsafe fn add_marker(_: &'static str, _: *const u8) {}
