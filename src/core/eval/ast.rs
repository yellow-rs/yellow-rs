#[derive(Debug, PartialEq)]
pub(crate) enum Operator {
    Add,
    Sub,
    Mul,

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

    LAnd, // Logical or
    LOr,  // Logical and
    LNot, // Logical not

    BitShiftR, // Right bit shift
    BitShiftL, // Left bit shift
}

#[derive(Debug, PartialEq)]
pub(crate) enum Expression {
    BinOp(BinOp),
    InfixOp(InfixOp),
}

#[derive(Debug, PartialEq)]
pub(crate) struct BinOp {
    op: Operator,
    left: Box<Expression>,
    right: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct InfixOp {
    op: Operator,
    value: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum TokenType {
    Integer,
    Operator(Operator),
}

#[derive(Debug, PartialEq)]
pub(crate) struct Token<'a> {
    pub(crate) tok_type: TokenType,
    pub(crate) value: &'a str,
    pub(crate) pos_start: usize,
    pub(crate) pos_end: usize,
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
            pos_start,
            pos_end,
        }
    }
}
