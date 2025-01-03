//! This module contains the binding to Klib's [`kvec`] type.

use core::mem;
use core::ops::{Deref, DerefMut};
use core::ptr;
use core::slice;

/// Binding to Klib's [`kvec`].
///
/// Neovim uses this for its [`Array`] and [`Dictionary`] types.
///
/// [`kvec`]: https://github.com/attractivechaos/klib/blob/master/kvec.h#L55
/// [`Array`]: https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/private/defs.h#L89
/// [`Dictionary`]: https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/private/defs.h#L92
#[repr(C)]
pub(crate) struct KVec<T> {
    pub(super) size: usize,
    pub(super) capacity: usize,
    pub(super) items: *mut T,
}

impl<T> Default for KVec<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for KVec<T> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T> KVec<T> {
    /// Returns a mutable slice of the vector's contents.
    #[inline]
    pub(crate) fn as_mut_slice(&mut self) -> &mut [T] {
        if self.items.is_null() {
            &mut []
        } else {
            assert!(self.len() * mem::size_of::<T>() <= isize::MAX as usize);
            unsafe { slice::from_raw_parts_mut(self.items, self.size) }
        }
    }

    /// Returns a slice of the vector's contents.
    #[inline]
    pub(crate) fn as_slice(&self) -> &[T] {
        if self.items.is_null() {
            &[]
        } else {
            assert!(self.len() * mem::size_of::<T>() <= isize::MAX as usize);
            unsafe { slice::from_raw_parts(self.items, self.size) }
        }
    }

    /// Returns the number of elements the vector can hold without reallocating.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns the number of elements in the vector.
    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.size
    }

    /// Returns `true` if the vector contains no elements.
    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Creates a new, empty `KVec<T>`.
    #[inline]
    pub(crate) fn new() -> Self {
        Self { items: core::ptr::null_mut(), size: 0, capacity: 0 }
    }

    /// Appends an element to the back of a collection.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX`.
    #[inline]
    pub(crate) fn push(&mut self, item: T) {
        if self.capacity == 0 {
            self.capacity = 4;
            self.items = unsafe {
                libc::malloc(self.capacity * mem::size_of::<T>()) as *mut T
            };
        } else if self.size == self.capacity {
            self.capacity *= 2;

            assert!(
                self.capacity * mem::size_of::<T>() <= isize::MAX as usize
            );

            self.items = unsafe {
                libc::realloc(
                    self.items as *mut libc::c_void,
                    self.capacity * mem::size_of::<T>(),
                ) as *mut T
            };
        }

        unsafe {
            ptr::write(self.items.add(self.size), item);
        }

        self.size += 1;
    }

    /// Removes an element from the `KVec` and returns it.
    ///
    /// The removed element is replaced by the last element of the vector.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    #[track_caller]
    #[inline]
    pub(crate) fn swap_remove(&mut self, index: usize) -> T {
        let len = self.len();
        if index >= len {
            panic!("swap_remove index is {index}, but len is {len}");
        }
        unsafe {
            let item = ptr::read(self.items.add(index));
            ptr::copy(self.items.add(len - 1), self.items.add(index), 1);
            self.size -= 1;
            item
        }
    }

    /// Creates a new, empty `KVec<T>` with the specified capacity.
    #[inline]
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        let items =
            unsafe { libc::malloc(capacity * mem::size_of::<T>()) as *mut T };

        Self { items, size: 0, capacity }
    }
}

impl<T: Clone> Clone for KVec<T> {
    #[inline]
    fn clone(&self) -> Self {
        let items = unsafe {
            libc::malloc(self.capacity * mem::size_of::<T>()) as *mut T
        };

        for idx in 0..self.size {
            unsafe {
                let item = &*self.items.add(idx);
                ptr::write(items.add(idx), item.clone());
            }
        }

        Self { items, size: self.size, capacity: self.capacity }
    }
}

impl<T: PartialEq> PartialEq for KVec<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T: PartialEq> PartialEq<[T]> for KVec<T> {
    #[inline]
    fn eq(&self, other: &[T]) -> bool {
        self.as_slice() == other
    }
}

impl<T: PartialEq> PartialEq<&[T]> for KVec<T> {
    #[inline]
    fn eq(&self, other: &&[T]) -> bool {
        self.as_slice() == *other
    }
}

impl<const N: usize, T: PartialEq> PartialEq<[T; N]> for KVec<T> {
    #[inline]
    fn eq(&self, other: &[T; N]) -> bool {
        self == other.as_slice()
    }
}

impl<const N: usize, T: PartialEq> PartialEq<&[T; N]> for KVec<T> {
    #[inline]
    fn eq(&self, other: &&[T; N]) -> bool {
        self == *other
    }
}

impl<T> Deref for KVec<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> DerefMut for KVec<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T> FromIterator<T> for KVec<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let (lo, hi) = iter.size_hint();
        let mut kvec = Self::with_capacity(hi.unwrap_or(lo));
        for item in iter {
            kvec.push(item);
        }
        kvec
    }
}

impl<T> IntoIterator for KVec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(self) -> IntoIter<T> {
        let ptr = self.items;
        let end = unsafe { ptr.add(self.size) };
        mem::forget(self);
        IntoIter { ptr, start: ptr, end }
    }
}

impl<T> Drop for KVec<T> {
    #[inline]
    fn drop(&mut self) {
        if !self.items.is_null() {
            // Drop each element before freeing the vector itself.
            for idx in 0..self.size {
                unsafe {
                    ptr::drop_in_place(self.items.add(idx));
                }
            }

            unsafe { libc::free(self.items as *mut libc::c_void) };
        }
    }
}

pub(crate) struct IntoIter<T> {
    /// Points to the start of the `KVec<T>`, used in the `Drop` impl.
    ptr: *mut T,

    /// The current position of the forward iterator.
    start: *mut T,

    /// One past the current position of the backward iterator.
    end: *mut T,
}

impl<T: Clone> Clone for IntoIter<T> {
    #[inline]
    fn clone(&self) -> Self {
        let len = unsafe { self.end.offset_from(self.start) as usize };
        let ptr = unsafe { libc::malloc(len * mem::size_of::<T>()) as *mut T };
        for idx in 0..len {
            unsafe {
                let item = &*self.start.add(idx);
                ptr::write(ptr.add(idx), item.clone());
            }
        }
        IntoIter { ptr, start: ptr, end: unsafe { ptr.add(len) } }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            let current = self.start;
            self.start = unsafe { self.start.offset(1) };
            Some(unsafe { ptr::read(current) })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = self.len();
        (exact, Some(exact))
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        unsafe { self.end.offset_from(self.start) as usize }
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            self.end = unsafe { self.end.offset(-1) };
            Some(unsafe { ptr::read(self.end) })
        }
    }
}

impl<T> core::iter::FusedIterator for IntoIter<T> {}

impl<T> Drop for IntoIter<T> {
    #[inline]
    fn drop(&mut self) {
        // Drop each element before freeing the original `KVec`.
        while self.start != self.end {
            let current = self.start;
            self.start = unsafe { self.start.offset(1) };
            unsafe { ptr::drop_in_place(current) };
        }

        unsafe { libc::free(self.ptr as *mut libc::c_void) };
    }
}

impl<T> From<Vec<T>> for KVec<T> {
    #[inline]
    fn from(vec: Vec<T>) -> Self {
        vec.into_iter().collect()
    }
}

impl<T> From<KVec<T>> for Vec<T> {
    #[inline]
    fn from(kvec: KVec<T>) -> Self {
        kvec.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kvec_as_slice() {
        let mut kvec = KVec::new();
        kvec.push("foo");
        kvec.push("bar");
        kvec.push("baz");
        assert_eq!(kvec.as_slice(), &["foo", "bar", "baz"]);
    }

    #[test]
    fn kvec_is_empty() {
        let mut kvec = KVec::<&str>::new();
        assert!(kvec.is_empty());

        kvec.push("foo");
        assert!(!kvec.is_empty());
    }

    #[test]
    fn kvec_with_capacity() {
        let kvec = KVec::<i32>::new();
        assert_eq!(kvec.capacity(), 0);

        let mut kvec = KVec::<i32>::with_capacity(2);
        assert_eq!(kvec.capacity(), 2);

        kvec.push(1);
        kvec.push(2);
        kvec.push(3);
        assert_eq!(kvec.capacity(), 4);
    }

    #[test]
    fn kvec_drop() {
        let mut kvec = KVec::new();
        kvec.push(String::from("foo"));
        kvec.push(String::from("bar"));
        kvec.push(String::from("baz"));
        assert_eq!(kvec.len(), 3);
        drop(kvec);
    }

    #[test]
    fn kvec_clone() {
        let mut kvec = KVec::new();
        kvec.push(String::from("foo"));
        kvec.push(String::from("bar"));
        kvec.push(String::from("baz"));

        let kvec2 = kvec.clone();

        assert_eq!(kvec, kvec2);

        drop(kvec);
        drop(kvec2);
    }

    #[test]
    fn kvec_from_iter() {
        let kvec = vec!["foo", "bar", "baz"].into_iter().collect::<KVec<_>>();
        assert_eq!(kvec.as_slice(), &["foo", "bar", "baz"]);
    }

    #[test]
    fn kvec_into_iter() {
        let kvec: KVec<String> =
            ["foo", "bar", "baz"].into_iter().map(Into::into).collect();

        let mut iter = kvec.into_iter();

        assert_eq!(Some("foo"), iter.next().as_deref());
        assert_eq!(Some("bar"), iter.next().as_deref());
        assert_eq!(Some("baz"), iter.next().as_deref());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn kvec_iter_backward() {
        let kvec: KVec<String> =
            ["foo", "bar", "baz"].into_iter().map(Into::into).collect();

        let mut iter = kvec.into_iter();

        assert_eq!(Some("baz"), iter.next_back().as_deref());
        assert_eq!(Some("bar"), iter.next_back().as_deref());
        assert_eq!(Some("foo"), iter.next_back().as_deref());
        assert_eq!(None, iter.next_back());
    }

    #[test]
    fn kvec_iter_drop_halfway() {
        let kvec: KVec<String> =
            ["foo", "bar", "baz"].into_iter().map(Into::into).collect();

        let mut iter = kvec.into_iter();
        assert_eq!(Some("foo"), iter.next().as_deref());
        assert_eq!(Some("bar"), iter.next().as_deref());
        drop(iter);
    }

    #[test]
    fn kvec_iter_clone() {
        let kvec: KVec<String> =
            ["foo", "bar", "baz"].into_iter().map(Into::into).collect();

        let mut iter = kvec.into_iter();

        assert_eq!(Some("foo"), iter.next().as_deref());

        let mut iter2 = iter.clone();

        assert_eq!(Some("bar"), iter.next().as_deref());
        assert_eq!(Some("baz"), iter.next().as_deref());
        assert_eq!(None, iter.next());

        assert_eq!(Some("bar"), iter2.next().as_deref());
        assert_eq!(Some("baz"), iter2.next().as_deref());
        assert_eq!(None, iter2.next());
    }

    #[test]
    fn swap_remove() {
        let mut kvec = KVec::from_iter([1, 2, 3, 4]);
        assert_eq!(kvec.swap_remove(1), 2);
        assert_eq!(kvec, &[1, 4, 3]);
    }

    #[should_panic]
    #[test]
    fn swap_remove_oob() {
        let mut kvec = KVec::from_iter([1, 2, 3, 4]);
        kvec.swap_remove(kvec.len());
    }
}
