use std::collections::HashMap as StdHashMap;
use std::mem::ManuallyDrop;
use std::{fmt, ptr};

use super::collection::Collection;
use super::object::Object;
use super::string::String;
use crate::non_owning::NonOwning;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L95
pub type Dictionary = Collection<KeyValuePair>;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L128
#[derive(Clone)]
#[repr(C)]
pub struct KeyValuePair {
    pub key: String,
    pub value: Object,
}

impl fmt::Debug for KeyValuePair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("KeyValuePair")
            .field(&self.key)
            .field(&self.value)
            .finish()
    }
}

impl<K, V> From<(K, V)> for KeyValuePair
where
    K: Into<String>,
    V: Into<Object>,
{
    fn from((k, v): (K, V)) -> Self {
        Self { key: k.into(), value: v.into() }
    }
}

impl fmt::Debug for Dictionary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map()
            .entries(self.iter().map(|pair| (&pair.key, &pair.value)))
            .finish()
    }
}

impl Dictionary {
    /// Make a non-owning version of this `Dictionary`.
    #[inline]
    pub fn non_owning(&self) -> NonOwning<'_, Self> {
        NonOwning::new(Self { ..*self })
    }
}

impl IntoIterator for Dictionary {
    type IntoIter = DictIter;
    type Item = <DictIter as Iterator>::Item;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let arr = ManuallyDrop::new(self);
        let start = arr.items.as_ptr();
        let end = unsafe { start.add(arr.len()) };

        DictIter { start, end }
    }
}

pub struct DictIter {
    start: *const KeyValuePair,
    end: *const KeyValuePair,
}

impl Iterator for DictIter {
    type Item = (String, Object);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        (self.start != self.end).then(|| {
            let old = self.start;
            self.start = unsafe { self.start.offset(1) };
            let KeyValuePair { key, value } = unsafe { ptr::read(old) };
            // TODO: read copies, there's a leak here!
            (key, value)
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

impl ExactSizeIterator for DictIter {
    fn len(&self) -> usize {
        unsafe { self.end.offset_from(self.start) as usize }
    }
}

impl<K, V> FromIterator<(K, V)> for Dictionary
where
    String: From<K>,
    Object: From<V>,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        iter.into_iter()
            .map(|(k, v)| (k, Object::from(v)))
            .filter(|(_, obj)| obj.is_some())
            .map(KeyValuePair::from)
            .collect::<Vec<KeyValuePair>>()
            .into()
    }
}

impl<K, V> From<StdHashMap<K, V>> for Dictionary
where
    String: From<K>,
    Object: From<V>,
{
    fn from(hashmap: StdHashMap<K, V>) -> Self {
        hashmap.into_iter().collect()
    }
}
