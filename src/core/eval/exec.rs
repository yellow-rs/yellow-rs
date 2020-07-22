use std::collections::HashMap;
use std::fmt;

use crate::core::eval::error::*;
use crate::core::eval::ast;

enum ExecutionExpr {
    Integer(i32),
    Float(f32),
    Bool(bool),
}

impl fmt::Display for ExecutionExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ExecutionExpr::Integer(val) => val.to_string(),
                ExecutionExpr::Float(val) => val.to_string(),
                ExecutionExpr::Bool(val) => val.to_string()
            }
        )
    }
}

struct EE {
    value: ExecutionExpr,
    pos: Pos
}

pub struct Executer<'a> {
    symbtab: HashMap<&'a str, EE>
}

impl<'a> Executer<'a> {
    pub(crate) fn new() -> Self {
        Executer {
            symbtab: HashMap::new()
        }
    }
    
    pub(crate) fn eval(&mut self, ast: ast::Expression<'a>) -> Result<String, Error> {
        Ok("".to_string())
    }
}

