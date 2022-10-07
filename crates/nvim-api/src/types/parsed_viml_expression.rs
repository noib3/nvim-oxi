use std::cmp::Ordering;
use std::collections::BTreeSet;

use nvim_types::{
    conversion::{self, FromObject},
    serde::Deserializer,
    Float,
    Integer,
    Object,
};
use serde::Deserialize;

use super::viml_ast_node::*;

#[non_exhaustive]
#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize)]
/// Informations about a parsed VimL expression returned by
/// [`api::parse_expression`](crate::api::parse_expression).
pub struct ParsedVimLExpression {
    /// The syntax tree of the parsed expression.
    #[serde(default)]
    pub ast: Option<VimLExpressionAst>,

    /// Only present if there was an error parsing the expression.
    #[serde(default)]
    pub error: Option<ParseExpressionError>,

    /// Only present if `include_highlight` was set to `true` when calling
    /// [`api::parse_expression`](crate::api::parse_expression). The first
    /// three items of the tuples represent the line, starting column and
    /// ending column (exclusive) of each highlight, while the last item is the
    /// highlight group name.
    #[serde(default)]
    pub highlight: Vec<(usize, usize, usize, String)>,

    /// Number of bytes successfully parsed.
    pub len: usize,
}

#[non_exhaustive]
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Deserialize)]
pub struct ParseExpressionError {
    /// Error message argument.
    pub arg: String,

    /// Error message in printf format. Contains exactly one `"%.*s"` block.
    pub message: String,
}

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(from = "DeserializedVimLExpressionAST")]
pub struct VimLExpressionAst {
    /// A tree of child nodes.
    #[serde(default)]
    pub children: BTreeSet<VimLExpressionAst>,

    /// Length of the node.
    pub len: usize,

    /// A `(line, column)` tuple describing where the the node is started.
    pub start: (usize, usize),

    /// Error message in printf format. Contains exactly one `"%.*s"` block.
    pub ty: VimLAstNode,
}

// fn deserialize_children<'de, D>(
//     deserializer: D,
// ) -> Result<BTreeSet<VimLExpressionAst>, D::Error>
// where
//     D: serde::de::Deserializer<'de>,
// {
//     todo!()
// }

/// Only used for deserialization purposes.
#[derive(Deserialize)]
#[allow(dead_code)]
struct DeserializedVimLExpressionAST {
    #[serde(default)]
    augmentation: Option<AssignmentAugmentation>,

    #[serde(default)]
    ccs_strategy: Option<ExprCaseCompareStrategy>,

    #[serde(default)]
    children: BTreeSet<VimLExpressionAst>,

    #[serde(default)]
    cmp_type: Option<ExprComparisonType>,

    #[serde(default)]
    fvalue: Option<Float>,

    #[serde(default)]
    ident: Option<String>,

    #[serde(default)]
    invert: Option<bool>,

    #[serde(default)]
    ivalue: Option<Integer>,

    len: usize,

    #[serde(default)]
    name: Option<i32>,

    #[serde(default)]
    scope: Option<ExprScope>,

    start: (usize, usize),

    #[serde(default)]
    svalue: Option<String>,

    #[serde(rename = "type")]
    ty: DeserializedVimLASTNode,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ExprScope {
    Opt(ExprOptScope),
    Var(ExprVarScope),
}

impl From<DeserializedVimLExpressionAST> for VimLExpressionAst {
    fn from(ast: DeserializedVimLExpressionAST) -> Self {
        use DeserializedVimLASTNode::*;
        let ty = match ast.ty {
            And => VimLAstNode::And,
            Arrow => VimLAstNode::Arrow,
            Assignment => VimLAstNode::Assignment(ast.augmentation.unwrap()),
            BinaryMinus => VimLAstNode::BinaryMinus,
            BinaryPlus => VimLAstNode::BinaryPlus,
            Call => VimLAstNode::Call,
            Colon => VimLAstNode::Colon,
            Comma => VimLAstNode::Comma,
            Comparison => VimLAstNode::Comparison {
                cmp_type: ast.cmp_type.unwrap(),
                ccs_strategy: ast.ccs_strategy.unwrap(),
                invert: ast.invert.unwrap(),
            },
            ComplexIdentifier => VimLAstNode::ComplexIdentifier,
            Concat => VimLAstNode::Concat,
            ConcatOrSubscript => VimLAstNode::ConcatOrSubscript,
            CurlyBracesIdentifier => VimLAstNode::CurlyBracesIdentifier,
            DictLiteral => VimLAstNode::DictLiteral,
            Division => VimLAstNode::Division,
            DoubleQuotedString => {
                VimLAstNode::DoubleQuotedString(ast.svalue.unwrap())
            },
            Environment => {
                VimLAstNode::Environment { ident: ast.ident.unwrap() }
            },
            Float => VimLAstNode::Float(ast.fvalue.unwrap()),
            Integer => VimLAstNode::Integer(ast.ivalue.unwrap()),
            Lambda => VimLAstNode::Lambda,
            ListLiteral => VimLAstNode::ListLiteral,
            Missing => VimLAstNode::Missing,
            Mod => VimLAstNode::Mod,
            Multiplication => VimLAstNode::Multiplication,
            Nested => VimLAstNode::Nested,
            Not => VimLAstNode::Not,
            OpMissing => VimLAstNode::OpMissing,
            Option => {
                let ident = ast.ident.unwrap();

                // The `scope` integer may be deserialized into both the `Opt`
                // and `Var` variants bc they overlap. To account for this we
                // first extract the u8 and then turn that back into a
                // `ExprOptScope` by transmuting. Same for `PlainIdentifier`.
                let scope = match ast.scope.unwrap() {
                    ExprScope::Opt(scope) => scope as u8,
                    ExprScope::Var(scope) => scope as u8,
                };
                // SAFETY: read above.
                let scope = unsafe { std::mem::transmute(scope) };

                VimLAstNode::Option { scope, ident }
            },
            Or => VimLAstNode::Or,
            PlainIdentifier => {
                let ident = ast.ident.unwrap();

                let scope = match ast.scope.unwrap() {
                    ExprScope::Opt(scope) => scope as u8,
                    ExprScope::Var(scope) => scope as u8,
                };
                // SAFETY: read above.
                let scope = unsafe { std::mem::transmute(scope) };

                VimLAstNode::PlainIdentifier { scope, ident }
            },
            PlainKey => VimLAstNode::PlainKey { ident: ast.ident.unwrap() },
            Register => VimLAstNode::Register { name: ast.name.unwrap() },
            SingleQuotedString => {
                VimLAstNode::SingleQuotedString(ast.svalue.unwrap())
            },
            Subscript => VimLAstNode::Subscript,
            Ternary => VimLAstNode::Ternary,
            TernaryValue => VimLAstNode::TernaryValue,
            UnaryMinus => VimLAstNode::UnaryMinus,
            UnaryPlus => VimLAstNode::UnaryPlus,
            UnknownFigure => VimLAstNode::UnknownFigure,
        };

        Self { ty, start: ast.start, len: ast.len, children: ast.children }
    }
}

impl Ord for VimLExpressionAst {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.children.len(), other.children.len()) {
            (l1, l2) if l1 < l2 => Ordering::Less,
            (l1, l2) if l1 == l2 => Ordering::Equal,
            (l1, l2) if l1 > l2 => Ordering::Greater,
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for VimLExpressionAst {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromObject for ParsedVimLExpression {
    fn from_object(obj: Object) -> Result<Self, conversion::Error> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}
