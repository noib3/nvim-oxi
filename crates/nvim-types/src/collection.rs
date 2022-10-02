//! This module contains functionality common to both `Array`s and
//! `Dictionary`s.

use std::ops::{Deref, DerefMut};
use std::ptr;
use std::slice;

use crate::NonOwning;

// Up until 0.7.* `items` was the first item of the struct. From 0.8 it's the
// last one.
#[repr(C)]
pub struct Collection<T> {
    #[cfg(feature = "neovim-0-7")]
    pub(crate) items: *mut T,
    pub(crate) size: usize,
    pub(crate) capacity: usize,
    #[cfg(not(feature = "neovim-0-7"))]
    pub(crate) items: *mut T,
}

impl<T> Default for Collection<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Collection<T> {
    /// Creates a new empty `Collection`.
    #[inline]
    pub const fn new() -> Self {
        Self { items: std::ptr::null_mut(), size: 0, capacity: 0 }
    }

    /// The number of items in the collection.
    #[inline]
    pub const fn len(&self) -> usize {
        self.size
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub(crate) fn as_slice(&self) -> &[T] {
        if self.items.is_null() {
            &[]
        } else {
            unsafe { slice::from_raw_parts(self.items, self.size) }
        }
    }

    #[inline]
    pub(crate) fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.items, self.size) }
    }

    #[inline]
    pub(crate) unsafe fn from_raw_parts(
        ptr: *mut T,
        size: usize,
        capacity: usize,
    ) -> Self {
        Self { items: ptr, size, capacity }
    }

    /// Make a non-owning version of this `Collection`.
    #[inline]
    #[doc(hidden)]
    pub fn non_owning(&self) -> NonOwning<'_, Self> {
        NonOwning::new(Self { ..*self })
    }
}

impl<T: Clone> Clone for Collection<T> {
    fn clone(&self) -> Self {
        self.as_slice().to_owned().into()
    }
}

impl<T> Drop for Collection<T> {
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
                self.items, self.size,
            ))
        }
    }
}

impl<T: PartialEq> PartialEq for Collection<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T> Deref for Collection<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> DerefMut for Collection<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T> From<Vec<T>> for Collection<T> {
    #[inline]
    fn from(vec: Vec<T>) -> Self {
        let size = vec.len();
        let capacity = vec.capacity();
        let ptr = vec.leak() as *mut [T] as *mut T;

        unsafe { Self::from_raw_parts(ptr, size, capacity) }
    }
}

impl<T> From<Collection<T>> for Vec<T> {
    #[inline]
    fn from(coll: Collection<T>) -> Self {
        unsafe {
            if coll.items.is_null() {
                Vec::new()
            } else {
                Vec::from_raw_parts(coll.items, coll.size, coll.capacity)
            }
        }
    }
}
