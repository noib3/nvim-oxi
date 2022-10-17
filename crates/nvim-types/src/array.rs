use std::ffi::c_int;
use std::fmt::{self, Debug, Display};
use std::mem::ManuallyDrop;
use std::ptr;

use luajit_bindings::{self as lua, ffi::lua_State};

use super::{KVec, Object};

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L89
//
/// A vector of Neovim [`Object`](Object)s.
pub type Array = KVec<Object>;

impl Debug for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl lua::Pushable for Array {
    unsafe fn push(self, state: *mut lua_State) -> Result<c_int, lua::Error> {
        <Vec<Object>>::from(self).push(state)
    }
}

impl lua::Poppable for Array {
    unsafe fn pop(state: *mut lua_State) -> Result<Self, lua::Error> {
        <Vec<Object> as lua::Poppable>::pop(state).map(Into::into)
    }
}

impl IntoIterator for Array {
    type IntoIter = ArrayIterator;
    type Item = <ArrayIterator as Iterator>::Item;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        // Wrap `self` in `ManuallyDrop` to avoid running destructor.
        let arr = ManuallyDrop::new(self);
        let start = arr.items;
        let end = unsafe { start.add(arr.len()) };
        ArrayIterator { start, end }
    }
}

impl<T> FromIterator<T> for Array
where
    T: Into<Object>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter()
            .map(Into::into)
            .filter(Object::is_some) // TODO: maybe  avoid this
            .collect::<Vec<Object>>()
            .into()
    }
}

/// An owning iterator over the [`Object`]s of a Neovim [`Array`].
pub struct ArrayIterator {
    start: *const Object,
    end: *const Object,
}

impl Iterator for ArrayIterator {
    type Item = Object;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            return None;
        }
        let current = self.start;
        self.start = unsafe { self.start.offset(1) };
        Some(unsafe { ptr::read(current) })
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

impl ExactSizeIterator for ArrayIterator {
    fn len(&self) -> usize {
        unsafe { self.end.offset_from(self.start) as usize }
    }
}

impl DoubleEndedIterator for ArrayIterator {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            return None;
        }
        let current = self.end;
        self.end = unsafe { self.end.offset(-1) };
        Some(unsafe { ptr::read(current) })
    }
}

impl std::iter::FusedIterator for ArrayIterator {}

impl Drop for ArrayIterator {
    fn drop(&mut self) {
        while self.start != self.end {
            unsafe {
                ptr::drop_in_place(self.start as *mut Object);
                self.start = self.start.offset(1);
            }
        }
    }
}

/// Implements `From<(A, B, C, ..)> for Array` for tuples `(A, B, C, ..)` where
/// all the elements in the tuple implement `Into<Object>`.
macro_rules! from_tuple {
    ($($ty:ident)*) => {
        impl <$($ty: Into<Object>),*> From<($($ty,)*)> for Array {
            #[allow(non_snake_case)]
            fn from(($($ty,)*): ($($ty,)*)) -> Self {
                Self::from_iter([$($ty.into(),)*])
            }
        }
    };
}

from_tuple!(A);
from_tuple!(A B);
from_tuple!(A B C);
from_tuple!(A B C D);
from_tuple!(A B C D E);
from_tuple!(A B C D E F);
from_tuple!(A B C D E F G);
from_tuple!(A B C D E F G H);
from_tuple!(A B C D E F G H I);
from_tuple!(A B C D E F G H I J);
from_tuple!(A B C D E F G H I J K);
from_tuple!(A B C D E F G H I J K L);
from_tuple!(A B C D E F G H I J K L M);
from_tuple!(A B C D E F G H I J K L M N);
from_tuple!(A B C D E F G H I J K L M N O);
from_tuple!(A B C D E F G H I J K L M N O P);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_basic() {
        let array = Array::from_iter(["Foo", "Bar", "Baz"]);

        let mut iter = array.into_iter();
        assert_eq!(Some(Object::from("Foo")), iter.next());
        assert_eq!(Some(Object::from("Bar")), iter.next());
        assert_eq!(Some(Object::from("Baz")), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn drop_iter_halfway() {
        let array = Array::from_iter(["Foo", "Bar", "Baz"]);

        let mut iter = array.into_iter();
        assert_eq!(Some(Object::from("Foo")), iter.next());
    }

    #[test]
    fn empty_array() {
        let empty = Array { size: 0, capacity: 0, items: ptr::null_mut() };
        assert_eq!(0, empty.into_iter().count());
    }

    #[test]
    fn debug_array() {
        let arr = Array::from((1, 2, 3, "a", true));
        assert_eq!(String::from("[1, 2, 3, \"a\", true]"), format!("{arr}"));
    }

    #[test]
    fn debug_nested_array() {
        let arr = Array::from_iter([Array::from((1, 2, 3))]);
        assert_eq!(String::from("[[1, 2, 3]]"), format!("{arr}"));
    }
}
