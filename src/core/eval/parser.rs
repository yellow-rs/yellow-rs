use crate::core::eval::ast;
use crate::core::eval::error::*;
use std::collections::HashMap;

pub(crate) struct Parser<'a> {
    tokens: Vec<ast::Token<'a>>,
    pos: usize,
    infix_op: HashMap<ast::Operator, u16>,
    prefix_op: HashMap<ast::Operator, u16>,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(tokens: Vec<ast::Token<'a>>) -> Self {
        Parser {
            tokens,
            pos: 0,
            prefix_op: HashMap::new(),
            infix_op: HashMap::new(),
        }
    }

    fn register_infix(&mut self, operator: ast::Operator, bp: u16) {
        self.infix_op.insert(operator, bp);
    }

    fn register_prefix(&mut self, operator: ast::Operator, bp: u16) {
        self.prefix_op.insert(operator, bp);
    }

    /// Configure precedence tables
    pub(crate) fn config(&mut self) {
        self.register_infix(ast::Operator::LOr, 10);
        self.register_infix(ast::Operator::LAnd, 10);

        self.register_infix(ast::Operator::Eql, 20);
        self.register_infix(ast::Operator::NEql, 20);

        self.register_infix(ast::Operator::GT, 21);
        self.register_infix(ast::Operator::LT, 21);
        self.register_infix(ast::Operator::LE, 21);
        self.register_infix(ast::Operator::GE, 21);

        self.register_infix(ast::Operator::BOr, 30);
        self.register_infix(ast::Operator::BXor, 31);
        self.register_infix(ast::Operator::BAnd, 32);

        self.register_infix(ast::Operator::BitShiftR, 40);
        self.register_infix(ast::Operator::BitShiftL, 40);

        self.register_infix(ast::Operator::Add, 50);
        self.register_infix(ast::Operator::Sub, 50);

        self.register_infix(ast::Operator::Mul, 60);
        self.register_infix(ast::Operator::Div, 60);
        self.register_infix(ast::Operator::IntDiv, 60);
        self.register_infix(ast::Operator::Mod, 60);

        self.register_infix(ast::Operator::As, 70);

        self.register_prefix(ast::Operator::LNot, 70);
        self.register_prefix(ast::Operator::BNot, 70);
        self.register_prefix(ast::Operator::Sub, 70);
        self.register_prefix(ast::Operator::Add, 70);
    }

    fn peek(&self) -> ast::Token<'a> {
        self.tokens[self.pos]
    }

    fn advance(&mut self) -> ast::Token<'a> {
        match self.tokens.get(self.pos) {
            Some(tok) => {
                self.pos += 1;
                *tok
            }
            None => {
                dbg!(self.tokens[self.tokens.len()-1])
            }
        }
    }

    fn bp_infix(&self, op: ast::Token<'a>) -> u16 {
        match op.tok_type {
            ast::TokenType::Operator(oper) => {
                self.infix_op[&oper]
            }
            _ => unreachable!()
        }
    }

    fn bp_prefix(&self, op: &ast::Token<'a>) -> u16 {
        self.prefix_op[&op.unwrap_op()]
    }


    fn item(&mut self) -> Result<ast::Expression<'a>, Error> {
        if let Ok(prefix) = self.get_operator_prefix() {
            let binding_power = self.bp_prefix(&prefix);
            let item = self.expr(binding_power)?;
            return Ok(ast::Expression {
                pos: Pos::new(prefix.pos.start, item.pos.end),
                expr: ast::ExpressionKind::PrefixOp(ast::PrefixOp {
                    op: prefix.unwrap_op(),
                    value: Box::new(item),
                }),
            });
        }

        let next = self.advance();
        match next.tok_type {
            ast::TokenType::Integer => Ok(ast::Expression {
                expr: ast::ExpressionKind::Integer(next.value),
                pos: next.pos,
            }),
            _ => Err(Error::new(
                format!("unexpected {}", next),
                ErrorType::SyntaxError,
                next.pos,
            )),
        }
    }

    fn get_operator_prefix(&mut self) -> Result<ast::Token<'a>, Error> {
        let pos = self.pos;
        let potential_op = self.advance();
        if let ast::TokenType::Operator(p_op) = potential_op.tok_type {
            match self.prefix_op.get(&p_op) {
                // its a valid operator
                Some(_) => return Ok(potential_op),
                None => {}
            }
        }

        self.pos = pos;
        Err(Error::new(
            format!("cannot use {} as an prefix operator", potential_op),
            ErrorType::SyntaxError,
            potential_op.pos,
        ))
    }

    fn get_operator_infix(&mut self) -> Result<ast::Operator, Error> {
        let pos = self.pos;
        let potential_op = self.peek();
        if let ast::TokenType::Operator(p_op) = potential_op.tok_type {
            match self.infix_op.get(&p_op) {
                // its a valid operator
                Some(_) => return Ok(p_op),
                None => {}
            }
        }

        self.pos = pos;
        Err(Error::new(
            format!("cannot use {} as an infix operator", potential_op),
            ErrorType::SyntaxError,
            potential_op.pos,
        ))
    }

    fn led(
        &mut self,
        left: ast::Expression<'a>,
        operator: ast::Operator,
    ) -> Result<ast::Expression<'a>, Error> {
        let binding_power = self.infix_op[&operator];

        let right = self.expr(binding_power)?;

        Ok(ast::Expression {
            pos: Pos::new(left.pos.start, right.pos.end),
            expr: ast::ExpressionKind::InfixOp(ast::InfixOp {
                op: operator,
                left: Box::new(left),
                right: Box::new(right),
            }),
        })
    }

    pub(crate) fn expr(&mut self, prec: u16) -> Result<ast::Expression<'a>, Error> {
        let mut left = self.item()?;

        let mut operator;

        loop {
            if self.peek().tok_type == ast::TokenType::EOF {
                break;
            }

            match self.get_operator_infix() {
                Ok(oper) => {
                    operator = oper;
                }
                Err(why) => return Err(why),
            }

            let binding_power = self.infix_op[&operator];

            if !(binding_power > prec) {
                break;
            }   
            
            self.advance(); // Advance operator

            match self.led(left, operator) {
                Ok(val) => {
                    left = val;
                }
                Err(why) => {
                    return Err(why);
                }
            }
        }

        Ok(left)
    }
}
