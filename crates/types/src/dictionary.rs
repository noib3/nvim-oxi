use luajit as lua;

use crate::kvec::{self, KVec};
use crate::NonOwning;
use crate::Object;

/// A vector of Neovim
/// `(`[`String`](crate::String)`, `[`Object`](crate::Object)`)` pairs.
#[derive(Clone, Default, PartialEq)]
#[repr(transparent)]
pub struct Dictionary(pub(super) KVec<KeyValuePair>);

/// A key-value pair mapping a [`String`] to an [`Object`].
//
// https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/private/defs.h#L122-L125
#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct KeyValuePair {
    key: crate::String,
    value: Object,
}

impl core::fmt::Debug for Dictionary {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{{ ")?;

        let num_elements = self.len();

        for (idx, (key, value)) in self.iter().enumerate() {
            write!(f, "{}: {:?}", key, value)?;

            if idx + 1 < num_elements {
                write!(f, ", ")?;
            }
        }

        write!(f, " }}")?;

        Ok(())
    }
}

impl Dictionary {
    /// Returns a slice of all key-value pairs in the dictionary.
    #[inline]
    pub fn as_slice(&self) -> &[KeyValuePair] {
        &self.0
    }

    /// Returns a mutable slice of all key-value pairs in the dictionary.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [KeyValuePair] {
        &mut self.0
    }

    /// Returns a reference to the value corresponding to the key.
    #[inline]
    pub fn get<Q>(&self, query: &Q) -> Option<&Object>
    where
        Q: ?Sized + PartialEq<crate::String>,
    {
        self.get_index(query).map(|idx| self.as_slice()[idx].value())
    }

    /// Returns the index of the key-value pair corresponding to the key.
    #[inline]
    pub fn get_index<Q>(&self, query: &Q) -> Option<usize>
    where
        Q: ?Sized + PartialEq<crate::String>,
    {
        self.keys()
            .enumerate()
            .find_map(|(idx, key)| (query == key).then_some(idx))
    }

    /// Returns a mutable reference to the value corresponding to the key.
    #[inline]
    pub fn get_mut<Q>(&mut self, query: &Q) -> Option<&mut Object>
    where
        Q: ?Sized + PartialEq<crate::String>,
    {
        self.get_index(query).map(|idx| self.as_mut_slice()[idx].value_mut())
    }

    /// Inserts a key-value pair into the dictionary.
    #[inline]
    pub fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: Into<crate::String>,
        V: Into<Object>,
    {
        let pair = KeyValuePair { key: key.into(), value: value.into() };
        self.0.push(pair);
    }

    /// Returns `true` if the dictionary contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns an iterator over the `(String, Object)` pairs of the
    /// dictionary.
    #[inline]
    pub fn iter(&self) -> DictIter<'_> {
        DictIter(self.0.iter())
    }

    /// Returns a mutable iterator over the `(String, Object)` pairs of the
    /// dictionary.
    #[inline]
    pub fn iter_mut(&mut self) -> DictIterMut<'_> {
        DictIterMut(self.0.iter_mut())
    }

    /// Returns the number of elements in the dictionary.
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns an iterator over the keys of the dictionary.
    #[inline]
    pub fn keys(&self) -> impl Iterator<Item = &crate::String> + '_ {
        self.iter().map(|(key, _)| key)
    }

    /// Creates a new, empty `Dictionary`.
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
}

impl KeyValuePair {
    /// Consumes the `KeyValuePair` and returns the key.
    #[inline]
    pub fn into_key(self) -> crate::String {
        self.key
    }

    /// Consumes the `KeyValuePair` and returns a `(key, value)` tuple.
    #[inline]
    pub fn into_tuple(self) -> (crate::String, Object) {
        (self.key, self.value)
    }

    /// Consumes the `KeyValuePair` and returns the value.
    #[inline]
    pub fn into_value(self) -> Object {
        self.value
    }

    /// Returns a shared reference to the key of the `KeyValuePair`.
    #[inline]
    pub fn key(&self) -> &crate::String {
        &self.key
    }

    /// Returns an exclusive reference to the key of the `KeyValuePair`.
    #[inline]
    pub fn key_mut(&mut self) -> &mut crate::String {
        &mut self.key
    }

    /// Returns references to both the key and value as a tuple.
    #[inline]
    pub fn tuple(&self) -> (&crate::String, &Object) {
        (&self.key, &self.value)
    }

    /// Returns exclusive references to both the key and value as a tuple.
    #[inline]
    pub fn tuple_mut(&mut self) -> (&mut crate::String, &mut Object) {
        (&mut self.key, &mut self.value)
    }

    /// Returns a shared reference to the value of the `KeyValuePair`.
    #[inline]
    pub fn value(&self) -> &Object {
        &self.value
    }

    /// Returns an exclusive reference to the value of the `KeyValuePair`.
    #[inline]
    pub fn value_mut(&mut self) -> &mut Object {
        &mut self.value
    }
}

impl<S> core::ops::Index<S> for Dictionary
where
    S: PartialEq<crate::String>,
{
    type Output = Object;

    #[inline]
    fn index(&self, index: S) -> &Self::Output {
        self.get(&index).unwrap()
    }
}

impl<S> core::ops::IndexMut<S> for Dictionary
where
    S: PartialEq<crate::String>,
{
    #[inline]
    fn index_mut(&mut self, index: S) -> &mut Self::Output {
        self.get_mut(&index).unwrap()
    }
}

impl<K, V> FromIterator<(K, V)> for Dictionary
where
    K: Into<crate::String>,
    V: Into<Object>,
{
    #[inline]
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        Self(
            iter.into_iter()
                .filter_map(|(k, v)| {
                    let value = v.into();
                    value
                        .is_some()
                        .then(|| KeyValuePair { key: k.into(), value })
                })
                .collect(),
        )
    }
}

impl IntoIterator for Dictionary {
    type Item = (crate::String, Object);
    type IntoIter = DictIterator;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        DictIterator(self.0.into_iter())
    }
}

/// An owning iterator over the `(String, Object)` pairs of a [`Dictionary`].
#[derive(Clone)]
pub struct DictIterator(kvec::IntoIter<KeyValuePair>);

impl Iterator for DictIterator {
    type Item = (crate::String, Object);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(KeyValuePair::into_tuple)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl ExactSizeIterator for DictIterator {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl DoubleEndedIterator for DictIterator {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(KeyValuePair::into_tuple)
    }
}

impl core::iter::FusedIterator for DictIterator {}

/// An iterator over the `(String, Object)` pairs of a [`Dictionary`].
#[derive(Clone)]
pub struct DictIter<'a>(core::slice::Iter<'a, KeyValuePair>);

impl<'a> Iterator for DictIter<'a> {
    type Item = (&'a crate::String, &'a Object);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(KeyValuePair::tuple)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl ExactSizeIterator for DictIter<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl DoubleEndedIterator for DictIter<'_> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(KeyValuePair::tuple)
    }
}

impl core::iter::FusedIterator for DictIter<'_> {}

/// A mutable iterator over the `(String, Object)` pairs of a [`Dictionary`].
pub struct DictIterMut<'a>(core::slice::IterMut<'a, KeyValuePair>);

impl<'a> Iterator for DictIterMut<'a> {
    type Item = (&'a mut crate::String, &'a mut Object);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(KeyValuePair::tuple_mut)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl ExactSizeIterator for DictIterMut<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl DoubleEndedIterator for DictIterMut<'_> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(KeyValuePair::tuple_mut)
    }
}

impl core::iter::FusedIterator for DictIterMut<'_> {}

impl lua::Poppable for Dictionary {
    #[inline]
    unsafe fn pop(lstate: *mut lua::ffi::State) -> Result<Self, lua::Error> {
        use lua::ffi::*;

        if lua_gettop(lstate) == 0 {
            return Err(lua::Error::PopEmptyStack);
        } else if lua_type(lstate, -1) != LUA_TTABLE {
            let ty = lua_type(lstate, -1);
            return Err(lua::Error::pop_wrong_type::<Self>(LUA_TTABLE, ty));
        }

        let mut kvec = KVec::with_capacity(lua_objlen(lstate, -1));

        lua_pushnil(lstate);

        while lua_next(lstate, -2) != 0 {
            let value = Object::pop(lstate)?;

            // The following `String::pop()` will pop the key, so we push
            // another copy on the stack for the next iteration.
            lua_pushvalue(lstate, -1);

            let key = crate::String::pop(lstate)?;

            kvec.push(KeyValuePair { key, value });
        }

        // Pop the table.
        lua_pop(lstate, 1);

        Ok(Self(kvec))
    }
}

impl lua::Pushable for Dictionary {
    #[inline]
    unsafe fn push(
        self,
        lstate: *mut lua::ffi::State,
    ) -> Result<core::ffi::c_int, lua::Error> {
        use lua::ffi::*;

        lua_createtable(lstate, 0, self.len() as _);

        for (key, obj) in self {
            lua_pushlstring(lstate, key.as_ptr(), key.len());
            obj.push(lstate)?;
            lua_rawset(lstate, -3);
        }

        Ok(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Object, String as NvimString};

    #[test]
    fn dict_layout() {
        use core::alloc::Layout;

        assert_eq!(
            Layout::new::<Dictionary>(),
            Layout::new::<KVec<KeyValuePair>>()
        );
    }

    #[test]
    fn iter_basic() {
        let dict = Dictionary::from_iter([
            ("foo", "Foo"),
            ("bar", "Bar"),
            ("baz", "Baz"),
        ]);

        let mut iter = dict.into_iter();
        assert_eq!(
            Some((NvimString::from("foo"), Object::from("Foo"))),
            iter.next()
        );
        assert_eq!(
            Some((NvimString::from("bar"), Object::from("Bar"))),
            iter.next()
        );
        assert_eq!(
            Some((NvimString::from("baz"), Object::from("Baz"))),
            iter.next()
        );
        assert_eq!(None, iter.next());
    }

    #[test]
    fn drop_iter_halfway() {
        let dict = Dictionary::from_iter([
            ("foo", "Foo"),
            ("bar", "Bar"),
            ("baz", "Baz"),
        ]);

        let mut iter = dict.into_iter();
        assert_eq!(
            Some((NvimString::from("foo"), Object::from("Foo"))),
            iter.next()
        );
    }
}
