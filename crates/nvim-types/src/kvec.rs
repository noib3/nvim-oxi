//! This module contains functionality common to both `Array`s and
//! `Dictionary`s.

use std::ops::{Deref, DerefMut};
use std::ptr;
use std::slice;

use crate::NonOwning;

// https://github.com/attractivechaos/klib/blob/master/kvec.h#L55
//
/// Binding to Klib's [`kvec`][1].
///
/// [1]: https://github.com/attractivechaos/klib/blob/master/kvec.h
#[repr(C)]
pub struct KVec<T> {
    #[cfg(feature = "neovim-0-7")]
    pub(crate) items: *mut T,
    pub(crate) size: usize,
    pub(crate) capacity: usize,
    #[cfg(not(feature = "neovim-0-7"))]
    pub(crate) items: *mut T,
}

impl<T> Default for KVec<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> KVec<T> {
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

    /// Drops the collection using `libc::free()` instead of calling `Drop`,
    /// returning a new copy that can be dropped as usual.
    ///
    /// This must always be called on the output of a Neovim function that
    /// returns either an `Array` or a `Dictionary` allocated with
    /// `malloc()`.
    #[inline]
    pub unsafe fn drop_with_free(self) -> Self
    where
        T: Clone,
    {
        let new = self.clone();
        libc::free(self.items as *mut libc::c_void);
        core::mem::forget(self);
        new
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

impl<T: Clone> Clone for KVec<T> {
    fn clone(&self) -> Self {
        self.as_slice().to_owned().into()
    }
}

impl<T> Drop for KVec<T> {
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
                self.items, self.size,
            ))
        }
    }
}

impl<T: PartialEq> PartialEq for KVec<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T> Deref for KVec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> DerefMut for KVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T> From<Vec<T>> for KVec<T> {
    #[inline]
    fn from(vec: Vec<T>) -> Self {
        let size = vec.len();
        let capacity = vec.capacity();
        let ptr = vec.leak() as *mut [T] as *mut T;

        unsafe { Self::from_raw_parts(ptr, size, capacity) }
    }
}

impl<T> From<KVec<T>> for Vec<T> {
    #[inline]
    fn from(coll: KVec<T>) -> Self {
        unsafe {
            if coll.items.is_null() {
                Vec::new()
            } else {
                Vec::from_raw_parts(coll.items, coll.size, coll.capacity)
            }
        }
    }
}
