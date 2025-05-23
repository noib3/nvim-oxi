use core::ops::{Deref, DerefMut};

use luajit as lua;

use crate::NonOwning;
use crate::Object;
use crate::kvec::{self, KVec};

/// A vector of Neovim [`Object`]s.
#[derive(Clone, Default, PartialEq)]
#[repr(transparent)]
pub struct Array(pub(super) KVec<Object>);

/// An owning iterator over the `Object`s of a [`Array`].
#[derive(Clone)]
pub struct ArrayIterator(kvec::IntoIter<Object>);

/// The error type returned when trying to convert an [`Array`] into a tuple.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum ArrayFromTupleError<T> {
    /// Not enough elements in the array.
    #[error(
        "not enough elements in the array, expected {expected_len} but got \
         {actual_len}"
    )]
    NotEnoughElements { expected_len: usize, actual_len: usize },
    /// The tuple element at the given index couldn't be converted into the
    /// requested type.
    #[error(
        "couldn't convert tuple element at index {element_idx} into object: \
         {error:?}"
    )]
    ElementFromObject { element_idx: usize, error: T },
}

impl core::fmt::Debug for Array {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl Array {
    /// Returns the number of elements in the array.
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the array contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns an iterator over the `Object`s of the array.
    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, Object> {
        self.0.as_slice().iter()
    }

    /// Creates a new, empty `Array`.
    #[inline]
    pub fn new() -> Self {
        Self(KVec::new())
    }

    /// Returns a non-owning version of this `Array`.
    #[inline]
    pub fn non_owning(&self) -> NonOwning<'_, Self> {
        #[allow(clippy::unnecessary_struct_initialization)]
        NonOwning::new(Self(KVec { ..self.0 }))
    }

    /// Appends an element to the back of the array.
    #[inline]
    pub fn push<V>(&mut self, value: V)
    where
        V: Into<Object>,
    {
        let value = value.into();
        if !value.is_nil() {
            self.0.push(value);
        }
    }

    /// Removes an `Object` from the `Array` and returns it.
    ///
    /// The removed object is replaced by the last element of the array.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    #[track_caller]
    #[inline]
    pub fn swap_remove(&mut self, index: usize) -> Object {
        self.0.swap_remove(index)
    }
}

impl Deref for Array {
    type Target = [Object];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl DerefMut for Array {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_slice()
    }
}

impl<T: Into<Object>> Extend<T> for Array {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for value in iter {
            self.push(value);
        }
    }
}

impl<T: Into<Object>> FromIterator<T> for Array {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut array = Self::new();
        array.extend(iter);
        array
    }
}

impl IntoIterator for Array {
    type Item = Object;
    type IntoIter = ArrayIterator;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        ArrayIterator(self.0.into_iter())
    }
}

impl Iterator for ArrayIterator {
    type Item = Object;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl ExactSizeIterator for ArrayIterator {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl DoubleEndedIterator for ArrayIterator {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl core::iter::FusedIterator for ArrayIterator {}

impl lua::Poppable for Array {
    #[inline]
    unsafe fn pop(lstate: *mut lua::ffi::State) -> Result<Self, lua::Error> {
        use lua::ffi::*;

        if lua_gettop(lstate) == 0 {
            return Err(lua::Error::PopEmptyStack);
        } else if lua_type(lstate, -1) != LUA_TTABLE {
            let ty = lua_type(lstate, -1);
            return Err(lua::Error::pop_wrong_type::<Self>(LUA_TTABLE, ty));
        }

        // TODO: check that the table is an array-like table and not a
        // dictionary-like one?

        let mut kvec = KVec::with_capacity(lua_objlen(lstate, -1));

        lua_pushnil(lstate);

        while lua_next(lstate, -2) != 0 {
            kvec.push(Object::pop(lstate)?);
        }

        // Pop the table.
        lua_pop(lstate, 1);

        Ok(Self(kvec))
    }
}

impl lua::Pushable for Array {
    #[inline]
    unsafe fn push(
        self,
        lstate: *mut lua::ffi::State,
    ) -> Result<core::ffi::c_int, lua::Error> {
        use lua::ffi::*;

        lua_createtable(lstate, self.len() as _, 0);

        for (idx, obj) in
            self.into_iter().filter(|obj| !obj.is_nil()).enumerate()
        {
            obj.push(lstate)?;
            lua_rawseti(lstate, -2, (idx + 1) as _);
        }

        Ok(1)
    }
}

/// Implements `From<(A, B, C, ..)>` for tuples `(A, B, C, ..)` where all the
/// elements in the tuple are `Into<Object>`.
macro_rules! array_from_tuple {
    ($($ty:ident)*) => {
        impl <$($ty: Into<Object>),*> From<($($ty,)*)> for Array {
            #[allow(non_snake_case)]
            fn from(($($ty,)*): ($($ty,)*)) -> Self {
                Self::from_iter([$($ty.into(),)*])
            }
        }
    };
}

array_from_tuple!(A);
array_from_tuple!(A B);
array_from_tuple!(A B C);
array_from_tuple!(A B C D);
array_from_tuple!(A B C D E);
array_from_tuple!(A B C D E F);
array_from_tuple!(A B C D E F G);
array_from_tuple!(A B C D E F G H);
array_from_tuple!(A B C D E F G H I);
array_from_tuple!(A B C D E F G H I J);
array_from_tuple!(A B C D E F G H I J K);
array_from_tuple!(A B C D E F G H I J K L);
array_from_tuple!(A B C D E F G H I J K L M);
array_from_tuple!(A B C D E F G H I J K L M N);
array_from_tuple!(A B C D E F G H I J K L M N O);
array_from_tuple!(A B C D E F G H I J K L M N O P);

macro_rules! count {
    () => {0i32};
    ($x:tt $($xs:tt)*) => {1i32 + count!($($xs)*)};
}

/// Implements `TryFrom<Array>` for tuples `(A, B, C, ..)` where all the
/// elements in the tuple are `TryFrom<Object>` with the same error.
macro_rules! tuple_try_from_array {
    ($($ty:ident)*) => {
        impl<Error, $($ty,)*> TryFrom<Array> for ($($ty,)*)
        where
            $($ty: TryFrom<Object, Error = Error>,)*
        {
            type Error = ArrayFromTupleError<Error>;

            #[inline]
            #[allow(non_snake_case)]
            fn try_from(array: Array) -> Result<Self, Self::Error> {
                let actual_len = array.len();
                let expected_len = count!($($ty)*) as usize;

                if actual_len < expected_len {
                    return Err(ArrayFromTupleError::NotEnoughElements {
                        expected_len,
                        actual_len
                    });
                }

                let mut iter = array.into_iter();

                $(
                    let $ty = $ty::try_from(
                        iter.next().expect("already checked len")
                    ).map_err(|error| ArrayFromTupleError::ElementFromObject {
                        element_idx: actual_len - iter.len() - 1,
                        error
                    })?;
                )*

                Ok(($($ty,)*))
            }
        }
    };
}

tuple_try_from_array!(A);
tuple_try_from_array!(A B);
tuple_try_from_array!(A B C);
tuple_try_from_array!(A B C D);
tuple_try_from_array!(A B C D E);
tuple_try_from_array!(A B C D E F);
tuple_try_from_array!(A B C D E F G);
tuple_try_from_array!(A B C D E F G H);
tuple_try_from_array!(A B C D E F G H I);
tuple_try_from_array!(A B C D E F G H I J);
tuple_try_from_array!(A B C D E F G H I J K);
tuple_try_from_array!(A B C D E F G H I J K L);
tuple_try_from_array!(A B C D E F G H I J K L M);
tuple_try_from_array!(A B C D E F G H I J K L M N);
tuple_try_from_array!(A B C D E F G H I J K L M N O);
tuple_try_from_array!(A B C D E F G H I J K L M N O P);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_layout() {
        use core::alloc::Layout;

        assert_eq!(Layout::new::<Array>(), Layout::new::<KVec<Object>>());
    }

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
        let empty = Array::default();
        assert_eq!(0, empty.into_iter().count());
    }
}
