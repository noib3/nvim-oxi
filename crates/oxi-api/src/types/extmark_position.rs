use oxi_types::{Array, Integer, Object};
use serde::Deserialize;

#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub enum ExtmarkPosition {
    /// Defines the extmark's position in the buffer by a 0-indexed `(row,
    /// col)` tuple.
    ByTuple((usize, usize)),

    /// Defines the extmark's position in the buffer by its id.
    ById(u32),
}

impl From<ExtmarkPosition> for Object {
    fn from(pos: ExtmarkPosition) -> Self {
        use ExtmarkPosition::*;

        match pos {
            ByTuple((row, col)) => {
                Array::from_iter([row as Integer, col as Integer]).into()
            },
            ById(extmark_id) => extmark_id.into(),
        }
    }
}
