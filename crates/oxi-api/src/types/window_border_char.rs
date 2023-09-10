use std::fmt;

use oxi_types::{self as nvim, Array, Object};
use serde::{de, Deserialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum WindowBorderChar {
    Char(Option<char>),
    CharAndHlGroup(Option<char>, String),
}

impl From<char> for WindowBorderChar {
    fn from(ch: char) -> Self {
        Self::Char(Some(ch))
    }
}

impl From<Option<char>> for WindowBorderChar {
    fn from(maybe: Option<char>) -> Self {
        Self::Char(maybe)
    }
}

impl<S: AsRef<str>> From<(char, S)> for WindowBorderChar {
    fn from((ch, hl): (char, S)) -> Self {
        Self::CharAndHlGroup(Some(ch), hl.as_ref().to_owned())
    }
}

impl From<WindowBorderChar> for Object {
    fn from(side: WindowBorderChar) -> Self {
        use WindowBorderChar::*;
        match side {
            Char(None) => nvim::String::new().into(),
            Char(Some(ch)) => ch.into(),
            CharAndHlGroup(None, hl) => Array::from(("", hl)).into(),
            CharAndHlGroup(Some(ch), hl) => Array::from((ch, hl)).into(),
        }
    }
}

impl<'de> Deserialize<'de> for WindowBorderChar {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct WindowBorderCharVisitor;

        impl<'de> de::Visitor<'de> for WindowBorderCharVisitor {
            type Value = WindowBorderChar;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a string or an array of two strings")
            }

            fn visit_str<E>(self, str: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Self::Value::Char(str_to_char(str)))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                match seq.size_hint() {
                    Some(2) => {},

                    Some(other) => {
                        return Err(de::Error::invalid_length(
                            other,
                            &"border character in array form expected to \
                              contain 2 elements representing character and \
                              highlight group respectively",
                        ))
                    },

                    None => {
                        return Err(de::Error::custom(
                            "couldn't determine array length",
                        ))
                    },
                }

                let ch = seq.next_element::<String>()?.unwrap();
                let hl = seq.next_element::<String>()?.unwrap();

                Ok(Self::Value::CharAndHlGroup(str_to_char(&ch), hl))
            }
        }

        deserializer.deserialize_any(WindowBorderCharVisitor)
    }
}

// NOTE: `str` is assumed to be at most 4 bytes long.
fn str_to_char(str: &str) -> Option<char> {
    match str.len() {
        0 => None,
        1 => Some(str.as_bytes()[0] as char),
        // This handles multibyte characters.
        2..=4 => {
            char::decode_utf16(str.encode_utf16()).next().and_then(Result::ok)
        },
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use oxi_types::serde::Deserializer;

    use super::*;

    #[test]
    fn to_char() {
        assert_eq!(None, str_to_char(""));
        assert_eq!(Some('a'), str_to_char("a"));
        assert_eq!(Some('£'), str_to_char("£")); // 2 bytes
        assert_eq!(Some('ʃ'), str_to_char("ʃ")); // 2 bytes
        assert_eq!(Some('ユ'), str_to_char("ユ")); // 3 bytes
        assert_eq!(Some('𒀀'), str_to_char("𒀀")); // 4 bytes
        assert_eq!(Some('😀'), str_to_char("😀")); // 4 bytes
    }

    #[test]
    fn deserialize_char_side() {
        let side = "|".into();
        let res = WindowBorderChar::deserialize(Deserializer::new(side));
        assert!(res.is_ok(), "{res:?}");
        assert_eq!(WindowBorderChar::Char(Some('|')), res.unwrap());
    }

    #[test]
    fn deserialize_char_and_hl_group_side() {
        let side = Array::from_iter(["|", "Foo"]).into();
        let res = WindowBorderChar::deserialize(Deserializer::new(side));
        assert!(res.is_ok(), "{res:?}");
        assert_eq!(
            WindowBorderChar::CharAndHlGroup(Some('|'), "Foo".into()),
            res.unwrap()
        );
    }
}
