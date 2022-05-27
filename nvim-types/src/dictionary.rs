use std::fmt;

use super::collection::Collection;
use super::object::Object;
use super::string::NvimString;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L95
pub type Dictionary = Collection<KeyValuePair>;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L128
#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct KeyValuePair {
    key: NvimString,
    value: Object,
}

impl Dictionary {
    pub fn get<K, V>(&self, key: &K) -> Option<V>
    where
        K: ?Sized + PartialEq<NvimString>,
        V: TryFrom<Object>,
    {
        // TODO: return reference if V is already of type Object, clone
        // otherwise?
        self.iter()
            .find_map(|target| {
                (key == &target.key).then(|| target.value.clone())
            })?
            .try_into()
            .ok()
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

impl<K: Into<NvimString>, V: Into<Object>> From<(K, V)> for KeyValuePair {
    fn from((key, value): (K, V)) -> Self {
        Self { key: key.into(), value: value.into() }
    }
}

impl KeyValuePair {
    pub fn new<K: Into<NvimString>, V: Into<Object>>(
        key: K,
        value: V,
    ) -> Self {
        Self { key: key.into(), value: value.into() }
    }

    #[inline]
    pub fn set<V: Into<Object>>(&mut self, v: V) {
        self.value = v.into();
    }
}

impl<K, V> FromIterator<(K, V)> for Dictionary
where
    K: Into<NvimString>,
    V: Into<Object>,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let a = iter
            .into_iter()
            .map(|(k, v)| KeyValuePair::new(k, v))
            .collect::<Vec<KeyValuePair>>();

        Self::from_vec(a)
    }
}
