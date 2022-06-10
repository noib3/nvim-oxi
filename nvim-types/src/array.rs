use std::mem::ManuallyDrop;
use std::{fmt, ptr};

use crate::NonOwning;

use super::collection::Collection;
use super::object::Object;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L95
pub type Array = Collection<Object>;

impl fmt::Debug for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl IntoIterator for Array {
    type IntoIter = ArrayIter;
    type Item = <ArrayIter as Iterator>::Item;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let arr = ManuallyDrop::new(self);
        let start = arr.items.as_ptr();
        let end = unsafe { start.add(arr.len()) };

        ArrayIter { start, end }
    }
}

pub struct ArrayIter {
    start: *const Object,
    end: *const Object,
}

impl Iterator for ArrayIter {
    type Item = Object;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
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

impl ExactSizeIterator for ArrayIter {
    fn len(&self) -> usize {
        unsafe { self.end.offset_from(self.start) as usize }
    }
}

impl<T> FromIterator<T> for Array
where
    Object: From<T>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter()
            .map(Object::from)
            .filter(Object::is_some)
            .collect::<Vec<Object>>()
            .into()
    }
}

impl Array {
    /// Make a non-owning version of this Array.
    #[inline]
    pub fn non_owning<'a>(&'a self) -> NonOwning<'a, Self> {
        // The Dictionary is owned by self, and will not be droped before 'a ends
        unsafe {
            NonOwning::new(Self {
                items: self.items,
                size: self.size,
                capacity: self.capacity,
            })
        }
    }
}
