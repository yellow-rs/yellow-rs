use std::collections::HashMap;
use std::fmt;

use crate::core::eval::error::*;
use crate::core::eval::{ast, ast::ExpressionKind};

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

use ExecutionExpr::*;
enum ExecutionExpr {
    Integer(i32),
    Float(f32),
    Bool(bool),
}

impl ExecutionExpr {
    fn display_type(&self) -> &'static str {
        match self {
            ExecutionExpr::Integer(_) => "integer",
            ExecutionExpr::Float(_) => "float",
            ExecutionExpr::Bool(_) => "boolean",
        }
    }
}

impl fmt::Display for ExecutionExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ExecutionExpr::Integer(val) => val.to_string(),
                ExecutionExpr::Float(val) => val.to_string(),
                ExecutionExpr::Bool(val) => val.to_string(),
            }
        )
    }
}

pub struct EE {
    value: ExecutionExpr,
    pos: Pos,
}

impl EE {
    fn new(expr: ExecutionExpr, pos: Pos) -> Self {
        EE { value: expr, pos }
    }

    fn calc_pos(&self, other: &Self) -> Pos {
        Pos::new(self.pos.start, other.pos.end)
    }

    fn gen_type_err(&self, other: &Self, operation: &'static str) -> Error {
        Error::new(
            format!(
                "cannot {} type {} by {}",
                operation,
                self.value.display_type(),
                other.value.display_type()
            ),
            ErrorType::TypeError,
            self.calc_pos(other),
        )
    }

    fn add(&self, other: &Self) -> Result<Self, Error> {
        let value = match (&self.value, &other.value) {
            (Integer(left), Integer(right)) => Integer(match left.checked_add(*right) {
                Some(val) => val,
                None => return Err(Error::new("value overflowed".to_string(), ErrorType::RuntimeError, self.calc_pos(other)))
            }),
            (Float(left), Float(right)) => Float(left.add(*right)),
            _ => return Err(self.gen_type_err(other, "add")),
        };

        let pos = self.calc_pos(other);

        Ok(EE { value, pos })
    }

    fn sub(&self, other: &Self) -> Result<Self, Error> {
        let value = match (&self.value, &other.value) {
            (Integer(left), Integer(right)) => Integer(match left.checked_sub(*right) {
                Some(val) => val,
                None => return Err(Error::new("value overflowed".to_string(), ErrorType::RuntimeError, self.calc_pos(other)))
            }),
            (Float(left), Float(right)) => Float(left.sub(*right)),
            _ => return Err(self.gen_type_err(other, "subtract")),
        };

        let pos = self.calc_pos(other);

        Ok(EE { value, pos })
    }

    fn mul(&self, other: &Self) -> Result<Self, Error> {
        let value = match (&self.value, &other.value) {
            (Integer(left), Integer(right)) => Integer(match left.checked_mul(*right) {
                Some(val) => val,
                None => return Err(Error::new("value overflowed".to_string(), ErrorType::RuntimeError, self.calc_pos(other)))
            }),
            (Float(left), Float(right)) => Float(left.mul(*right)),
            _ => return Err(self.gen_type_err(other, "multiply")),
        };

        let pos = self.calc_pos(other);

        Ok(EE { value, pos })
    }

    fn div(&self, other: &Self) -> Result<Self, Error> {
        let value = match (&self.value, &other.value) {
            (Integer(left), Integer(right)) => Float((*left as f32).div(*right as f32)),
            (Float(left), Float(right)) => Float(left.div(*right)),
            _ => return Err(self.gen_type_err(other, "divide")),
        };

        let pos = self.calc_pos(other);

        Ok(EE { value, pos })
    }

    fn int_div(&self, other: &Self) -> Result<Self, Error> {
        let value = match (&self.value, &other.value) {
            (Integer(left), Integer(right)) => Integer(match left.checked_div(*right) {
                Some(val) => val,
                None => return Err(Error::new("value overflowed".to_string(), ErrorType::RuntimeError, self.calc_pos(other)))
            }),
            (Float(left), Float(right)) => Integer(match (*left as i32).checked_div(*right as i32) {
                Some(val) => val,
                None => return Err(Error::new("value overflowed".to_string(), ErrorType::RuntimeError, self.calc_pos(other)))
            }),
            _ => return Err(self.gen_type_err(other, "divide")),
        };

        let pos = self.calc_pos(other);

        Ok(EE { value, pos })
    }

    fn neg(&self) -> Result<Self, Error> {
        let value = match &self.value {
            Integer(val) => Integer(-val),
            Float(val) => Float(-val),
            _ => return Err(Error::new(format!("cannot make type {} negative", self.value.display_type()), ErrorType::TypeError, self.pos)),
        };

        Ok(EE { value, pos: self.pos })
    }

}

impl fmt::Display for EE {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub struct Executer<'a> {
    symbtab: HashMap<&'a str, EE>,
}

impl<'a> Executer<'a> {
    pub(crate) fn new() -> Self {
        Executer {
            symbtab: HashMap::new(),
        }
    }

    pub(crate) fn eval(&mut self, ast: ast::Expression<'a>) -> Result<EE, Error> {
        Ok(match ast.expr {
            ExpressionKind::Integer(val) => EE::new(ExecutionExpr::Integer(val.parse::<i32>().unwrap()), ast.pos),
            ExpressionKind::InfixOp(val) => match val.op {
                ast::Operator::Add => self.eval(*val.left)?.add(&self.eval(*val.right)?)?,
                ast::Operator::Sub => self.eval(*val.left)?.sub(&self.eval(*val.right)?)?,
                ast::Operator::Mul => self.eval(*val.left)?.mul(&self.eval(*val.right)?)?,
                ast::Operator::Div => self.eval(*val.left)?.div(&self.eval(*val.right)?)?,
                ast::Operator::IntDiv => self.eval(*val.left)?.int_div(&self.eval(*val.right)?)?,
                _ => {
                    return Err(Error::new(
                        format!("infix {} not implemented yet", val.op),
                        ErrorType::TypeError,
                        ast.pos,
                    ))
                }
            }

            ExpressionKind::PrefixOp(val) => match val.op {
                ast::Operator::Sub => self.eval(*val.value)?.neg()?,
                _ => {
                    return Err(Error::new(
                        format!("prefix {} not implemented yet", val.op),
                        ErrorType::TypeError,
                        ast.pos,
                    ))
                }
            }
        })
    }
}
