use std::collections::HashMap as StdHashMap;
use std::error::Error;
use std::ffi::c_int;
use std::mem::ManuallyDrop;
use std::{fmt, ptr};

use lua::{ffi::*, LuaPoppable, LuaPushable};
use luajit_bindings as lua;

use super::{Collection, Object, String};

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L95
//
/// A mapping from Neovim [`String`s](String) to [`Object`s](Object).
pub type Dictionary = Collection<KeyValuePair>;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L128
#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct KeyValuePair {
    pub(crate) key: String,
    pub(crate) value: Object,
}

impl fmt::Debug for KeyValuePair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for KeyValuePair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.key, self.value)
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

impl Dictionary {
    pub fn get<Q>(&self, query: &Q) -> Option<&Object>
    where
        String: PartialEq<Q>,
    {
        self.iter()
            .find_map(|pair| (&pair.key == query).then_some(&pair.value))
    }

    pub fn get_mut<Q>(&mut self, query: &Q) -> Option<&mut Object>
    where
        String: PartialEq<Q>,
    {
        self.iter_mut()
            .find_map(|pair| (&pair.key == query).then_some(&mut pair.value))
    }
}

impl fmt::Debug for Dictionary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map()
            .entries(self.iter().map(|pair| (&pair.key, &pair.value)))
            .finish()
    }
}

impl fmt::Display for Dictionary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl<S: Into<String>> std::ops::Index<S> for Dictionary {
    type Output = Object;

    fn index(&self, index: S) -> &Self::Output {
        self.get(&index.into()).unwrap()
    }
}

impl<S: Into<String>> std::ops::IndexMut<S> for Dictionary {
    fn index_mut(&mut self, index: S) -> &mut Self::Output {
        self.get_mut(&index.into()).unwrap()
    }
}

impl LuaPushable for Dictionary {
    unsafe fn push(
        self,
        lstate: *mut lua_State,
    ) -> Result<c_int, Box<dyn Error>> {
        lua::ffi::lua_createtable(lstate, 0, self.len().try_into()?);

        for (key, obj) in self {
            lua::ffi::lua_pushlstring(lstate, key.as_ptr(), key.len());
            obj.push(lstate)?;
            lua::ffi::lua_rawset(lstate, -3);
        }

        Ok(1)
    }
}

impl LuaPoppable for Dictionary {
    const N: c_int = 1;

    unsafe fn pop(lstate: *mut lua_State) -> Result<Self, Box<dyn Error>> {
        if lua_type(lstate, -1) != lua::ffi::LUA_TTABLE
            || lua::utils::is_table_array(lstate, -1)
        {
            // TODO: return early
            todo!()
        }

        let len = lua_objlen(lstate, -1);
        let mut pairs = Vec::<(crate::String, Object)>::with_capacity(len);

        // Pushing `nil` as the first key.
        lua_pushnil(lstate);

        while lua_next(lstate, -2) != 0 {
            if lua_type(lstate, -2) != LUA_TSTRING {
                let typename = lua::utils::debug_type(lstate, -2);

                // TODO: return early
                todo!()

                // return Err(Error::custom(format!(
                //     "encountered a {typename} key while popping a dictionary \
                //      off the stack"
                // )));
            }

            let key = {
                let mut len = 0;
                let ptr = lua_tolstring(lstate, -2, &mut len);

                let mut vec = Vec::<u8>::with_capacity(len);
                std::ptr::copy(ptr as *const u8, vec.as_mut_ptr(), len);
                vec.set_len(len);

                crate::String::from_bytes(vec)
            };

            let value = Object::pop(lstate)?;

            pairs.push((key, value));
        }

        lua_pop(lstate, 1);

        Ok(Dictionary::from_iter(pairs))
    }
}

impl IntoIterator for Dictionary {
    type IntoIter = DictIterator;
    type Item = <DictIterator as Iterator>::Item;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        // Wrap `self` in `ManuallyDrop` to avoid running destructor.
        let arr = ManuallyDrop::new(self);
        let start = arr.items;
        let end = unsafe { start.add(arr.len()) };

        DictIterator { start, end }
    }
}

pub struct DictIterator {
    start: *const KeyValuePair,
    end: *const KeyValuePair,
}

impl Iterator for DictIterator {
    type Item = (String, Object);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            return None;
        }
        let current = self.start;
        self.start = unsafe { self.start.offset(1) };
        let KeyValuePair { key, value } = unsafe { ptr::read(current) };
        Some((key, value))
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

impl ExactSizeIterator for DictIterator {
    #[inline]
    fn len(&self) -> usize {
        unsafe { self.end.offset_from(self.start) as usize }
    }
}

impl DoubleEndedIterator for DictIterator {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            return None;
        }
        let current = self.end;
        self.end = unsafe { self.end.offset(-1) };
        let KeyValuePair { key, value } = unsafe { ptr::read(current) };
        Some((key, value))
    }
}

impl std::iter::FusedIterator for DictIterator {}

impl Drop for DictIterator {
    fn drop(&mut self) {
        while self.start != self.end {
            unsafe {
                ptr::drop_in_place(self.start as *mut Object);
                self.start = self.start.offset(1);
            }
        }
    }
}

impl<K, V> FromIterator<(K, V)> for Dictionary
where
    K: Into<String>,
    V: Into<Object>,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        iter.into_iter()
            .map(|(k, v)| (k, v.into()))
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

#[cfg(test)]
mod tests {
    use super::{Dictionary, Object, String as NvimString};

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

    #[test]
    fn debug_dict() {
        let dict = Dictionary::from_iter([
            ("a", Object::from(1)),
            ("b", Object::from(true)),
            ("c", Object::from("foobar")),
        ]);

        assert_eq!(
            String::from("{a: 1, b: true, c: \"foobar\"}"),
            format!("{dict}")
        );
    }

    #[test]
    fn debug_nested_dict() {
        let dict = Dictionary::from_iter([(
            "foo",
            Object::from(Dictionary::from_iter([("a", 1)])),
        )]);

        assert_eq!(String::from("{foo: {a: 1}}"), format!("{dict}"));
    }
}
