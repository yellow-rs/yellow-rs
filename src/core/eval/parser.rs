use crate::core::eval::ast;
use std::collections::HashMap;

pub(crate) struct Parser<'a> {
    tokens: Vec<ast::Token<'a>>,
    pos: usize,
    prec_table: HashMap<ast::Operator, u16>,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(tokens: Vec<ast::Token<'a>>) -> Self {
        Parser {
            tokens,
            pos: 0,
            prec_table: HashMap::new(),
        }
    }

    /// Configure precedence tables
    pub(crate) fn config(&mut self) {
        self.prec_table.insert(ast::Operator::LOr, 10);
        self.prec_table.insert(ast::Operator::LAnd, 12);

        self.prec_table.insert(ast::Operator::Eql, 20);
        self.prec_table.insert(ast::Operator::NEql, 20);

        self.prec_table.insert(ast::Operator::GT, 21);
        self.prec_table.insert(ast::Operator::LT, 21);
        self.prec_table.insert(ast::Operator::LE, 21);
        self.prec_table.insert(ast::Operator::GE, 21);
        
        self.prec_table.insert(ast::Operator::BOr, 30);
        self.prec_table.insert(ast::Operator::BXor, 31);
        self.prec_table.insert(ast::Operator::BAnd, 32);

        self.prec_table.insert(ast::Operator::BitShiftR, 40);
        self.prec_table.insert(ast::Operator::BitShiftL, 40);

        self.prec_table.insert(ast::Operator::Add, 50);
        self.prec_table.insert(ast::Operator::Sub, 50);

        self.prec_table.insert(ast::Operator::Mul, 60);
        self.prec_table.insert(ast::Operator::Div, 60);
        self.prec_table.insert(ast::Operator::Mod, 60);
    }

    //pub(crate) fn parse(&mut self) -> ast::Expression {}
}
