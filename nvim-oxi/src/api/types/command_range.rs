use nvim_types::object::Object;

use crate::object::ToObject;

/// See `:h command-range` for details.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum CommandRange {
    // #[object("true")]
    CurrentLine,

    // #[object("\"%\"")]
    WholeFile,

    Count(u32),
}

impl ToObject for CommandRange {
    fn to_obj(self) -> Object {
        use CommandRange::*;
        match self {
            CurrentLine => true.to_obj(),
            WholeFile => "%".to_obj(),
            Count(n) => n.to_obj(),
        }
    }
}
