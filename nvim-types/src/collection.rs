//! This module contains functionality common to both `Array`s and
//! `Dictionary`s.

use std::ops::{Deref, Index};
use std::slice::{self, SliceIndex};

use libc::size_t;

#[repr(C)]
pub struct Collection<T> {
    pub(crate) items: *mut T,
    pub(crate) size: size_t,
    pub(crate) capacity: size_t,
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
    pub(crate) unsafe fn from_raw_parts(
        ptr: *mut T,
        size: usize,
        capacity: usize,
    ) -> Self {
        Self { items: ptr, size, capacity }
    }
}

impl<T: Clone> Clone for Collection<T> {
    fn clone(&self) -> Self {
        self.as_slice().to_owned().into()
    }
}

impl<T> Deref for Collection<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<I, T> Index<I> for Collection<T>
where
    I: SliceIndex<[T]>,
{
    type Output = <I as SliceIndex<[T]>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.deref().index(index)
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
                Vec::from_raw_parts(
                    coll.items,
                    coll.size,
                    coll.capacity,
                )
            }
        }
    }
}
