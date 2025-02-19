use std::{
    mem::size_of,
    ptr::{slice_from_raw_parts, slice_from_raw_parts_mut},
};

#[inline]
pub fn as_bytes<T: Sized>(x: &T) -> &[u8] {
    unsafe { &*slice_from_raw_parts(x as *const _ as *const u8, size_of::<T>()) }
}

#[inline]
pub unsafe fn as_bytes_mut<T: Sized>(x: &mut T) -> &mut [u8] {
    &mut *slice_from_raw_parts_mut(x as *mut _ as *mut u8, size_of::<T>())
}

#[inline]
pub unsafe fn from_bytes<T: Sized>(x: &[u8]) -> &T {
    &*(x.as_ptr() as *const _)
}

#[inline]
pub unsafe fn from_bytes_mut<T: Sized>(x: &mut [u8]) -> &mut T {
    &mut *(x.as_mut_ptr() as *mut _)
}
