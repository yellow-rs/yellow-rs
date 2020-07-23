use crate::core::eval::error::*;
use std::fmt;

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
pub(crate) enum Operator {
    Add,
    Sub,
    Mul,
    Mod,

    Div,
    IntDiv,

    Pow,

    Eql,  // Equal
    NEql, // Not equal

    GT, // Greater than
    LT, // Less than
    GE, // Greater than or equal to
    LE, // Less than or equal to

    BAnd, // Bitwise and
    BOr,  // Bitwise or
    BNot, // Bitwise not
    BXor, // Bitwise XOR

    LAnd, // Logical and
    LOr,  // Logical or
    LNot, // Logical not

    BitShiftR, // Right bit shift
    BitShiftL, // Left bit shift

    As, // Casting
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "operator {}",
            match self {
                Operator::Add => "+",
                Operator::Sub => "-",
                Operator::Mul => "*",
                Operator::Mod => "%",
                Operator::Div => "/",
                Operator::IntDiv => "//",
                Operator::Pow => "**",

                Operator::Eql => "==",
                Operator::NEql => "!=",

                Operator::GT => ">",
                Operator::LT => "<",
                Operator::GE => ">=",
                Operator::LE => "<=",

                Operator::BAnd => "&",
                Operator::BOr => "|",
                Operator::BNot => "~",
                Operator::BXor => "^",

                Operator::LAnd => "&&",
                Operator::LOr => "||",
                Operator::LNot => "!",

                Operator::As => "as",

                Operator::BitShiftL => "<<",
                Operator::BitShiftR => ">>",
            }
        )
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum ExpressionKind<'a> {
    PrefixOp(PrefixOp<'a>),
    InfixOp(InfixOp<'a>),
    Integer(&'a str),
    Float(&'a str),
    Ident(&'a str),
}

impl fmt::Display for ExpressionKind<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ExpressionKind::PrefixOp(prefix) => prefix.op.to_string(),
                ExpressionKind::InfixOp(infix) => infix.op.to_string(),
                ExpressionKind::Integer(_) => "integer".to_string(),
                ExpressionKind::Float(_) => "float".to_string(),
                ExpressionKind::Ident(_) => "identifier".to_string()
            }
        )
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Expression<'a> {
    pub(crate) expr: ExpressionKind<'a>,
    pub(crate) pos: Pos,
}

#[derive(Debug, PartialEq)]
pub(crate) struct InfixOp<'a> {
    pub(crate) op: Operator,
    pub(crate) left: Box<Expression<'a>>,
    pub(crate) right: Box<Expression<'a>>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct PrefixOp<'a> {
    pub(crate) op: Operator,
    pub(crate) value: Box<Expression<'a>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum TokenType {
    Identifier,
    Integer,
    Float,
    RP, // )
    LP, // (
    EOF,
    Operator(Operator),
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                // XXX: Get rid of these sad allocations
                TokenType::Integer => "integer".to_string(),
                TokenType::Float => "float".to_string(),
                TokenType::Identifier => "identifier".to_string(),
                TokenType::RP => "`)`".to_string(),
                TokenType::LP => "`(`".to_string(),
                TokenType::EOF => "end of file".to_string(),

                TokenType::Operator(op) => op.to_string()
            }
        )
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) struct Token<'a> {
    pub(crate) tok_type: TokenType,
    pub(crate) value: &'a str,
    pub(crate) pos: Pos,
}

impl<'a> Token<'a> {
    pub(crate) fn unwrap_op(&self) -> Operator {
        match self.tok_type {
            TokenType::Operator(op) => op,
            _ => panic!("Tried to unwrap non operator")
        }
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tok_type)
    }
}

impl<'a> Token<'a> {
    pub(crate) fn new(
        tok_type: TokenType,
        value: &'a str,
        pos_start: usize,
        pos_end: usize,
    ) -> Self {
        Token {
            tok_type,
            value,
            pos: Pos::new(pos_start, pos_end),
        }
    }
}
