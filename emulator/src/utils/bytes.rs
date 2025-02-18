use std::{
    mem::size_of,
    ptr::{slice_from_raw_parts, slice_from_raw_parts_mut},
};

pub fn as_bytes<T: Sized>(x: &T) -> &[u8] {
    unsafe { &*slice_from_raw_parts(x as *const _ as *const u8, size_of::<T>()) }
}

pub unsafe fn as_bytes_mut<T: Sized>(x: &mut T) -> &mut [u8] {
    &mut *slice_from_raw_parts_mut(x as *mut _ as *mut u8, size_of::<T>())
}
