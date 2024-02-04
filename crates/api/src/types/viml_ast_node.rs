use serde::Deserialize;
use serde_repr::Deserialize_repr;
use types::{Float, Integer};

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize)]
pub enum VimLAstNode {
    And,
    Arrow,
    Assignment(/* augmentation: */ AssignmentAugmentation),
    BinaryMinus,
    BinaryPlus,
    Call,
    Colon,
    Comma,
    Comparison {
        cmp_type: ExprComparisonType,
        ccs_strategy: ExprCaseCompareStrategy,
        invert: bool,
    },
    ComplexIdentifier,
    Concat,
    ConcatOrSubscript,
    CurlyBracesIdentifier,
    DictLiteral,
    Division,

    DoubleQuotedString(/* svalue: */ String),
    Environment {
        ident: String,
    },
    Float(/* fvalue: */ Float),
    Integer(/* ivalue: */ Integer),
    Lambda,
    ListLiteral,
    Missing,
    Mod,
    Multiplication,
    Nested,
    Not,
    OpMissing,
    Option {
        scope: ExprOptScope,
        ident: String,
    },
    Or,
    PlainIdentifier {
        scope: ExprVarScope,
        ident: String,
    },
    PlainKey {
        ident: String,
    },
    Register {
        name: i32,
    },
    SingleQuotedString(/* svalue: */ String),
    Subscript,
    Ternary,
    TernaryValue,
    UnaryMinus,
    UnaryPlus,
    UnknownFigure,
}

impl Eq for VimLAstNode {}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Hash, Deserialize)]
pub enum AssignmentAugmentation {
    /// `=`
    #[serde(rename = "")]
    None,

    /// `+=`
    #[serde(rename = "+=")]
    Add,

    /// `+=`
    #[serde(rename = "-=")]
    Subtract,

    /// `+=`
    #[serde(rename = ".=")]
    Concat,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Hash, Deserialize)]
// https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/viml/parser/expressions.h#L68
pub enum ExprComparisonType {
    Equal,
    Greater,
    GreaterOrEqual,
    Identical,
    Matches,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Hash, Deserialize)]
// https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/viml/parser/expressions.h#L23
pub enum ExprCaseCompareStrategy {
    UseOption,
    MatchCase,
    IgnoreCase,
}

#[derive(
    Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Hash, Deserialize_repr,
)]
// https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/viml/parser/expressions.h#L78
#[repr(u8)]
pub enum ExprOptScope {
    Unspecified = 0,
    Global = b'g',
    Local = b'l',
}

#[derive(
    Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Hash, Deserialize_repr,
)]
// https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/viml/parser/expressions.h#L99
#[repr(u8)]
pub enum ExprVarScope {
    Missing = 0,
    Script = b's',
    Global = b'g',
    Vim = b'v',
    Buffer = b'b',
    Window = b'w',
    Tabpage = b't',
    Local = b'l',
    Arguments = b'a',
}

/// Only used for deserialization purposes.
#[derive(Copy, Clone, Deserialize)]
pub(crate) enum DeserializedVimLASTNode {
    And,
    Arrow,
    Assignment,
    BinaryMinus,
    BinaryPlus,
    Call,
    Colon,
    Comma,
    Comparison,
    ComplexIdentifier,
    Concat,
    ConcatOrSubscript,
    CurlyBracesIdentifier,
    DictLiteral,
    Division,
    DoubleQuotedString,
    Environment,
    Float,
    Integer,
    Lambda,
    ListLiteral,
    Missing,
    Mod,
    Multiplication,
    Nested,
    Not,
    OpMissing,
    Option,
    Or,
    PlainIdentifier,
    PlainKey,
    Register,
    SingleQuotedString,
    Subscript,
    Ternary,
    TernaryValue,
    UnaryMinus,
    UnaryPlus,
    UnknownFigure,
}
