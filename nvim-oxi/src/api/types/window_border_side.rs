use nvim_types::{Array, Object};
use serde::Deserialize;

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
#[serde(untagged)]
pub enum WindowBorderSide {
    Char(String),
    CharAndHlGroup(String, String),
}

impl<S: AsRef<str>> From<S> for WindowBorderSide {
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

impl From<WindowBorderSide> for Object {
    fn from(side: WindowBorderSide) -> Self {
        use WindowBorderSide::*;
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
        let res = WindowBorderSide::deserialize(Deserializer::new(side));
        assert!(res.is_ok(), "{res:?}");
        assert_eq!(WindowBorderSide::Char("|".into()), res.unwrap());
    }

    #[test]
    fn deserialize_char_and_hl_group_side() {
        let side = Array::from_iter(["|", "Foo"]).into();
        let res = WindowBorderSide::deserialize(Deserializer::new(side));
        assert!(res.is_ok(), "{res:?}");
        assert_eq!(
            WindowBorderSide::CharAndHlGroup("|".into(), "Foo".into()),
            res.unwrap()
        );
    }
}
