use std::collections::HashMap as StdHashMap;
use std::fmt;

use super::collection::Collection;
use super::object::Object;
use super::string::NvimString;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L95
pub type Dictionary = Collection<KeyValuePair>;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L128
// #[derive(Clone, PartialEq)]
#[repr(C)]
pub struct KeyValuePair {
    key: NvimString,
    value: Object,
}

// impl Dictionary {
//     pub fn get<K, V>(&self, key: &K) -> Option<V>
//     where
//         K: ?Sized + PartialEq<NvimString>,
//         V: TryFrom<Object>,
//     {
//         // TODO: return reference if V is already of type Object, clone
//         // otherwise?
//         self.iter()
//             .find_map(|target| {
//                 (key == &target.key).then(|| target.value.clone())
//             })?
//             .try_into()
//             .ok()
//     }
// }

impl fmt::Debug for Dictionary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO:
        // f.debug_map().entries(self.iter()).finish()
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<K, V> FromIterator<(K, V)> for Dictionary
where
    K: Into<NvimString>,
    V: Into<Object>,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let vec = iter
            .into_iter()
            .filter_map(|(k, v)| {
                let obj = v.into();
                (!obj.is_nil()).then(|| KeyValuePair::from((k, obj)))
            })
            .collect::<Vec<KeyValuePair>>();

        let size = vec.len();
        let capacity = vec.capacity();
        let ptr = vec.leak() as *mut [KeyValuePair] as *mut KeyValuePair;

        unsafe { Self::from_raw_parts(ptr, size, capacity) }
    }
}

impl<K, V> From<StdHashMap<K, V>> for Dictionary
where
    K: Into<NvimString>,
    V: Into<Object>,
{
    fn from(hashmap: StdHashMap<K, V>) -> Self {
        hashmap.into_iter().collect()
    }
}

impl fmt::Debug for KeyValuePair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("KeyValuePair")
            .field("key", &self.key.to_string_lossy())
            .field("value", &self.value)
            .finish()
    }
}

impl<K, V> From<(K, V)> for KeyValuePair
where
    K: Into<NvimString>,
    V: Into<Object>,
{
    fn from((k, v): (K, V)) -> Self {
        Self { key: k.into(), value: v.into() }
    }
}
