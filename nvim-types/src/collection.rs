//! This module contains functionality common to both `Array`s and
//! `Dictionary`s.

use std::marker::PhantomData;
use std::ptr::NonNull;

use libc::size_t;

#[repr(C)]
pub struct Collection<T> {
    pub(crate) items: NonNull<T>,
    pub(crate) size: size_t,
    pub(crate) capacity: size_t,
    pub(crate) _marker: PhantomData<T>,
}

unsafe impl<T: Send> Send for Collection<T> {}
unsafe impl<T: Sync> Sync for Collection<T> {}

impl<T> Collection<T> {
    pub const fn new() -> Self {
        Self {
            items: NonNull::dangling(),
            size: 0,
            capacity: 0,
            _marker: PhantomData,
        }
    }
}

impl<T> Default for Collection<T> {
    fn default() -> Self {
        Self::new()
    }
}
