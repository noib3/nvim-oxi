use nvim_types::{Array, Object};
use serde::Deserialize;

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
#[serde(untagged)]
// TODO: use `char` instead of `String`
pub enum WindowBorderChar {
    Char(String),
    CharAndHlGroup(String, String),
}

impl<S: AsRef<str>> From<S> for WindowBorderChar {
    fn from(s: S) -> Self {
        Self::Char(s.as_ref().to_owned())
    }
}

// Aaarghh, I want specilizationn!!
//
// impl<S: AsRef<str>, H: AsRef<str>> From<(S, H)> for WindowBorderSide {
//     fn from((s, h): (S, H)) -> Self {
//         Self::CharAndHlGroup(s.as_ref().to_owned(), h.as_ref().to_owned())
//     }
// }

impl From<WindowBorderChar> for Object {
    fn from(side: WindowBorderChar) -> Self {
        use WindowBorderChar::*;
        match side {
            Char(ch) => ch.into(),
            CharAndHlGroup(ch, hl) => Array::from_iter([ch, hl]).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::Deserializer;

    #[test]
    fn deserialize_char_side() {
        let side = "|".into();
        let res = WindowBorderChar::deserialize(Deserializer::new(side));
        assert!(res.is_ok(), "{res:?}");
        assert_eq!(WindowBorderChar::Char("|".into()), res.unwrap());
    }

    #[test]
    fn deserialize_char_and_hl_group_side() {
        let side = Array::from_iter(["|", "Foo"]).into();
        let res = WindowBorderChar::deserialize(Deserializer::new(side));
        assert!(res.is_ok(), "{res:?}");
        assert_eq!(
            WindowBorderChar::CharAndHlGroup("|".into(), "Foo".into()),
            res.unwrap()
        );
    }
}
