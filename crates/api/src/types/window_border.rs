use std::fmt;

use serde::de;
use types::{Array, Object};

use super::WindowBorderChar;

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum WindowBorder {
    /// No border.
    None,

    /// A single line box.
    Single,

    /// A double line box.
    Double,

    /// Like `Single`, but with rounded corners (`╭`, etc).
    Rounded,

    /// Adds a single whitespace cell of padding.
    Solid,

    /// A drop shadow effect by blending with the background.
    Shadow,

    /// A single character used for the whole border:
    ///
    /// aaaaa
    /// a   a
    /// a   a
    /// aaaaa
    Uniform(WindowBorderChar),

    /// A tuple `(a, b)` where `a` is used for the border's corners and `b` for
    /// its edges:
    ///
    /// abbba
    /// b   b
    /// b   b
    /// abbba
    CornersEdges(WindowBorderChar, WindowBorderChar),

    /// A tuple `(a, b, c, d)` where `a` and `c` are used for the border's
    /// corners, `b` for its horizontal edges and `d` for the vertical ones:
    ///
    /// abbbc
    /// d   d
    /// d   d
    /// cbbba
    CornersHorizontalVertical(
        WindowBorderChar,
        WindowBorderChar,
        WindowBorderChar,
        WindowBorderChar,
    ),

    /// Use this if you're a control freak and want to specify every single
    /// corner and edge character individually.
    /// The tuple `(a, b, c, d, e, f, g, h)` specifies every corner and edge
    /// character clockwise:
    ///
    /// abbbc
    /// h   d
    /// h   d
    /// gfffe
    Anal(
        WindowBorderChar,
        WindowBorderChar,
        WindowBorderChar,
        WindowBorderChar,
        WindowBorderChar,
        WindowBorderChar,
        WindowBorderChar,
        WindowBorderChar,
    ),
}

macro_rules! impl_from_tuple {
    ($variant:ident, $($ty:ident)*) => {
        impl <$($ty: Into<WindowBorderChar>),*> From<($($ty,)*)> for WindowBorder {
            #[allow(non_snake_case)]
            fn from(($($ty,)*): ($($ty,)*)) -> Self {
                Self::$variant($($ty.into(),)*)
            }
        }
    };
}

impl_from_tuple!(Uniform, A);
impl_from_tuple!(CornersEdges, A B);
impl_from_tuple!(CornersHorizontalVertical, A B C D);
impl_from_tuple!(Anal, A B C D E F G H);

impl From<WindowBorder> for Object {
    fn from(border: WindowBorder) -> Self {
        use WindowBorder::*;
        match border {
            None => "none".into(),
            Single => "single".into(),
            Double => "double".into(),
            Rounded => "rounded".into(),
            Solid => "solid".into(),
            Shadow => "shadow".into(),

            Uniform(a) => Array::from_iter([a]).into(),

            CornersEdges(a, b) => Array::from_iter([a, b]).into(),

            CornersHorizontalVertical(a, b, c, d) => {
                Array::from_iter([a, b, c, d]).into()
            },

            Anal(a, b, c, d, e, f, g, h) => {
                Array::from_iter([a, b, c, d, e, f, g, h]).into()
            },
        }
    }
}

impl<'de> de::Deserialize<'de> for WindowBorder {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct WindowBorderVisitor;

        impl<'de> de::Visitor<'de> for WindowBorderVisitor {
            type Value = WindowBorder;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str(
                    "\"none\", \"single\", \"double\", \"rounded\", \
                     \"solid\", \"shadow\" or an array with n elements where \
                     n is a factor of 8",
                )
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match s {
                    "none" => Ok(Self::Value::None),
                    "single" => Ok(Self::Value::Single),
                    "double" => Ok(Self::Value::Double),
                    "rounded" => Ok(Self::Value::Rounded),
                    "solid" => Ok(Self::Value::Solid),
                    "shadow" => Ok(Self::Value::Shadow),

                    _ => Err(E::invalid_value(
                        de::Unexpected::Str(s),
                        &"\"none\", \"single\", \"double\", \"rounded\", \
                          \"solid\", \"shadow\" or an array with n elements \
                          where n is a factor of 8",
                    )),
                }
            }

            // If the returned value is a sequence we always return the
            // `WindowBorderSide::Anal` variant since Neovim always returns all
            // the 8 characters specifying the border, even if it was set from
            // a variant with fewer chars.
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                match seq.size_hint() {
                    Some(8) => {},

                    Some(other) => {
                        return Err(de::Error::invalid_length(
                            other,
                            &"border styles returned by Neovim always \
                              contain 8 items",
                        ));
                    },

                    None => {
                        return Err(de::Error::custom(
                            "couldn't determine array length",
                        ));
                    },
                };

                let a = seq.next_element::<WindowBorderChar>()?.unwrap();
                let b = seq.next_element::<WindowBorderChar>()?.unwrap();
                let c = seq.next_element::<WindowBorderChar>()?.unwrap();
                let d = seq.next_element::<WindowBorderChar>()?.unwrap();
                let e = seq.next_element::<WindowBorderChar>()?.unwrap();
                let f = seq.next_element::<WindowBorderChar>()?.unwrap();
                let g = seq.next_element::<WindowBorderChar>()?.unwrap();
                let h = seq.next_element::<WindowBorderChar>()?.unwrap();

                Ok(Self::Value::Anal(a, b, c, d, e, f, g, h))
            }
        }

        deserializer.deserialize_any(WindowBorderVisitor)
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use types::serde::Deserializer;

    use super::*;

    #[test]
    fn deserialize_none() {
        let side = "none".into();
        let res = WindowBorder::deserialize(Deserializer::new(side));
        assert!(res.is_ok(), "{res:?}");
        assert_eq!(WindowBorder::None, res.unwrap());
    }

    #[test]
    fn deserialize_single() {
        let side = "single".into();
        let res = WindowBorder::deserialize(Deserializer::new(side));
        assert!(res.is_ok(), "{res:?}");
        assert_eq!(WindowBorder::Single, res.unwrap());
    }

    #[test]
    fn deserialize_double() {
        let side = "double".into();
        let res = WindowBorder::deserialize(Deserializer::new(side));
        assert!(res.is_ok(), "{res:?}");
        assert_eq!(WindowBorder::Double, res.unwrap());
    }

    #[test]
    fn deserialize_rounded() {
        let side = "rounded".into();
        let res = WindowBorder::deserialize(Deserializer::new(side));
        assert!(res.is_ok(), "{res:?}");
        assert_eq!(WindowBorder::Rounded, res.unwrap());
    }

    #[test]
    fn deserialize_solid() {
        let side = "solid".into();
        let res = WindowBorder::deserialize(Deserializer::new(side));
        assert!(res.is_ok(), "{res:?}");
        assert_eq!(WindowBorder::Solid, res.unwrap());
    }

    #[test]
    fn deserialize_shadow() {
        let side = "shadow".into();
        let res = WindowBorder::deserialize(Deserializer::new(side));
        assert!(res.is_ok(), "{res:?}");
        assert_eq!(WindowBorder::Shadow, res.unwrap());
    }

    #[test]
    fn deserialize_anal() {
        let sides = Array::from_iter(["a"; 8]).into();
        let res = WindowBorder::deserialize(Deserializer::new(sides));
        assert!(res.is_ok(), "{res:?}");
        assert_eq!(
            WindowBorder::from(('a', 'a', 'a', 'a', 'a', 'a', 'a', 'a')),
            res.unwrap()
        );

        let sides =
            Array::from_iter(["", "", "", ">", "", "", "", "<"]).into();
        let res = WindowBorder::deserialize(Deserializer::new(sides));
        assert!(res.is_ok(), "{res:?}");

        assert_eq!(
            WindowBorder::from(
                (None, None, None, '>', None, None, None, '<',)
            ),
            res.unwrap()
        );
    }

    #[test]
    fn deserialize_anal_with_hl_groups() {
        let sides = Array::from_iter([
            Object::from(Array::from_iter(["a", "Foo"])),
            "b".into(),
            Object::from(Array::from_iter(["", "Bar"])),
            "c".into(),
            Object::from(Array::from_iter(["d", "Baz"])),
            "".into(),
            Object::from(Array::from_iter(["", "FooBar"])),
            "e".into(),
        ])
        .into();
        let res = WindowBorder::deserialize(Deserializer::new(sides));
        assert!(res.is_ok(), "{res:?}");
        assert_eq!(
            WindowBorder::Anal(
                WindowBorderChar::CharAndHlGroup(Some('a'), "Foo".into()),
                WindowBorderChar::Char(Some('b')),
                WindowBorderChar::CharAndHlGroup(None, "Bar".into()),
                WindowBorderChar::Char(Some('c')),
                WindowBorderChar::CharAndHlGroup(Some('d'), "Baz".into()),
                WindowBorderChar::Char(None),
                WindowBorderChar::CharAndHlGroup(None, "FooBar".into()),
                WindowBorderChar::Char(Some('e')),
            ),
            res.unwrap()
        );
    }
}
