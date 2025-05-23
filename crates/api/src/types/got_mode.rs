use core::ops::Deref;

use types::{
    Dictionary,
    Object,
    String as NvimString,
    conversion,
    serde::Deserializer,
};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, serde::Deserialize)]
pub struct GotMode {
    pub blocking: bool,
    pub mode: ModeStr,
}

/// A newtype around an [`NvimString`] whose contents are guaranteed to match
/// the textual representation of one of the modes listed under `:help mode()`.
#[derive(Clone, Debug, Eq, Hash, serde::Deserialize)]
#[serde(transparent)]
pub struct ModeStr(NvimString);

impl TryFrom<Dictionary> for GotMode {
    type Error = conversion::Error;

    #[inline]
    fn try_from(dict: Dictionary) -> Result<Self, Self::Error> {
        use serde::Deserialize;
        Self::deserialize(Deserializer::new(dict.into())).map_err(Into::into)
    }
}

impl TryFrom<Object> for ModeStr {
    type Error = <NvimString as TryFrom<Object>>::Error;

    #[inline]
    fn try_from(obj: Object) -> Result<Self, Self::Error> {
        NvimString::try_from(obj).map(Self)
    }
}

impl ModeStr {
    // TODO: add more methods for checking mode properties.

    #[inline]
    pub fn is_select_or_visual(&self) -> bool {
        self.is_select() || self.is_visual()
    }

    #[inline]
    pub fn is_select(&self) -> bool {
        self.is_select_blockwise()
            || self.is_select_by_character()
            || self.is_select_by_line()
    }

    #[inline]
    pub fn is_select_blockwise(&self) -> bool {
        self.first_char() == '\u{13}' // CTRL-S
    }

    #[inline]
    pub fn is_select_by_character(&self) -> bool {
        self.first_char() == 's'
    }

    #[inline]
    pub fn is_select_by_line(&self) -> bool {
        self.first_char() == 'S'
    }

    #[inline]
    pub fn is_visual(&self) -> bool {
        self.is_visual_blockwise()
            || self.is_visual_by_character()
            || self.is_visual_by_line()
    }

    #[inline]
    pub fn is_visual_blockwise(&self) -> bool {
        self.first_char() == '\u{16}' // CTRL-V
    }

    #[inline]
    pub fn is_visual_by_character(&self) -> bool {
        self.first_char() == 'v'
    }

    #[inline]
    pub fn is_visual_by_line(&self) -> bool {
        self.first_char() == 'V'
    }

    #[inline]
    fn first_char(&self) -> char {
        self.as_bytes().first().copied().expect("mode is not empty") as char
    }
}

impl<T: PartialEq<NvimString>> PartialEq<T> for ModeStr {
    #[inline]
    fn eq(&self, other: &T) -> bool {
        other == self.deref()
    }
}

impl Deref for ModeStr {
    type Target = NvimString;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
