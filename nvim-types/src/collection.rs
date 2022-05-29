//! This module contains functionality common to both `Array`s and
//! `Dictionary`s.

use std::marker::PhantomData;
use std::mem::{self, ManuallyDrop};
use std::ops::{Deref, Index};
use std::ptr::{self, NonNull};
use std::slice::{self, SliceIndex};

use libc::size_t;

#[repr(C)]
pub struct Collection<T> {
    pub(crate) items: NonNull<T>,
    pub(crate) size: size_t,
    pub(crate) capacity: size_t,
    pub(crate) _marker: PhantomData<T>,
}

// unsafe impl<T: Send> Send for Collection<T> {}
// unsafe impl<T: Sync> Sync for Collection<T> {}

impl<T> Collection<T> {
    /// Creates a new empty `Collection`. If you already know how many elements
    /// the collection will have consider using `Collection::with_capacity`
    /// instead.
    pub const fn new() -> Self {
        Self {
            items: NonNull::dangling(),
            size: 0,
            capacity: 0,
            _marker: PhantomData,
        }
    }

    // /// Creates a new empty `Collection` with a preallocated capacity.
    // pub const fn with_capacity(capacity: usize) -> Self {
    //     todo!()
    // }

    // /// The number of items in the collection.
    // pub const fn len(&self) -> usize {
    //     self.size
    // }

    #[inline]
    pub(crate) fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.items.as_ptr(), self.size) }
    }

    #[inline]
    pub(crate) unsafe fn from_raw_parts(
        ptr: *mut T,
        size: usize,
        capacity: usize,
    ) -> Self {
        Self {
            items: NonNull::new_unchecked(ptr),
            size,
            capacity,
            _marker: PhantomData,
        }
    }

    pub fn from_vec<V: Into<Vec<T>>>(v: V) -> Self {
        // let mut collect = MaybeUninit::<Self>::uninit();
        // let ptr = collect.as_mut_ptr();

        // let mut vec = v.into();
        // let items = vec.as_mut_ptr();

        // unsafe {
        //     addr_of_mut!((*ptr).size).write(vec.len());
        //     addr_of_mut!((*ptr).capacity).write(vec.capacity());
        //     addr_of_mut!((*ptr).items).write(NonNull::new_unchecked(items));
        // }

        // mem::forget(vec);
        // unsafe { collect.assume_init() };

        // Why couldn't this work?

        let mut vec = v.into();
        // let (ptr, size, capacity) = vec.into_raw_parts();

        let new = Self {
            items: unsafe { NonNull::new_unchecked(vec.as_mut_ptr()) },
            size: vec.len(),
            capacity: vec.capacity(),
            _marker: PhantomData,
        };

        mem::forget(vec);

        new
    }
}

// impl<T> Default for Collection<T> {
//     fn default() -> Self {
//         Self::new()
//     }
// }

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

// impl<T: Clone> Clone for Collection<T> {
//     fn clone(&self) -> Self {
//         Self::from_vec(self.as_slice().to_owned())
//         // todo!()
//     }
// }

// impl<T: PartialEq> PartialEq<Self> for Collection<T> {
//     #[inline]
//     fn eq(&self, other: &Self) -> bool {
//         self.as_slice() == other.as_slice()
//     }
// }

impl<T> IntoIterator for Collection<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let capacity = self.capacity;

        let mut me = ManuallyDrop::new(self);

        let start = me.items.as_ptr();
        let end = unsafe { start.add(me.len()) };

        IntoIter {
            buf: unsafe { NonNull::new_unchecked(start) },
            capacity,
            start,
            end,
            _marker: PhantomData,
        }
    }
}

// impl<T: fmt::Debug> fmt::Debug for Collection<T> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         f.debug_list().entries(self.iter()).finish()
//     }
// }

/// An iterator that moves out of a `Collection`.
pub struct IntoIter<T> {
    buf: NonNull<T>,
    start: *const T,
    end: *const T,
    capacity: usize,
    _marker: PhantomData<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        (self.start != self.end).then(|| {
            let old = self.start;
            self.start = unsafe { self.start.offset(1) };
            unsafe { ptr::read(old) }
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = self.len();
        (exact, Some(exact))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        unsafe { self.end.offset_from(self.start) as usize }
    }
}
