use nvim_types::{Float, Integer};
use serde::Deserialize;
use serde_repr::Deserialize_repr;

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
// https://github.com/neovim/neovim/blob/master/src/nvim/viml/parser/expressions.h#L65
pub enum ExprComparisonType {
    Equal,
    Greater,
    GreaterOrEqual,
    Identical,
    Matches,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Hash, Deserialize)]
// https://github.com/neovim/neovim/blob/master/src/nvim/viml/parser/expressions.h#L20
pub enum ExprCaseCompareStrategy {
    UseOption,
    MatchCase,
    IgnoreCase,
}

#[derive(
    Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Hash, Deserialize_repr,
)]
// https://github.com/neovim/neovim/blob/master/src/nvim/viml/parser/expressions.h#L72
#[repr(u8)]
pub enum ExprOptScope {
    Unspecified = 0,
    Global = 'g' as u8,
    Local = 'l' as u8,
}

#[derive(
    Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Hash, Deserialize_repr,
)]
// https://github.com/neovim/neovim/blob/master/src/nvim/viml/parser/expressions.h#L96
#[repr(u8)]
pub enum ExprVarScope {
    Missing = 0,
    Script = 's' as u8,
    Global = 'g' as u8,
    Vim = 'v' as u8,
    Buffer = 'b' as u8,
    Window = 'w' as u8,
    Tabpage = 't' as u8,
    Local = 'l' as u8,
    Arguments = 'a' as u8,
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
