use core::slice;
use std::{
    mem::size_of,
    ops::Range,
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

#[inline]
pub fn slice_as_bytes<T: Sized>(x: &[T]) -> &[u8] {
    let Range { start, end } = x.as_ptr_range();
    let start = start as *const u8;
    let end = end as *const u8;
    unsafe { slice::from_ptr_range(start..end) }
}

#[inline]
pub unsafe fn bytes_to_value<T: Sized>(bytes: Box<[u8]>) -> Box<T> {
    Box::from_raw(Box::into_raw(bytes) as *mut _)
}

#[inline]
pub unsafe fn bytes_to_slice<T: Sized>(mut bytes: Box<[u8]>) -> Box<[T]> {
    let Range { start, end } = bytes.as_mut_ptr_range();
    let start = start as *mut _;
    let end = end as *mut _;
    Box::leak(bytes);
    Box::from_raw(slice::from_mut_ptr_range(Range { start, end }))
}
