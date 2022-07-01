use std::fmt;

use nvim_types::{Array, Integer, Object};
use serde::de;

#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum CmdRange {
    None,
    Single(usize),
    Double(usize, usize),
}

impl<'de> de::Deserialize<'de> for CmdRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct CmdRangeVisitor;

        impl<'de> de::Visitor<'de> for CmdRangeVisitor {
            type Value = CmdRange;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("an array with 0, 1 or 2 numbers")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                match seq.size_hint().ok_or_else(|| {
                    de::Error::custom(&"couldn't determine array length")
                })? {
                    0 => Ok(Self::Value::None),
                    1 => {
                        let a = seq.next_element::<usize>()?.unwrap();
                        Ok(Self::Value::Single(a))
                    },
                    2 => {
                        let a = seq.next_element::<usize>()?.unwrap();
                        let b = seq.next_element::<usize>()?.unwrap();
                        Ok(Self::Value::Double(a, b))
                    },
                    num => Err(de::Error::invalid_length(
                        num,
                        &"an array with 0, 1 or 2 numbers",
                    )),
                }
            }
        }

        deserializer.deserialize_seq(CmdRangeVisitor)
    }
}

impl From<CmdRange> for Object {
    fn from(rng: CmdRange) -> Self {
        use CmdRange::*;

        match rng {
            None => Array::new(),
            Single(a) => Array::from_iter([a as Integer]),
            Double(a, b) => Array::from_iter([a as Integer, b as Integer]),
        }
        .into()
    }
}
