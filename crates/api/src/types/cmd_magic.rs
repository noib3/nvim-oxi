use serde::Deserialize;

#[non_exhaustive]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Deserialize)]
pub struct CmdMagic {
    /// If `true` the `|` character is treated as a command separator and the
    /// double quote character (`"`) is treated as the start of a comment.
    pub bar: bool,

    /// Whether the command expands filenames, resulting in characters like
    /// `"%"`, `"#"` and other wildcards to be expanded.
    pub file: bool,
}

impl From<CmdMagic> for types::Dictionary {
    #[inline(always)]
    fn from(magic: CmdMagic) -> Self {
        Self::from_iter([("file", magic.file), ("bar", magic.bar)])
    }
}

impl From<CmdMagic> for types::Object {
    #[inline(always)]
    fn from(magic: CmdMagic) -> Self {
        types::Dictionary::from(magic).into()
    }
}
