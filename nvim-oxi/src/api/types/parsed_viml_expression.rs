use std::cmp::Ordering;
use std::collections::BTreeSet;

use nvim_types::{Float, Integer, Object};
use serde::Deserialize;

use super::viml_ast_node::*;
use crate::object::{self, FromObject};

#[non_exhaustive]
#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize)]
pub struct ParsedVimLExpression {
    #[serde(default)]
    pub ast: Option<VimLExpressionAst>,

    #[serde(default)]
    pub error: Option<ParseExpressionError>,

    #[serde(default)]
    pub highlight: Vec<(usize, usize, usize, String)>,

    pub len: usize,
}

#[non_exhaustive]
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Deserialize)]
pub struct ParseExpressionError {
    /// Error message in printf format. Contains exactly one `"%.*s"` block.
    pub message: String,

    /// Error message argument.
    pub arg: String,
}

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Deserialize)]
#[serde(from = "DeserializedVimLExpressionAST")]
pub struct VimLExpressionAst {
    /// Error message in printf format. Contains exactly one `"%.*s"` block.
    pub ty: VimLAstNode,

    /// A `(line, column)` tuple describing where the the node is started.
    pub start: (usize, usize),

    /// Length of the node.
    pub len: usize,

    /// A tree of child nodes.
    #[serde(default)]
    pub children: BTreeSet<VimLExpressionAst>,
}

/// Only used for deserialization purposes.
#[derive(Deserialize)]
#[allow(dead_code)]
struct DeserializedVimLExpressionAST {
    #[serde(rename = "type")]
    ty: DeserializedVimLASTNode,

    start: (usize, usize),
    len: usize,

    #[serde(default)]
    children: BTreeSet<VimLExpressionAst>,

    #[serde(default)]
    augmentation: Option<AssignmentAugmentation>,

    #[serde(default)]
    cmp_type: Option<ExprComparisonType>,

    #[serde(default)]
    ccs_strategy: Option<ExprCaseCompareStrategy>,

    #[serde(default)]
    invert: Option<bool>,

    #[serde(default)]
    svalue: Option<String>,

    #[serde(default)]
    fvalue: Option<Float>,

    #[serde(default)]
    ivalue: Option<Integer>,

    #[serde(default)]
    scope: Option<ExprScope>,

    #[serde(default)]
    ident: Option<String>,

    #[serde(default)]
    name: Option<i32>,
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

impl FromObject for ParsedVimLExpression {
    fn from_obj(obj: Object) -> crate::Result<Self> {
        Self::deserialize(object::Deserializer::new(obj))
    }
}
