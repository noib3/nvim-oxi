use std::fmt;

use super::collection::Collection;
use super::object::Object;
use super::string::String;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L95
pub type Dictionary = Collection<KeyValuePair>;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L128
#[repr(C)]
pub struct KeyValuePair {
    key: String,
    value: Object,
}

impl fmt::Debug for KeyValuePair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("KeyValuePair")
            .field("key", &self.key.to_string_lossy())
            .field("value", &self.value)
            .finish()
    }
}

impl<K: Into<String>, V: Into<Object>> From<(K, V)> for KeyValuePair {
    fn from((key, value): (K, V)) -> Self {
        Self { key: key.into(), value: value.into() }
    }
}

// impl<K, V, E> TryFrom<(K, V)> for KeyValuePair
// where
//     K: Into<String>,
//     V: TryInto<Object, Error = E>,
// {
//     type Error = E;

//     fn try_from((key, value): (K, V)) -> Result<Self, Self::Error> {
//         Ok(Self { key: key.into(), value: value.try_into()? })
//     }
// }

impl KeyValuePair {
    pub fn new<K: Into<String>, V: Into<Object>>(key: K, value: V) -> Self {
        Self { key: key.into(), value: value.into() }
    }

    #[inline]
    pub fn set<V: Into<Object>>(&mut self, v: V) {
        self.value = v.into();
    }
}
